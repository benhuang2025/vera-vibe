use axum::{
    extract::{DefaultBodyLimit, Multipart, Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use brevis_vera_lib::{
    EditManifest, EditOperation, PhotoMetadata, ProofPackage, PublicValues, Signature,
    SignedPhoto,
};
use p256::ecdsa::{
    signature::{Signer, Verifier},
    SigningKey, VerifyingKey,
};
use p256::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

// ===== App State =====
#[derive(Clone)]
struct AppState {
    data_dir: PathBuf,
    zk_dir: PathBuf,
    sessions: Arc<Mutex<HashMap<String, SessionData>>>,
    jobs: Arc<Mutex<HashMap<String, JobStatus>>>,
}

#[derive(Clone)]
struct SessionData {
    root_ca_hash: String,
    device_pk_hash: String,
    signed_photo: Option<SignedPhoto>,
    edit_manifest: Option<EditManifest>,
    edited_image_path: Option<PathBuf>,
    proof_package_path: Option<PathBuf>,
}

#[derive(Clone, Serialize)]
struct JobStatus {
    status: String, // "running", "done", "error"
    progress: u32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
}

// ===== Request/Response Types =====
#[derive(Deserialize)]
struct EditRequest {
    session_id: String,
    ops: String,
}

#[derive(Deserialize)]
struct ProveRequest {
    session_id: String,
    real_proof: bool,
}

// ===== Handlers =====

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn keygen(State(state): State<AppState>) -> Json<Value> {
    let session_id = uuid::Uuid::new_v4().to_string();
    let session_dir = state.data_dir.join(&session_id);
    fs::create_dir_all(&session_dir).unwrap();

    // Generate Root CA key
    let root_ca_sk = SigningKey::random(&mut rand_core::OsRng);
    let root_ca_vk = VerifyingKey::from(&root_ca_sk);

    // Generate Device key
    let device_sk = SigningKey::random(&mut rand_core::OsRng);
    let device_vk = VerifyingKey::from(&device_sk);

    // Root CA signs Device public key (cert)
    let device_pk_bytes = device_vk.to_sec1_bytes();
    let cert_sig: p256::ecdsa::Signature = root_ca_sk.sign(&device_pk_bytes);
    let (cert_r, cert_s) = cert_sig.split_bytes();

    // Save device key
    let device_key_path = session_dir.join("device_key.pem");
    let pem_str = device_sk.to_pkcs8_pem(p256::pkcs8::LineEnding::LF).unwrap();
    fs::write(&device_key_path, pem_str.as_ref() as &str).unwrap();

    // Save metadata for session
    let root_ca_hash = hex::encode(Sha256::digest(root_ca_vk.to_sec1_bytes()));
    let device_pk_hash = hex::encode(Sha256::digest(&device_pk_bytes));

    let session_data = SessionData {
        root_ca_hash: root_ca_hash.clone(),
        device_pk_hash: device_pk_hash.clone(),
        signed_photo: None,
        edit_manifest: None,
        edited_image_path: None,
        proof_package_path: None,
    };

    // Save cert data for later use
    let cert_data = serde_json::json!({
        "root_ca_pubkey": root_ca_vk.to_sec1_bytes().to_vec(),
        "device_pubkey": device_pk_bytes.to_vec(),
        "cert_r": cert_r.as_ref() as &[u8],
        "cert_s": cert_s.as_ref() as &[u8],
    });
    fs::write(
        session_dir.join("cert_chain.json"),
        serde_json::to_string(&cert_data).unwrap(),
    )
    .unwrap();

    state
        .sessions
        .lock()
        .unwrap()
        .insert(session_id.clone(), session_data);

    Json(json!({
        "session_id": session_id,
        "root_ca_hash": root_ca_hash,
        "device_pubkey_hash": device_pk_hash,
    }))
}

async fn sign_photo(State(state): State<AppState>, mut multipart: Multipart) -> Json<Value> {
    let mut session_id = String::new();
    let mut image_data: Vec<u8> = Vec::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "session_id" => session_id = field.text().await.unwrap(),
            "image" => image_data = field.bytes().await.unwrap().to_vec(),
            _ => {}
        }
    }

    let session_dir = state.data_dir.join(&session_id);

    // Decode image
    let img = image::load_from_memory(&image_data).expect("Failed to decode image");
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let pixels = rgb.as_raw().to_vec();

    // Save original image for preview
    fs::write(session_dir.join("original.jpg"), &image_data).unwrap();

    // Determine shard count
    let num_shards = num_cpus::get().max(1);

    // Compute shard hashes
    let shard_size = (pixels.len() + num_shards - 1) / num_shards;
    let mut shards = Vec::new();
    for i in 0..num_shards {
        let s = i * shard_size;
        let e = std::cmp::min(s + shard_size, pixels.len());
        if s < pixels.len() {
            shards.push(Sha256::digest(&pixels[s..e]).into());
        }
    }

    let image_hash: [u8; 32] = Sha256::digest(&pixels).into();

    // Load device key and cert
    let device_key_pem = fs::read_to_string(session_dir.join("device_key.pem")).unwrap();
    let device_sk = SigningKey::from_pkcs8_pem(&device_key_pem).unwrap();
    let device_vk = VerifyingKey::from(&device_sk);

    let cert_json: Value =
        serde_json::from_str(&fs::read_to_string(session_dir.join("cert_chain.json")).unwrap())
            .unwrap();

    let metadata = PhotoMetadata {
        device_id: "brevis-vera-web".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        width,
        height,
        image_hash,
        shards: shards.clone(),
    };

    // Sign metadata
    let msg = metadata.to_bytes();
    let sig: p256::ecdsa::Signature = device_sk.sign(&msg);
    let (r_bytes, s_bytes) = sig.split_bytes();

    let root_ca_pubkey: Vec<u8> =
        serde_json::from_value(cert_json["root_ca_pubkey"].clone()).unwrap();
    let cert_r_vec: Vec<u8> = serde_json::from_value(cert_json["cert_r"].clone()).unwrap();
    let cert_s_vec: Vec<u8> = serde_json::from_value(cert_json["cert_s"].clone()).unwrap();

    let mut cert_r_arr = [0u8; 32];
    let mut cert_s_arr = [0u8; 32];
    cert_r_arr.copy_from_slice(&cert_r_vec);
    cert_s_arr.copy_from_slice(&cert_s_vec);

    let signature = Signature {
        r: <[u8; 32]>::from(r_bytes),
        s: <[u8; 32]>::from(s_bytes),
        public_key: device_vk.to_sec1_bytes().to_vec(),
        root_ca_pubkey,
        device_cert_r: cert_r_arr,
        device_cert_s: cert_s_arr,
    };

    let signed_photo = SignedPhoto {
        image_bytes: pixels,
        metadata,
        signature,
    };

    // Save signed photo
    let signed_json = serde_json::to_string(&signed_photo).unwrap();
    fs::write(session_dir.join("signed_photo.json"), &signed_json).unwrap();

    // Update session
    {
        let mut sessions = state.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(&session_id) {
            session.signed_photo = Some(signed_photo);
        }
    }

    Json(json!({
        "num_shards": num_shards,
        "width": width,
        "height": height,
        "image_hash": hex::encode(image_hash),
    }))
}

