#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};
use sha2::{Sha256, Digest};
use p256::ecdsa::{VerifyingKey, Signature, signature::Verifier};
use brevis_vera_lib::{SignedPhoto, EditManifest, EditOperation, PublicValues, pixel_utils};

pub fn main() {
    // 1. Read Inputs
    let signed_photo: SignedPhoto = read_as();
    let manifest: EditManifest = read_as();

    // 2. Verify Authenticity (ECDSA P-256)
    // First, verify that the metadata reflects the provided image hash
    let mut hasher = Sha256::new();
    hasher.update(&signed_photo.image_bytes);
    let computed_image_hash: [u8; 32] = hasher.finalize().into();
    
    assert_eq!(
        computed_image_hash, 
        signed_photo.metadata.image_hash, 
        "Image data does not match metadata hash"
    );

    // Second, verify the signature over the metadata
    let metadata_bytes = serde_json::to_vec(&signed_photo.metadata).unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(&signed_photo.signature.public_key)
        .expect("Invalid public key");
    let signature = Signature::from_scalars(
        signed_photo.signature.r, 
        signed_photo.signature.s
    ).expect("Invalid signature scalars");

    verifying_key.verify(&metadata_bytes, &signature)
        .expect("Signature verification failed");

    // 3. Replay Edits (Integrity)
    let mut current_pixels = signed_photo.image_bytes;
    let mut current_w = signed_photo.metadata.width;
    let mut current_h = signed_photo.metadata.height;
    let mut edit_types = Vec::new();

    for op in manifest.operations {
        edit_types.push(op.name());
        match op {
            EditOperation::Crop { x, y, width, height } => {
                current_pixels = pixel_utils::apply_crop(
                    &current_pixels, current_w, current_h, x, y, width, height
                );
                current_w = width;
                current_h = height;
            }
            EditOperation::AdjustBrightness { delta } => {
                current_pixels = pixel_utils::apply_brightness(&current_pixels, delta);
            }
        }
    }

    // 4. Compute Final Hash
    let mut final_hasher = Sha256::new();
    final_hasher.update(&current_pixels);
    let output_image_hash: [u8; 32] = final_hasher.finalize().into();

    // 5. Commit Public Values
    let mut pub_key_hasher = Sha256::new();
    pub_key_hasher.update(&signed_photo.signature.public_key);
    let pub_key_hash: [u8; 32] = pub_key_hasher.finalize().into();

    commit(&PublicValues {
        pub_key_hash,
        edit_types,
        output_image_hash,
    });
}
