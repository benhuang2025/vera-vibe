use pico_sdk::{client::DefaultProverClient, init_logger};
use brevis_vera_lib::{SignedPhoto, EditManifest, PublicValues, ProofPackage};
use std::fs;
use std::path::PathBuf;
use clap::Parser;

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
    let signed_photo: SignedPhoto = serde_json::from_str(&photo_json).expect("Failed to parse photo JSON");

    let manifest_json = fs::read_to_string(&cli.manifest).expect("Failed to read manifest JSON");
    let manifest: EditManifest = serde_json::from_str(&manifest_json).expect("Failed to parse manifest JSON");

    // 2. Setup ZKVM
    let elf = load_elf("../app/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let mut stdin_builder = client.new_stdin_builder();

    stdin_builder.write(&signed_photo);
    stdin_builder.write(&manifest);

    // 3. Generate Proof
    // On Mac, we'll use emulation first to verify logic, then try prove_fast.
    // Given the previous OOM, let's stick to 'emulate' for testing logic, 
    // but the final goal is 'prove_fast'.
    
    println!("Emulating in ZKVM to verify logic...");
    let (reports, public_buffer) = client.emulate(stdin_builder);
    let total_cycles = reports.last().map(|r| r.current_cycle).unwrap_or(0);
    println!("✅ Emulation finished. Total cycles: {}", total_cycles);

    let public_values: PublicValues = bincode::deserialize(&public_buffer)
        .expect("Failed to deserialize public values");

    println!("--- Public Commitments ---");
    println!("Public Key Hash: {}", hex::encode(public_values.pub_key_hash));
    println!("Edit Operations: {:?}", public_values.edit_types);
    println!("Output Image Hash: {}", hex::encode(public_values.output_image_hash));

    // 4. Save Proof Package (Placeholder for actual proof bytes since we emulated)
    let edited_image_bytes = fs::read(&cli.edited_image).unwrap_or_default();
    
    let package = ProofPackage {
        edited_image: edited_image_bytes,
        proof: vec![0u8; 32], // Placeholder
        public_values,
    };

    let pkg_json = serde_json::to_string_pretty(&package).expect("Failed to serialize package");
    fs::write(&cli.output, pkg_json).expect("Failed to write proof package");
    
    println!("📦 Proof package saved to {:?}", cli.output);
}
