#![no_main]

pico_sdk::entrypoint!(main);
use brevis_vera_lib::{EditOperation, BLOCK_SIZE};
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};

pub fn main() {
    // 1. Read input: (shard_pixels, edit_ops, num_blocks_in_shard)
    let (pixel_segment, edit_ops, num_blocks): (Vec<u8>, Vec<EditOperation>, usize) = read_as();

    // 2. Compute per-block original hashes
    let mut orig_block_hashes: Vec<[u8; 32]> = Vec::with_capacity(num_blocks);
    for i in 0..num_blocks {
        let s = i * BLOCK_SIZE;
        let e = if s + BLOCK_SIZE < pixel_segment.len() {
            s + BLOCK_SIZE
        } else {
            pixel_segment.len()
        };
        orig_block_hashes.push(Sha256::digest(&pixel_segment[s..e]).into());
    }

    // 3. Apply Transformations (actual pixel math inside ZK!)
    let mut current_pixels = pixel_segment;
    for op in edit_ops {
        match op {
            EditOperation::AdjustBrightness { delta } => {
                current_pixels =
                    brevis_vera_lib::pixel_utils::apply_brightness(&current_pixels, delta);
            }
            EditOperation::AdjustContrast { factor } => {
                current_pixels =
                    brevis_vera_lib::pixel_utils::apply_contrast(&current_pixels, factor);
            }
            _ => { /* Crop, Rotate90, Grayscale need full-image context → host handles */ }
        }
    }

    // 4. Compute per-block edited hashes
    let mut edited_block_hashes: Vec<[u8; 32]> = Vec::with_capacity(num_blocks);
    for i in 0..num_blocks {
        let s = i * BLOCK_SIZE;
        let e = if s + BLOCK_SIZE < current_pixels.len() {
            s + BLOCK_SIZE
        } else {
            current_pixels.len()
        };
        edited_block_hashes.push(Sha256::digest(&current_pixels[s..e]).into());
    }

    // 5. Commit per-block hash pairs
    commit(&(orig_block_hashes, edited_block_hashes));
}
