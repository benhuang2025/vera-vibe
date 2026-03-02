#![no_main]

pico_sdk::entrypoint!(main);
use brevis_vera_lib::EditOperation;
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};

pub fn main() {
    // 1. Read input: (pixel_segment, edit_operations)
    let (pixel_segment, edit_ops): (Vec<u8>, Vec<EditOperation>) = read_as();

    // 2. Compute Original Hash (Hard Link: proves these pixels came from the signed image)
    let mut orig_hasher = Sha256::new();
    orig_hasher.update(&pixel_segment);
    let orig_hash: [u8; 32] = orig_hasher.finalize().into();

    // 3. Apply Transformations (actual pixel math inside ZK!)
    let mut current_pixels = pixel_segment;
    for op in edit_ops {
        match op {
            EditOperation::AdjustBrightness { delta } => {
                current_pixels =
                    brevis_vera_lib::pixel_utils::apply_brightness(&current_pixels, delta);
            }
            _ => { /* Crop is handled by host-level sharding for now */ }
        }
    }

    // 4. Compute Edited Hash
    let mut final_hasher = Sha256::new();
    final_hasher.update(&current_pixels);
    let final_hash: [u8; 32] = final_hasher.finalize().into();

    // 5. Commit both hashes: (original, edited)
    commit(&(orig_hash, final_hash));
}
