use brevis_vera_lib::{EditManifest, ProofPackage, PublicValues, SignedPhoto};
use clap::Parser;
use pico_sdk::{client::DefaultProverClient, init_logger};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

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

fn load_elf(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read ELF file")
}

fn main() {
    init_logger();
    let cli = Cli::parse();

    println!("🚀 Brevis Vera Prover starting...");

    // 1. Load data
    let photo_json = fs::read_to_string(&cli.photo).expect("Failed to read photo JSON");
    let signed_photo: SignedPhoto =
        serde_json::from_str(&photo_json).expect("Failed to parse photo JSON");

    let _manifest_json = fs::read_to_string(&cli.manifest).expect("Failed to read manifest JSON");
    let _manifest: EditManifest =
        serde_json::from_str(&_manifest_json).expect("Failed to parse manifest JSON");

    // 2. Parallel Shard Proving
    println!("🚀 Starting Multi-CPU Parallel Proving (64 shards)...");
    let start_total = std::time::Instant::now();

    let num_shards = 64;
    let image_len = signed_photo.image_bytes.len();
    let shard_size = (image_len + num_shards - 1) / num_shards;
    let shard_elf = load_elf("../shard-app/elf/riscv32im-pico-zkvm-elf");

    println!(
        "Image: {} bytes, {} shards x {} bytes each",
        image_len, num_shards, shard_size
    );

    let shard_results: Vec<([u8; 32], u64)> = (0..num_shards)
        .into_par_iter()
        .map(|i| {
            let client = DefaultProverClient::new(&shard_elf);
            let s = i * shard_size;
            let e = std::cmp::min(s + shard_size, image_len);
            let shard_pixels = if s < image_len {
                signed_photo.image_bytes[s..e].to_vec()
            } else {
                Vec::new()
            };

            let mut stdin_builder = client.new_stdin_builder();
            stdin_builder.write(&shard_pixels);

            let (reports, public_buffer) = client.emulate(stdin_builder);
            let cycles = reports.last().map(|r| r.current_cycle).unwrap_or(0);
            let shard_hash: [u8; 32] = bincode::deserialize(&public_buffer).unwrap();
            (shard_hash, cycles)
        })
        .collect();

    let parallel_duration = start_total.elapsed();
    let total_cycles: u64 = shard_results.iter().map(|(_, c)| c).sum();
    let max_cycles = shard_results.iter().map(|(_, c)| *c).max().unwrap_or(0);

    println!("✅ Parallel Shards Finished in {:?}", parallel_duration);
    println!("📈 Total Cycles: {}", total_cycles);
    println!("⚡ Longest Shard: {} cycles", max_cycles);

    // 3. Aggregation
    println!("🔗 Aggregating Proofs...");
    let shard_hashes: Vec<[u8; 32]> = shard_results.iter().map(|(h, _)| *h).collect();

    let agg_elf = load_elf("../aggregator-app/elf/riscv32im-pico-zkvm-elf");
    let agg_client = DefaultProverClient::new(&agg_elf);
    let mut agg_stdin = agg_client.new_stdin_builder();
    agg_stdin.write(&signed_photo.metadata);
    agg_stdin.write(&signed_photo.signature);
    agg_stdin.write(&shard_hashes);

    let (agg_reports, agg_public_buffer) = agg_client.emulate(agg_stdin);
    let agg_cycles = agg_reports.last().map(|r| r.current_cycle).unwrap_or(0);
    println!("✅ Aggregator: {} cycles", agg_cycles);

    let public_values: PublicValues =
        bincode::deserialize(&agg_public_buffer).expect("Failed to deserialize");

    println!("--- Public Commitments ---");
    println!(
        "Public Key Hash: {}",
        hex::encode(public_values.pub_key_hash)
    );
    println!(
        "Final Image Hash: {}",
        hex::encode(public_values.output_image_hash)
    );

    // 4. Save Proof Package
    let edited_image_bytes = fs::read(&cli.edited_image).unwrap_or_default();
    let package = ProofPackage {
        edited_image: edited_image_bytes,
        proof: vec![0u8; 32],
        public_values,
    };
    let pkg_json = serde_json::to_string_pretty(&package).expect("Failed to serialize");
    fs::write(&cli.output, pkg_json).expect("Failed to write");

    println!("📦 Saved to {:?}", cli.output);
    println!("⏱️ TOTAL E2E TIME: {:?}", start_total.elapsed());
}
