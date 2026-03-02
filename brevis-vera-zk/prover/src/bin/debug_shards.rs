use brevis_vera_lib::SignedPhoto;
use sha2::{Digest, Sha256};
use std::fs;

fn main() {
    let path = "/home/ubuntu/ben/vera-vibe/signed_p16.json";
    let data = fs::read_to_string(path).expect("Read fail");
    let signed_photo: SignedPhoto = serde_json::from_str(&data).expect("Parse fail");

    let num_shards = 16;
    let image_len = signed_photo.image_bytes.len();
    let shard_size = (image_len + num_shards - 1) / num_shards;

    println!("Total image len: {}", image_len);
    println!("Shard size: {}", shard_size);

    for i in 0..num_shards {
        let start = i * shard_size;
        let end = std::cmp::min(start + shard_size, image_len);
        let segment = &signed_photo.image_bytes[start..end];

        let mut hasher = Sha256::new();
        hasher.update(segment);
        let hash: [u8; 32] = hasher.finalize().into();

        let meta_hash = signed_photo.metadata.shards[i];

        println!("Shard {}: ", i);
        println!("  Calc: {}", hex::encode(hash));
        println!("  Meta: {}", hex::encode(meta_hash));

        if hash != meta_hash {
            println!("  ❌ MISMATCH!");
        } else {
            println!("  ✅ MATCH");
        }
    }
}