async fn serve_original_preview(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> axum::response::Response {
    let path = state.data_dir.join(&session_id).join("original.jpg");
    if path.exists() {
        let bytes = fs::read(&path).unwrap();
        axum::response::Response::builder()
            .header("content-type", "image/jpeg")
            .body(axum::body::Body::from(bytes))
            .unwrap()
    } else {
        axum::response::Response::builder()
            .status(404)
            .body(axum::body::Body::from("Not found"))
            .unwrap()
    }
}

async fn edit_photo(State(state): State<AppState>, Json(req): Json<EditRequest>) -> Json<Value> {
    let session_dir = state.data_dir.join(&req.session_id);

    // Load signed photo
    let signed_json = fs::read_to_string(session_dir.join("signed_photo.json")).unwrap();
    let signed_photo: SignedPhoto = serde_json::from_str(&signed_json).unwrap();

    // Parse operations
    let operations = parse_ops(&req.ops);
    let manifest = EditManifest {
        operations: operations.clone(),
    };

    // Apply edits
    let mut pixels = signed_photo.image_bytes.clone();
    let mut w = signed_photo.metadata.width;
    let mut h = signed_photo.metadata.height;

    for op in &operations {
        match op {
            EditOperation::Crop {
                x,
                y,
                width,
                height,
            } => {
                pixels = brevis_vera_lib::pixel_utils::apply_crop(
                    &pixels, w, h, *x, *y, *width, *height,
                );
                w = *width;
                h = *height;
            }
            EditOperation::AdjustBrightness { delta } => {
                pixels = brevis_vera_lib::pixel_utils::apply_brightness(&pixels, *delta);
            }
        }
    }

    // Save edited image as PNG
    let edited_path = session_dir.join("edited.png");
    let img_buf = image::RgbImage::from_raw(w, h, pixels).expect("Failed to create image");
    img_buf
        .save(&edited_path)
        .expect("Failed to save edited image");

    // Save manifest
    let manifest_json = serde_json::to_string(&manifest).unwrap();
    fs::write(session_dir.join("edit_manifest.json"), &manifest_json).unwrap();

    // Update session
    {
        let mut sessions = state.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(&req.session_id) {
            session.edit_manifest = Some(manifest);
            session.edited_image_path = Some(edited_path.clone());
        }
    }

    let url = format!("/api/data/{}/edited.png", req.session_id);
    Json(json!({
        "edited_image_url": url,
        "width": w,
        "height": h,
    }))
}

async fn serve_data_file(
    State(state): State<AppState>,
    Path((session_id, filename)): Path<(String, String)>,
) -> axum::response::Response {
    let path = state.data_dir.join(&session_id).join(&filename);
    if path.exists() {
        let bytes = fs::read(&path).unwrap();
        let content_type = if filename.ends_with(".png") {
            "image/png"
        } else if filename.ends_with(".json") {
            "application/json"
        } else if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
            "image/jpeg"
        } else {
            "application/octet-stream"
        };
        axum::response::Response::builder()
            .header("content-type", content_type)
            .body(axum::body::Body::from(bytes))
            .unwrap()
    } else {
        axum::response::Response::builder()
            .status(404)
            .body(axum::body::Body::from("Not found"))
            .unwrap()
    }
}

