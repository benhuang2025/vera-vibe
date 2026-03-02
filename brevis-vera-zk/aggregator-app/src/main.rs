#![no_main]

pico_sdk::entrypoint!(main);
use brevis_vera_lib::{PhotoMetadata, PublicValues, Signature};
use p256::ecdsa::{Signature as SignatureEcdsa, VerifyingKey, signature::Verifier};
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};

pub fn main() {
    // 1. Read Inputs (Optimized: No large image_bytes!)
    let metadata: PhotoMetadata = read_as();
    let signature: Signature = read_as();
    let shard_hashes: Vec<[u8; 32]> = read_as();

    // 2. Verify ECDSA signature of metadata
    let meta_bytes = metadata.to_bytes();

    // We can't really "println" easily in Guest without overhead,
    // but we can use assert messages or commit debug info.
    // For now, let's just make sure the verification is correct.

    let vk = VerifyingKey::from_sec1_bytes(&signature.public_key).expect("VK_INIT_FAIL");

    // Convert [u8; 32] to FieldBytes (GenericArray)
    let r_bytes = p256::FieldBytes::from(signature.r);
    let s_bytes = p256::FieldBytes::from(signature.s);
    let sig = SignatureEcdsa::from_scalars(r_bytes, s_bytes).expect("SIG_INIT_FAIL");

    vk.verify(&meta_bytes, &sig).expect("SIG_VERIFY_FAIL");

    // 3. Verify Shard Consistency
    assert_eq!(
        shard_hashes.len(),
        metadata.shards.len(),
        "SHARD_LEN_MISMATCH"
    );
    for (i, (&h_in, &h_meta)) in shard_hashes.iter().zip(metadata.shards.iter()).enumerate() {
        if h_in != h_meta {
            // In Pico ZKVM, a failed assert with a message is better than nothing
            panic!("SHARD_HASH_MISMATCH_AT_{}", i);
        }
    }

    // 4. Final Commit
    let mut pub_key_hasher = Sha256::new();
    pub_key_hasher.update(&signature.public_key);
    let pub_key_hash: [u8; 32] = pub_key_hasher.finalize().into();

    commit(&PublicValues {
        pub_key_hash,
        edit_types: vec!["ParallelVerification".to_string()],
        output_image_hash: metadata.image_hash,
        shard_hashes,
    });
}
