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
    /// Generate a real STARK proof (slower but cryptographically verifiable)
    #[arg(long, default_value = "false")]
    real_proof: bool,
}

fn load_program(path: &str) -> Arc<Program> {
    let elf_bytes = fs::read(path).expect("Failed to read ELF file");
    Compiler::new(SourceType::RISCV, &elf_bytes).compile()
}

fn main() {
    init_logger();
    let cli = Cli::parse();

    println!("🚀 Brevis Vera Prover (AOT) starting...");
    if cli.real_proof {
        println!("🔐 Real STARK proof mode enabled");
    }

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

    // 2. Parallel Shard Proving (AOT emulation) — block-based (Plan D)
    use brevis_vera_lib::BLOCK_SIZE;

    let image_len = signed_photo.image_bytes.len();
    let num_blocks = (image_len + BLOCK_SIZE - 1) / BLOCK_SIZE;
    let num_shards = num_cpus::get().max(1);
    let blocks_per_shard = (num_blocks + num_shards - 1) / num_shards;

    println!(
        "🚀 Plan D: {} bytes, {} blocks ({}B each), {} shards ({} blocks/shard)",
        image_len, num_blocks, BLOCK_SIZE, num_shards, blocks_per_shard
    );
    let start_total = std::time::Instant::now();

    let shard_program = load_program("../shard-app/elf/riscv32im-pico-zkvm-elf");

    let shard_results: Vec<((Vec<[u8; 32]>, Vec<[u8; 32]>), u64)> = (0..num_shards)
        .into_par_iter()
        .map(|i| {
            let block_start = i * blocks_per_shard;
            let block_end = std::cmp::min(block_start + blocks_per_shard, num_blocks);
            if block_start >= num_blocks {
                return ((vec![], vec![]), 0u64);
            }
            let pixel_start = block_start * BLOCK_SIZE;
            let pixel_end = std::cmp::min(block_end * BLOCK_SIZE, image_len);
            let shard_pixels = signed_photo.image_bytes[pixel_start..pixel_end].to_vec();
            let num_blocks_in_shard = block_end - block_start;

            let shard_input =
                bincode::serialize(&(&shard_pixels, &_manifest.operations, &num_blocks_in_shard))
                    .unwrap();
            let mut emu = AotEmulatorCore::new(shard_program.clone(), vec![shard_input]);
            shard_aot::run_aot(&mut emu).expect("Shard AOT run failed");

            let hashes: (Vec<[u8; 32]>, Vec<[u8; 32]>) =
                bincode::deserialize(&emu.public_values_stream)
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

    // 3. Flatten block hashes from all shards
    let mut all_orig_block_hashes: Vec<[u8; 32]> = Vec::with_capacity(num_blocks);
    let mut all_edited_block_hashes: Vec<[u8; 32]> = Vec::with_capacity(num_blocks);
    for ((orig, edited), _) in &shard_results {
        all_orig_block_hashes.extend_from_slice(orig);
        all_edited_block_hashes.extend_from_slice(edited);
    }
    println!(
        "📊 Block hashes: {} orig, {} edited",
        all_orig_block_hashes.len(),
        all_edited_block_hashes.len()
    );

    let public_values: PublicValues;
    let proof_bytes: Vec<u8>;

    if cli.real_proof {
        // ===== REAL STARK PROOF MODE =====
        println!("🔗 Generating Real STARK Proof for Aggregator...");

        use pico_vm::configs::config::StarkGenericConfig;
        use pico_vm::configs::stark_config::KoalaBearPoseidon2;
        use pico_vm::emulator::stdin::EmulatorStdinBuilder;
        use pico_vm::proverchain::{InitialProverSetup, MachineProver, RiscvProver};

        let agg_elf = fs::read("../aggregator-app/elf/riscv32im-pico-zkvm-elf")
            .expect("Failed to read aggregator ELF");

        let prover = RiscvProver::new_initial_prover(
            (KoalaBearPoseidon2::new(), agg_elf.as_slice()),
            Default::default(),
            None,
        );

        let mut stdin_builder = EmulatorStdinBuilder::<Vec<u8>, KoalaBearPoseidon2>::default();
        stdin_builder.write(&signed_photo.metadata);
        stdin_builder.write(&signed_photo.signature);
        stdin_builder.write(&all_orig_block_hashes);
        stdin_builder.write(&all_edited_block_hashes);
        let (stdin, _) = stdin_builder.finalize();

        // Generate STARK proof
        let start_prove = std::time::Instant::now();
        let meta_proof = prover.prove(stdin);
        let prove_duration = start_prove.elapsed();
        println!("✅ Real STARK Proof generated in {:?}", prove_duration);

        // Verify the proof
        let vk = prover.vk();
        let verified = prover.verify(&meta_proof, vk);
        if verified {
            println!("✅ STARK Proof verified successfully!");
        } else {
            println!("❌ STARK Proof verification FAILED!");
            std::process::exit(1);
        }

        // Extract public values
        let pv_stream = meta_proof.pv_stream.clone().unwrap_or_default();
        public_values = bincode::deserialize(&pv_stream)
            .expect("Failed to deserialize aggregator public values");

        // Save proof and VK to files
        let proof_dir = cli.output.parent().unwrap_or(std::path::Path::new("."));
        let proof_path = proof_dir.join("stark_proof.bin");
        let vk_path = proof_dir.join("stark_vk.bin");

        meta_proof
            .save_to_file(&proof_path)
            .expect("Failed to save proof");
        println!("💾 STARK proof saved to {:?}", proof_path);

        // Serialize VK
        let vk_bytes = bincode::serialize(vk).expect("Failed to serialize VK");
        fs::write(&vk_path, &vk_bytes).expect("Failed to write VK");
        println!("💾 Verifying key saved to {:?}", vk_path);

        // Serialize proof reference for the package
        proof_bytes = bincode::serialize(&("stark_proof.bin", "stark_vk.bin"))
            .expect("Failed to serialize proof reference");
    } else {
        // ===== EMULATION MODE (fast, no real proof) =====
        println!("🔗 Aggregating Proofs with AOT (emulation mode)...");
        let aggregator_program = load_program("../aggregator-app/elf/riscv32im-pico-zkvm-elf");

        let agg_stdin = vec![
            bincode::serialize(&signed_photo.metadata).unwrap(),
            bincode::serialize(&signed_photo.signature).unwrap(),
            bincode::serialize(&all_orig_block_hashes).unwrap(),
            bincode::serialize(&all_edited_block_hashes).unwrap(),
        ];

        let mut agg_emu = AotEmulatorCore::new(aggregator_program, agg_stdin);
        aggregator_aot::run_aot(&mut agg_emu).expect("Aggregator AOT run failed");

        public_values = bincode::deserialize(&agg_emu.public_values_stream)
            .expect("Failed to deserialize agg values");

        println!("✅ Aggregator: {} insns", agg_emu.insn_count);
        proof_bytes = vec![0u8; 32]; // placeholder
    }

    println!("--- Public Commitments ---");
    println!(
        "Original Image Commitment: {}",
        hex::encode(public_values.original_image_commitment)
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
        proof: proof_bytes,
        public_values,
    };
    let pkg_json = serde_json::to_string_pretty(&package).expect("Failed to serialize");
    fs::write(&cli.output, pkg_json).expect("Failed to write");

    println!("📦 Saved to {:?}", cli.output);
    println!("⏱️ TOTAL E2E TIME: {:?}", start_total.elapsed());
}