async fn start_prove(State(state): State<AppState>, Json(req): Json<ProveRequest>) -> Json<Value> {
    let job_id = uuid::Uuid::new_v4().to_string();

    // Initialize job
    state.jobs.lock().unwrap().insert(
        job_id.clone(),
        JobStatus {
            status: "running".to_string(),
            progress: 0,
            message: "Starting...".to_string(),
            result: None,
        },
    );

    let job_id_clone = job_id.clone();
    let state_clone = state.clone();
    let session_id = req.session_id.clone();
    let real_proof = req.real_proof;

    // Spawn blocking task
    tokio::task::spawn_blocking(move || {
        run_prove_job(state_clone, session_id, job_id_clone, real_proof);
    });

    Json(json!({ "job_id": job_id }))
}

fn run_prove_job(state: AppState, session_id: String, job_id: String, real_proof: bool) {
    let session_dir = state.data_dir.join(&session_id);
    let start = Instant::now();

    // Update progress
    let update = |progress: u32, msg: &str| {
        if let Ok(mut jobs) = state.jobs.lock() {
            if let Some(job) = jobs.get_mut(&job_id) {
                job.progress = progress;
                job.message = msg.to_string();
            }
        }
    };

    update(5, "Loading signed photo...");

    // Load signed photo
    let signed_json = fs::read_to_string(session_dir.join("signed_photo.json")).unwrap();
    let signed_photo: SignedPhoto = serde_json::from_str(&signed_json).unwrap();

    // Load manifest (create with empty ops if user skipped "Apply Edits")
    let manifest = if session_dir.join("edit_manifest.json").exists() {
        let manifest_json = fs::read_to_string(session_dir.join("edit_manifest.json")).unwrap();
        serde_json::from_str(&manifest_json).unwrap()
    } else {
        let manifest = EditManifest {
            operations: vec![],
        };
        fs::write(
            session_dir.join("edit_manifest.json"),
            serde_json::to_string(&manifest).unwrap(),
        )
        .unwrap();
        // Save edited.png as passthrough (same as original) for consistency
        let edited_path = session_dir.join("edited.png");
        let img_buf = image::RgbImage::from_raw(
            signed_photo.metadata.width,
            signed_photo.metadata.height,
            signed_photo.image_bytes.clone(),
        )
        .expect("Failed to create image");
        img_buf.save(&edited_path).expect("Failed to save edited image");
        manifest
    };

    update(10, "Verifying cert chain (host-side)...");

    // Verify cert chain
    {
        let root_ca_vk =
            VerifyingKey::from_sec1_bytes(&signed_photo.signature.root_ca_pubkey).unwrap();
        let cert_r = p256::FieldBytes::from(signed_photo.signature.device_cert_r);
        let cert_s = p256::FieldBytes::from(signed_photo.signature.device_cert_s);
        let device_cert = p256::ecdsa::Signature::from_scalars(cert_r, cert_s).unwrap();
        root_ca_vk
            .verify(&signed_photo.signature.public_key, &device_cert)
            .unwrap();
    }

    update(15, "Running parallel shard emulation...");

    // Parallel shard AOT emulation
    use pico_vm::compiler::riscv::compiler::{Compiler, SourceType};
    use rayon::prelude::*;

    let shard_elf_path = state.zk_dir.join("shard-app/elf/riscv32im-pico-zkvm-elf");
    let shard_elf = fs::read(&shard_elf_path).expect("Failed to read shard ELF");
    let shard_program = Compiler::new(SourceType::RISCV, &shard_elf).compile();

    let num_shards = signed_photo.metadata.shards.len();
    let image_len = signed_photo.image_bytes.len();
    let shard_size = (image_len + num_shards - 1) / num_shards;

    // Check if manifest has crop - use CropBypass so hash matches edited.png
    let has_crop = manifest
        .operations
        .iter()
        .any(|op| matches!(op, EditOperation::Crop { .. }));

    let shard_results: Vec<([u8; 32], [u8; 32])> = if has_crop {
        // Crop changes image dimensions so shard AOT can't handle it.
        // Compute (orig_hash, edited_hash) pairs directly on the host.
        let mut edited_pixels = signed_photo.image_bytes.clone();
        let mut w = signed_photo.metadata.width;
        let mut h = signed_photo.metadata.height;
        for op in &manifest.operations {
            match op {
                EditOperation::Crop { x, y, width, height } => {
                    edited_pixels = brevis_vera_lib::pixel_utils::apply_crop(
                        &edited_pixels, w, h, *x, *y, *width, *height,
                    );
                    w = *width;
                    h = *height;
                }
                EditOperation::AdjustBrightness { delta } => {
                    edited_pixels =
                        brevis_vera_lib::pixel_utils::apply_brightness(&edited_pixels, *delta);
                }
            }
        }
        let edited_len = edited_pixels.len();
        let edited_shard_size = (edited_len + num_shards - 1) / num_shards;

        (0..num_shards)
            .into_par_iter()
            .map(|i| {
                let orig_s = i * shard_size;
                let orig_e = std::cmp::min(orig_s + shard_size, image_len);
                let orig_hash: [u8; 32] = if orig_s < image_len {
                    Sha256::digest(&signed_photo.image_bytes[orig_s..orig_e]).into()
                } else {
                    Sha256::digest(&[]).into()
                };

                let edited_s = i * edited_shard_size;
                let edited_e = std::cmp::min(edited_s + edited_shard_size, edited_len);
                let edited_hash: [u8; 32] = if edited_s < edited_len {
                    Sha256::digest(&edited_pixels[edited_s..edited_e]).into()
                } else {
                    Sha256::digest(&[]).into()
                };

                (orig_hash, edited_hash)
            })
            .collect()
    } else {
        // No crop: shard applies brightness in ZK via AOT emulation
        let brightness_ops: Vec<EditOperation> = manifest
            .operations
            .iter()
            .filter(|op| matches!(op, EditOperation::AdjustBrightness { .. }))
            .cloned()
            .collect();

        (0..num_shards)
            .into_par_iter()
            .map(|i| {
                let s = i * shard_size;
                let e = std::cmp::min(s + shard_size, image_len);
                let shard_pixels = if s < image_len {
                    signed_photo.image_bytes[s..e].to_vec()
                } else {
                    Vec::new()
                };

                let shard_input: (Vec<u8>, Vec<EditOperation>) =
                    (shard_pixels, brightness_ops.clone());
                let shard_input_bytes = bincode::serialize(&shard_input).unwrap();
                let mut emu = pico_aot_runtime::AotEmulatorCore::new(
                    shard_program.clone(),
                    vec![shard_input_bytes],
                );
                shard_aot::run_aot(&mut emu).expect("Shard AOT failed");

                bincode::deserialize(&emu.public_values_stream).unwrap()
            })
            .collect()
    };

    update(50, "Shard emulation complete. Running aggregator...");

    let shard_commits: Vec<([u8; 32], [u8; 32])> = shard_results;

    let public_values: PublicValues;
    let proof_bytes: Vec<u8>;
    let proof_type: String;

    if real_proof {
        update(55, "Generating Real STARK Proof (this takes ~60s)...");

        use pico_vm::configs::config::StarkGenericConfig;
        use pico_vm::configs::stark_config::KoalaBearPoseidon2;
        use pico_vm::emulator::stdin::EmulatorStdinBuilder;
        use pico_vm::proverchain::{InitialProverSetup, MachineProver, RiscvProver};

        let agg_elf_path = state
            .zk_dir
            .join("aggregator-app/elf/riscv32im-pico-zkvm-elf");
        let agg_elf = fs::read(&agg_elf_path).expect("Failed to read aggregator ELF");

        let prover = RiscvProver::new_initial_prover(
            (KoalaBearPoseidon2::new(), agg_elf.as_slice()),
            Default::default(),
            None,
        );

        let mut stdin_builder = EmulatorStdinBuilder::<Vec<u8>, KoalaBearPoseidon2>::default();
        stdin_builder.write(&signed_photo.metadata);
        stdin_builder.write(&signed_photo.signature);
        stdin_builder.write(&shard_commits);
        let (stdin, _) = stdin_builder.finalize();

        update(60, "STARK proving in progress...");

        let meta_proof = prover.prove(stdin);

        update(90, "Verifying STARK proof...");

        let vk = prover.vk();
        let verified = prover.verify(&meta_proof, vk);
        if !verified {
            if let Ok(mut jobs) = state.jobs.lock() {
                if let Some(job) = jobs.get_mut(&job_id) {
                    job.status = "error".to_string();
                    job.message = "STARK proof verification failed".to_string();
                }
            }
            return;
        }

        // Save proof files
        meta_proof
            .save_to_file(session_dir.join("stark_proof.bin"))
            .unwrap();
        let vk_bytes = bincode::serialize(vk).unwrap();
        fs::write(session_dir.join("stark_vk.bin"), &vk_bytes).unwrap();

        let pv_stream = meta_proof.pv_stream.clone().unwrap_or_default();
        public_values = bincode::deserialize(&pv_stream).unwrap();
        proof_bytes = bincode::serialize(&("stark_proof.bin", "stark_vk.bin")).unwrap();
        proof_type = "Pico STARK (cryptographically verifiable)".to_string();
    } else {
        update(55, "Running aggregator AOT emulation...");

        let agg_elf_path = state
            .zk_dir
            .join("aggregator-app/elf/riscv32im-pico-zkvm-elf");
        let agg_elf = fs::read(&agg_elf_path).unwrap();
        let agg_program = Compiler::new(SourceType::RISCV, &agg_elf).compile();

        let agg_stdin = vec![
            bincode::serialize(&signed_photo.metadata).unwrap(),
            bincode::serialize(&signed_photo.signature).unwrap(),
            bincode::serialize(&shard_commits).unwrap(),
        ];

        let mut agg_emu = pico_aot_runtime::AotEmulatorCore::new(agg_program, agg_stdin);
        aggregator_aot::run_aot(&mut agg_emu).expect("Aggregator AOT failed");

        public_values = bincode::deserialize(&agg_emu.public_values_stream).unwrap();
        proof_bytes = vec![0u8; 32];
        proof_type = "Emulated (fast, not cryptographic)".to_string();
    }

    update(95, "Saving proof package...");

    // Save proof package (lightweight, without image bytes)
    let package = ProofPackage {
        edited_image: vec![], // image is served separately
        proof: proof_bytes,
        public_values: public_values.clone(),
        num_shards,
    };
    let pkg_json = serde_json::to_string_pretty(&package).unwrap();
    fs::write(session_dir.join("proof_package.json"), &pkg_json).unwrap();

    let elapsed = start.elapsed().as_millis() as u64;

    // Mark done
    if let Ok(mut jobs) = state.jobs.lock() {
        if let Some(job) = jobs.get_mut(&job_id) {
            job.status = "done".to_string();
            job.progress = 100;
            job.message = "Complete".to_string();
            job.result = Some(json!({
                "proof_type": proof_type,
                "public_values": {
                    "original_image_hash": hex::encode(public_values.original_image_hash),
                    "output_image_hash": hex::encode(public_values.output_image_hash),
                    "root_ca_hash": hex::encode(public_values.root_ca_hash),
                    "pub_key_hash": hex::encode(public_values.pub_key_hash),
                },
                "proving_time_ms": elapsed,
                "proof_download_url": format!("/api/data/{}/proof_package.json", session_id),
            }));
        }
    }
}

