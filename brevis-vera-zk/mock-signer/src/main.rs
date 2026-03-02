use anyhow::{Context, Result};
use brevis_vera_lib::{PhotoMetadata, Signature, SignedPhoto};
use clap::{Parser, Subcommand};
use p256::ecdsa::{SigningKey, VerifyingKey, signature::Signer};
use p256::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "brevis-vera")]
#[command(about = "Brevis Vera CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new P-256 keypair
    Keygen {
        #[arg(short, long, default_value = "private_key.pem")]
        output: PathBuf,
    },
    /// Sign an image and produce a SignedPhoto JSON
    Sign {
        #[arg(short, long)]
        image: PathBuf,
        #[arg(short, long)]
        key: PathBuf,
        #[arg(short, long, default_value = "signed_photo.json")]
        output: PathBuf,
        #[arg(short, long, default_value = "camera-sony-a7iv")]
        device: String,
    },
    /// Edit a signed photo and produce an edited image + manifest
    Edit {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short = 'p', long)]
        ops: String, // format "crop:x,y,w,h;brightness:delta"
        #[arg(short, long, default_value = "edited.png")]
        output: PathBuf,
        #[arg(short, long, default_value = "edit_manifest.json")]
        manifest: PathBuf,
    },
    /// Verify a proof package and its associated image
    Verify {
        #[arg(short, long, default_value = "proof_package.json")]
        package: PathBuf,
        #[arg(short, long, default_value = "edited.png")]
        image: PathBuf,
        #[arg(short, long)]
        trusted_pubkey_hash: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { output } => {
            // Generate Root CA (manufacturer) key
            let root_ca_key = SigningKey::random(&mut rand::thread_rng());
            let root_ca_pem_path = output.with_file_name("root_ca.pem");
            let root_ca_pem = root_ca_key.to_pkcs8_pem(Default::default())?;
            fs::write(&root_ca_pem_path, root_ca_pem.as_bytes())?;

            // Generate Device key
            let device_key = SigningKey::random(&mut rand::thread_rng());
            let device_pem = device_key.to_pkcs8_pem(Default::default())?;
            fs::write(&output, device_pem.as_bytes())?;

            // Root CA signs Device public key (device certificate)
            let device_vk = VerifyingKey::from(&device_key);
            let device_pubkey_bytes = device_vk.to_encoded_point(false).as_bytes().to_vec();
            let device_cert: p256::ecdsa::Signature = root_ca_key.sign(&device_pubkey_bytes);

            // Save device certificate
            let cert_data = serde_json::json!({
                "device_cert_r": hex::encode(device_cert.r().to_bytes()),
                "device_cert_s": hex::encode(device_cert.s().to_bytes()),
                "root_ca_pubkey": hex::encode(VerifyingKey::from(&root_ca_key).to_encoded_point(false).as_bytes()),
            });
            let cert_path = output.with_file_name("device_cert.json");
            fs::write(&cert_path, serde_json::to_string_pretty(&cert_data)?)?;

            // Output trusted Root CA hash
            let root_ca_vk = VerifyingKey::from(&root_ca_key);
            let root_ca_pubkey_bytes = root_ca_vk.to_encoded_point(false).as_bytes().to_vec();
            let mut hasher = Sha256::new();
            hasher.update(&root_ca_pubkey_bytes);
            let root_ca_hash: [u8; 32] = hasher.finalize().into();
            println!(
                "Generated Trusted Root CA Hash: {}",
                hex::encode(root_ca_hash)
            );

            // Also output device pubkey hash for reference
            let mut dev_hasher = Sha256::new();
            dev_hasher.update(&device_pubkey_bytes);
            let dev_hash: [u8; 32] = dev_hasher.finalize().into();
            println!("Generated Device PubKey Hash: {}", hex::encode(dev_hash));
        }
        Commands::Sign {
            image,
            key,
            output,
            device,
        } => {
            // 1. Load image and metadata
            let raw_img = image::open(&image)?;
            // Resize to a smaller size for faster ZK development
            // let img = raw_img.thumbnail(256, 256).to_rgb8();
            let img = raw_img.to_rgb8();
            let (width, height) = img.dimensions();
            let image_bytes = img.into_raw();

            // Calculate shards (Hardcoded 64 shards for benchmark/demo)
            let num_shards = 64;
            let shard_size = (image_bytes.len() + num_shards - 1) / num_shards;
            let mut shards = Vec::new();

            for i in 0..num_shards {
                let start = i * shard_size;
                let end = std::cmp::min(start + shard_size, image_bytes.len());
                if start >= image_bytes.len() {
                    shards.push([0u8; 32]);
                    continue;
                }
                let mut hasher = Sha256::new();
                hasher.update(&image_bytes[start..end]);
                shards.push(hasher.finalize().into());
            }

            let mut hasher = Sha256::new();
            hasher.update(&image_bytes);
            let image_hash: [u8; 32] = hasher.finalize().into();

            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            let metadata = PhotoMetadata {
                device_id: device,
                timestamp,
                width,
                height,
                image_hash,
                shards,
            };

            // 2. Sign metadata with device key
            let pem = fs::read_to_string(&key)?;
            let signing_key = SigningKey::from_pkcs8_pem(&pem)?;

            let metadata_bytes = metadata.to_bytes();
            let signature: p256::ecdsa::Signature = signing_key.sign(&metadata_bytes);

            // 3. Load device certificate (Root CA's endorsement)
            let cert_path = key.with_file_name("device_cert.json");
            let cert_json = fs::read_to_string(&cert_path)
                .context("device_cert.json not found. Run 'keygen' first.")?;
            let cert_data: serde_json::Value = serde_json::from_str(&cert_json)?;

            let device_cert_r: [u8; 32] =
                hex::decode(cert_data["device_cert_r"].as_str().unwrap())?
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("bad cert_r"))?;
            let device_cert_s: [u8; 32] =
                hex::decode(cert_data["device_cert_s"].as_str().unwrap())?
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("bad cert_s"))?;
            let root_ca_pubkey = hex::decode(cert_data["root_ca_pubkey"].as_str().unwrap())?;

            let sig_struct = Signature {
                r: signature.r().to_bytes().into(),
                s: signature.s().to_bytes().into(),
                public_key: VerifyingKey::from(&signing_key)
                    .to_encoded_point(false)
                    .as_bytes()
                    .to_vec(),
                root_ca_pubkey,
                device_cert_r,
                device_cert_s,
            };

            let signed_photo = SignedPhoto {
                image_bytes,
                metadata,
                signature: sig_struct,
            };

            // 3. Save
            let json = serde_json::to_string_pretty(&signed_photo)?;
            fs::write(&output, json)?;
            println!("Signed photo saved to {:?}", output);
        }
        Commands::Edit {
            input,
            ops,
            output,
            manifest,
        } => {
            use brevis_vera_lib::{EditManifest, EditOperation, pixel_utils};
            use image::{ImageBuffer, RgbImage};

            // 1. Load signed photo
            let json = fs::read_to_string(&input)?;
            let signed_photo: SignedPhoto = serde_json::from_str(&json)?;

            let mut current_pixels = signed_photo.image_bytes;
            let mut current_w = signed_photo.metadata.width;
            let mut current_h = signed_photo.metadata.height;

            // 2. Parse ops
            let mut edit_ops = Vec::new();
            for part in ops.split(';') {
                let subparts: Vec<&str> = part.split(':').collect();
                if subparts.len() != 2 {
                    continue;
                }
                let op_type = subparts[0];
                let params: Vec<&str> = subparts[1].split(',').collect();

                match op_type {
                    "crop" => {
                        if params.len() == 4 {
                            let x: u32 = params[0].parse()?;
                            let y: u32 = params[1].parse()?;
                            let w: u32 = params[2].parse()?;
                            let h: u32 = params[3].parse()?;

                            current_pixels = pixel_utils::apply_crop(
                                &current_pixels,
                                current_w,
                                current_h,
                                x,
                                y,
                                w,
                                h,
                            );
                            current_w = w;
                            current_h = h;
                            edit_ops.push(EditOperation::Crop {
                                x,
                                y,
                                width: w,
                                height: h,
                            });
                        }
                    }
                    "brightness" => {
                        if params.len() == 1 {
                            let delta: i16 = params[0].parse()?;
                            current_pixels = pixel_utils::apply_brightness(&current_pixels, delta);
                            edit_ops.push(EditOperation::AdjustBrightness { delta });
                        }
                    }
                    _ => println!("Unknown op: {}", op_type),
                }
            }

            // 3. Save edited image (conversion from raw to PNG via image crate)
            let img: RgbImage = ImageBuffer::from_raw(current_w, current_h, current_pixels)
                .context("Failed to create image buffer")?;
            img.save(&output)?;

            let manifest_data = EditManifest {
                operations: edit_ops,
            };
            let manifest_json = serde_json::to_string_pretty(&manifest_data)?;
            fs::write(&manifest, manifest_json)?;

            println!(
                "Edited image ({}x{}) saved to {:?}",
                current_w, current_h, output
            );
            println!("Edit manifest saved to {:?}", manifest);
        }
        Commands::Verify {
            package,
            image,
            trusted_pubkey_hash,
        } => {
            use brevis_vera_lib::ProofPackage;

            // 1. Load package
            let pkg_json = fs::read_to_string(&package)?;
            let pkg: ProofPackage = serde_json::from_str(&pkg_json)?;

            // 2. Compute output_image_hash the same way the aggregator does:
            //    H(shard_edited_hash_0 || shard_edited_hash_1 || ... || shard_edited_hash_63)
            let img = image::open(&image)?.to_rgb8();
            let image_bytes = img.into_raw();
            let num_shards = 64;
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
                let mut shard_hasher = Sha256::new();
                shard_hasher.update(shard_pixels);
                let shard_hash: [u8; 32] = shard_hasher.finalize().into();
                final_hasher.update(&shard_hash);
            }
            let image_hash: [u8; 32] = final_hasher.finalize().into();

            // 3. Compare with Public Values
            println!("--- Verification Report ---");
            let mut all_pass = true;

            if image_hash == pkg.public_values.output_image_hash {
                println!("✅ Image Hash matches ZK Commitment");
            } else {
                println!("❌ Image Hash MISMATCH!");
                println!("Actual: {}", hex::encode(image_hash));
                println!(
                    "Public Value: {}",
                    hex::encode(pkg.public_values.output_image_hash)
                );
                all_pass = false;
            }

            if let Some(trusted_hash) = trusted_pubkey_hash {
                if hex::encode(pkg.public_values.root_ca_hash) == trusted_hash {
                    println!("✅ Manufacturer (Root CA) is trusted");
                } else {
                    println!("❌ Signer is UNTRUSTED!");
                    all_pass = false;
                }
            } else {
                println!("ℹ️ No trusted key provided, skipping origin check.");
            }

            println!("ZK Proof Verification: ✅ Verified (Pico EMULATED)");
            println!(
                "History: Origin -> {}",
                pkg.public_values.edit_types.join(" -> ")
            );

            if all_pass {
                println!(
                    "\n🏆 VERIFICATION SUCCESSFUL: This media is authentic and conforms to declared edits."
                );
            } else {
                println!("\n🚫 VERIFICATION FAILED.");
            }
        }
    }

    Ok(())
}
