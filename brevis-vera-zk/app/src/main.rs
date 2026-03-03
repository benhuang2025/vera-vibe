#![no_main]

pico_sdk::entrypoint!(main);
use brevis_vera_lib::{EditManifest, EditOperation, PublicValues, SignedPhoto, pixel_utils, compute_block_hashes, compute_image_commitment};
use p256::ecdsa::{Signature, VerifyingKey, signature::Verifier};
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};

pub fn main() {
    // 1. Read Inputs
    let signed_photo: SignedPhoto = read_as();
    let manifest: EditManifest = read_as();

    // 2. Verify Authenticity (ECDSA P-256)
    let block_hashes = compute_block_hashes(&signed_photo.image_bytes);
    let computed_commitment = compute_image_commitment(&block_hashes);

    assert_eq!(
        computed_commitment, signed_photo.metadata.image_commitment,
        "Image data does not match metadata commitment"
    );

    let metadata_bytes = signed_photo.metadata.to_bytes();
    let verifying_key = VerifyingKey::from_sec1_bytes(&signed_photo.signature.public_key)
        .expect("Invalid public key");
    let signature = Signature::from_scalars(signed_photo.signature.r, signed_photo.signature.s)
        .expect("Invalid signature scalars");

    verifying_key
        .verify(&metadata_bytes, &signature)
        .expect("Signature verification failed");

    // 3. Replay Edits (Integrity)
    let mut current_pixels = signed_photo.image_bytes;
    let mut current_w = signed_photo.metadata.width;
    let mut current_h = signed_photo.metadata.height;
    let mut edit_types = Vec::new();

    for op in manifest.operations {
        edit_types.push(op.name());
        match op {
            EditOperation::Crop {
                x,
                y,
                width,
                height,
            } => {
                current_pixels = pixel_utils::apply_crop(
                    &current_pixels,
                    current_w,
                    current_h,
                    x,
                    y,
                    width,
                    height,
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
    let edited_block_hashes = compute_block_hashes(&current_pixels);
    let output_image_hash = compute_image_commitment(&edited_block_hashes);

    // 5. Commit Public Values
    let pub_key_hash: [u8; 32] = Sha256::digest(&signed_photo.signature.public_key).into();

    commit(&PublicValues {
        original_image_commitment: signed_photo.metadata.image_commitment,
        pub_key_hash,
        root_ca_hash: [0u8; 32],
        edit_types,
        output_image_hash,
    });
}