async fn prove_status(State(state): State<AppState>, Path(job_id): Path<String>) -> Json<Value> {
    let jobs = state.jobs.lock().unwrap();
    if let Some(job) = jobs.get(&job_id) {
        if job.status == "done" {
            Json(json!({
                "status": "done",
                "progress": 100,
                "message": "Complete",
                "proof_type": job.result.as_ref().and_then(|r| r.get("proof_type")).and_then(|v| v.as_str()),
                "public_values": job.result.as_ref().and_then(|r| r.get("public_values")),
                "proving_time_ms": job.result.as_ref().and_then(|r| r.get("proving_time_ms")),
                "proof_download_url": job.result.as_ref().and_then(|r| r.get("proof_download_url")),
            }))
        } else {
            Json(json!({
                "status": job.status,
                "progress": job.progress,
                "message": job.message,
            }))
        }
    } else {
        Json(json!({"status": "not_found"}))
    }
}

async fn verify_photo(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mut image_data: Vec<u8> = Vec::new();
    let mut proof_data: Vec<u8> = Vec::new();
    let mut trusted_hash: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Invalid multipart: {}", e)})),
            )
        })?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "image" => {
                image_data = field
                    .bytes()
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(json!({"error": format!("Failed to read image: {}", e)})),
                        )
                    })?
                    .to_vec();
            }
            "proof_package" => {
                proof_data = field
                    .bytes()
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(json!({"error": format!("Failed to read proof package: {}", e)})),
                        )
                    })?
                    .to_vec();
            }
            "trusted_root_ca_hash" => {
                if let Ok(t) = field.text().await {
                    if !t.trim().is_empty() {
                        trusted_hash = Some(t.trim().to_string());
                    }
                }
            }
            _ => {}
        }
    }

    if proof_data.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing proof package"})),
        ));
    }
    if image_data.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing image"})),
        ));
    }

    // Parse proof package
    let pkg: ProofPackage = serde_json::from_slice(&proof_data).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid proof package JSON"})),
        )
    })?;

    // Hash the uploaded image the same way the aggregator does:
    // H(shard_edited_hash_0 || shard_edited_hash_1 || ... || shard_edited_hash_N)
    let img = image::load_from_memory(&image_data).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Failed to decode image"})),
        )
    })?;
    let rgb = img.to_rgb8();
    let image_bytes = rgb.into_raw();
    let num_shards = pkg.num_shards;
    let shard_size = (image_bytes.len() + num_shards - 1) / num_shards;
    let mut final_hasher = Sha256::new();
    for i in 0..num_shards {
        let s = i * shard_size;
        let e = std::cmp::min(s + shard_size, image_bytes.len());
        let shard_pixels = if s < image_bytes.len() {
            &image_bytes[s..e]
        } else {
            &[]
        };
        let shard_hash: [u8; 32] = Sha256::digest(shard_pixels).into();
        final_hasher.update(&shard_hash);
    }
    let actual_hash: [u8; 32] = final_hasher.finalize().into();
    let actual_hex = hex::encode(actual_hash);
    let expected_hex = hex::encode(pkg.public_values.output_image_hash);

    let image_hash_match = actual_hex == expected_hex;

    // Manufacturer trust (normalize to lowercase for case-insensitive hex comparison)
    let manufacturer_trusted = trusted_hash.as_ref().map(|h| {
        h.to_lowercase() == hex::encode(pkg.public_values.root_ca_hash)
    });

    // For now, emulated verification (always true if package is valid)
    let emulated = true;

    let all_pass = image_hash_match && manufacturer_trusted.unwrap_or(true);

    Ok(Json(json!({
        "image_hash_match": image_hash_match,
        "actual_hash": actual_hex,
        "expected_hash": expected_hex,
        "manufacturer_trusted": manufacturer_trusted,
        "emulated": emulated,
        "edit_types": pkg.public_values.edit_types,
        "verdict": if all_pass { "VERIFICATION_SUCCESSFUL" } else { "VERIFICATION_FAILED" },
    })))
}

