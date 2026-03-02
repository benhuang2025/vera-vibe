#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};

pub fn main() {
    // 1. Read raw pixels via bincode (matches prover's stdin_builder.write())
    let pixel_segment: Vec<u8> = read_as();

    // 2. Compute Hash
    let mut hasher = Sha256::new();
    hasher.update(&pixel_segment);
    let shard_hash: [u8; 32] = hasher.finalize().into();

    // 3. Commit the hash
    commit(&shard_hash);
}
