use aggregator_aot;
use brevis_vera_lib::{EditManifest, ProofPackage, PublicValues, SignedPhoto};
use clap::Parser;
use pico_aot_runtime::AotEmulatorCore;
use pico_sdk::init_logger;
use pico_vm::compiler::riscv::{
    compiler::{Compiler, SourceType},
    program::Program,
};
use rayon::prelude::*;
use shard_aot;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "signed_photo.json")]
    photo: PathBuf,
    #[arg(short, long, default_value = "edit_manifest.json")]
    manifest: PathBuf,
    #[arg(short, long, default_value = "proof_package.json")]
    output: PathBuf,
    #[arg(short, long, default_value = "edited.png")]
    edited_image: PathBuf,
}

fn load_program(path: &str) -> Arc<Program> {
    let elf_bytes = fs::read(path).expect("Failed to read ELF file");
    Compiler::new(SourceType::RISCV, &elf_bytes).compile()
}

fn main() {
    init_logger();
    let cli = Cli::parse();

    println!("🚀 Brevis Vera Prover (AOT) starting...");

    // 1. Load data
    let photo_json = fs::read_to_string(&cli.photo).expect("Failed to read photo JSON");
    let signed_photo: SignedPhoto =
        serde_json::from_str(&photo_json).expect("Failed to parse photo JSON");

    let _manifest_json = fs::read_to_string(&cli.manifest).expect("Failed to read manifest JSON");
    let _manifest: EditManifest =
        serde_json::from_str(&_manifest_json).expect("Failed to parse manifest JSON");

    // 1b. Host-side Root CA cert chain verification
    {
        use p256::ecdsa::{Signature as Sig, VerifyingKey, signature::Verifier};
        let root_ca_vk = VerifyingKey::from_sec1_bytes(&signed_photo.signature.root_ca_pubkey)
            .expect("Invalid Root CA public key");
        let cert_r = p256::FieldBytes::from(signed_photo.signature.device_cert_r);
        let cert_s = p256::FieldBytes::from(signed_photo.signature.device_cert_s);
        let device_cert = Sig::from_scalars(cert_r, cert_s).expect("Invalid device certificate");
        root_ca_vk
            .verify(&signed_photo.signature.public_key, &device_cert)
            .expect("❌ FATAL: Root CA did not sign this device key!");
        println!("✅ Root CA → Device Key cert chain verified (host-side)");
    }

    // 2. Parallel Shard Proving
    let num_shards = signed_photo.metadata.shards.len();
    println!(
        "🚀 Starting Multi-CPU Parallel AOT Proving ({} shards)...",
        num_shards
    );
    let start_total = std::time::Instant::now();

    let image_len = signed_photo.image_bytes.len();
    let shard_size = (image_len + num_shards - 1) / num_shards;
    let shard_program = load_program("../shard-app/elf/riscv32im-pico-zkvm-elf");

    println!(
        "Image: {} bytes, {} shards x {} bytes each",
        image_len, num_shards, shard_size
    );

    let shard_results: Vec<(([u8; 32], [u8; 32]), u64)> = (0..num_shards)
        .into_par_iter()
        .map(|i| {
            let s = i * shard_size;
            let e = std::cmp::min(s + shard_size, image_len);
            let shard_pixels = if s < image_len {
                signed_photo.image_bytes[s..e].to_vec()
            } else {
                Vec::new()
            };

            // AOT RUN: send (pixels, edit_ops)
            let shard_input = bincode::serialize(&(&shard_pixels, &_manifest.operations)).unwrap();
            let mut emu = AotEmulatorCore::new(shard_program.clone(), vec![shard_input]);
            shard_aot::run_aot(&mut emu).expect("Shard AOT run failed");

            // Output: (orig_hash, edited_hash)
            let hashes: ([u8; 32], [u8; 32]) = bincode::deserialize(&emu.public_values_stream)
                .expect("Failed to parse shard commit");
            (hashes, emu.insn_count)
        })
        .collect();

    let parallel_duration = start_total.elapsed();
    let total_insns: u64 = shard_results.iter().map(|(_, c)| c).sum();
    let max_insns = shard_results.iter().map(|(_, c)| *c).max().unwrap_or(0);

    println!("✅ Parallel Shards Finished in {:?}", parallel_duration);
    println!("📈 Total Insns: {}", total_insns);
    println!("⚡ Longest Shard: {} insns", max_insns);

    // 3. Aggregation
    println!("🔗 Aggregating Proofs with AOT...");
    let aggregator_program = load_program("../aggregator-app/elf/riscv32im-pico-zkvm-elf");

    let shard_commits: Vec<([u8; 32], [u8; 32])> = shard_results.iter().map(|(h, _)| *h).collect();
    let agg_stdin = vec![
        bincode::serialize(&signed_photo.metadata).unwrap(),
        bincode::serialize(&signed_photo.signature).unwrap(),
        bincode::serialize(&shard_commits).unwrap(),
    ];

    let mut agg_emu = AotEmulatorCore::new(aggregator_program, agg_stdin);
    aggregator_aot::run_aot(&mut agg_emu).expect("Aggregator AOT run failed");

    let public_values: PublicValues = bincode::deserialize(&agg_emu.public_values_stream)
        .expect("Failed to deserialize agg values");

    println!("✅ Aggregator: {} insns", agg_emu.insn_count);

    println!("--- Public Commitments ---");
    println!(
        "Original Image Hash: {}",
        hex::encode(public_values.original_image_hash)
    );
    println!(
        "Public Key Hash: {}",
        hex::encode(public_values.pub_key_hash)
    );
    println!("Root CA Hash: {}", hex::encode(public_values.root_ca_hash));
    println!(
        "Edited Image Hash: {}",
        hex::encode(public_values.output_image_hash)
    );

    // 4. Save Proof Package
    let edited_image_bytes = fs::read(&cli.edited_image).unwrap_or_default();
    let package = ProofPackage {
        edited_image: edited_image_bytes,
        proof: vec![0u8; 32],
        public_values,
        num_shards,
    };
    let pkg_json = serde_json::to_string_pretty(&package).expect("Failed to serialize");
    fs::write(&cli.output, pkg_json).expect("Failed to write");

    println!("📦 Saved to {:?}", cli.output);
    println!("⏱️ TOTAL E2E TIME: {:?}", start_total.elapsed());
}