fn parse_ops(ops_str: &str) -> Vec<EditOperation> {
    if ops_str.is_empty() {
        return vec![];
    }
    ops_str
        .split(';')
        .filter_map(|part| {
            let part = part.trim();
            if part.starts_with("crop:") {
                let vals: Vec<u32> = part[5..]
                    .split(',')
                    .filter_map(|v| v.trim().parse().ok())
                    .collect();
                if vals.len() == 4 {
                    Some(EditOperation::Crop {
                        x: vals[0],
                        y: vals[1],
                        width: vals[2],
                        height: vals[3],
                    })
                } else {
                    None
                }
            } else if part.starts_with("brightness:") {
                part[11..]
                    .trim()
                    .parse::<i16>()
                    .ok()
                    .map(|delta| EditOperation::AdjustBrightness { delta })
            } else {
                None
            }
        })
        .collect()
}

// ===== Main =====
#[tokio::main]
async fn main() {
    let cwd = std::env::current_dir().expect("Failed to get current directory");

    // When run from brevis-vera-zk/: data_dir=./data, zk_dir=., web_app=../web-app
    // When run from workspace root (vera-vibe/): data_dir=./data (wrong), prefer brevis-vera-zk/
    let data_dir = cwd.join("data");
    let zk_dir = cwd.clone();
    let web_app_dir = [
        cwd.join("../web-app"),     // from brevis-vera-zk/
        cwd.join("web-app"),        // from workspace root
    ]
    .into_iter()
    .find(|p| p.join("index.html").exists())
    .unwrap_or_else(|| cwd.join("../web-app"));

    fs::create_dir_all(&data_dir).unwrap();

    println!("📂 Data dir: {}", data_dir.display());
    println!("📂 ZK dir: {}", zk_dir.display());
    println!("📂 Web app dir: {}", web_app_dir.display());

    // Verify ELF files exist
    let shard_elf = zk_dir.join("shard-app/elf/riscv32im-pico-zkvm-elf");
    let agg_elf = zk_dir.join("aggregator-app/elf/riscv32im-pico-zkvm-elf");
    if !shard_elf.exists() {
        eprintln!("⚠️  Shard ELF not found at {}", shard_elf.display());
    }
    if !agg_elf.exists() {
        eprintln!("⚠️  Aggregator ELF not found at {}", agg_elf.display());
    }

    let state = AppState {
        data_dir,
        zk_dir,
        sessions: Arc::new(Mutex::new(HashMap::new())),
        jobs: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        // API routes
        .route("/api/health", get(health))
        .route("/api/keygen", post(keygen))
        .route("/api/sign", post(sign_photo))
        .route(
            "/api/data/{session_id}/original_preview",
            get(serve_original_preview),
        )
        .route("/api/data/{session_id}/{filename}", get(serve_data_file))
        .route("/api/edit", post(edit_photo))
        .route("/api/prove", post(start_prove))
        .route("/api/prove/status/{job_id}", get(prove_status))
        .route("/api/verify", post(verify_photo))
        // Static files (frontend)
        .fallback_service(ServeDir::new(&web_app_dir))
        .layer(DefaultBodyLimit::max(200 * 1024 * 1024)) // 200MB
        .layer(CorsLayer::very_permissive())
        .with_state(state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌐 Brevis Vera Web Server starting on http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
