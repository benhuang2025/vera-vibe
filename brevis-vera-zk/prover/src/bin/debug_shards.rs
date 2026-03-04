use brevis_vera_lib::{SignedPhoto, BLOCK_SIZE, compute_block_hashes, compute_image_commitment};
use std::fs;

fn main() {
    let path = "/home/ubuntu/ben/vera-vibe/signed_p16.json";
    let data = fs::read_to_string(path).expect("Read fail");
    let signed_photo: SignedPhoto = serde_json::from_str(&data).expect("Parse fail");

    let image_len = signed_photo.image_bytes.len();
    let block_hashes = compute_block_hashes(&signed_photo.image_bytes);
    let commitment = compute_image_commitment(&block_hashes);

    println!("Total image len: {}", image_len);
    println!("Blocks: {} (BLOCK_SIZE={})", block_hashes.len(), BLOCK_SIZE);
    println!("Image commitment (metadata): {}", hex::encode(signed_photo.metadata.image_commitment));
    println!("Image commitment (computed): {}", hex::encode(commitment));

    for (i, h) in block_hashes.iter().enumerate() {
        println!("Block {}: {}", i, hex::encode(h));
    }
}
