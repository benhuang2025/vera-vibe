#![no_main]

pico_sdk::entrypoint!(main);
use brevis_vera_lib::{PhotoMetadata, PublicValues, Signature};
use p256::ecdsa::{Signature as SignatureEcdsa, VerifyingKey, signature::Verifier};
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};

pub fn main() {
    // 1. Read Inputs
    let metadata: PhotoMetadata = read_as();
    let signature: Signature = read_as();
    let shard_results: Vec<([u8; 32], [u8; 32])> = read_as(); // (orig_hash, edited_hash)

    // 2. Identity Verification: ECDSA P-256 signature check
    let meta_bytes = metadata.to_bytes();
    let vk = VerifyingKey::from_sec1_bytes(&signature.public_key).expect("VK_INIT_FAIL");
    let r_bytes = p256::FieldBytes::from(signature.r);
    let s_bytes = p256::FieldBytes::from(signature.s);
    let sig = SignatureEcdsa::from_scalars(r_bytes, s_bytes).expect("SIG_INIT_FAIL");
    vk.verify(&meta_bytes, &sig).expect("SIG_VERIFY_FAIL");

    // 3. Hard Linkage: each shard's orig_hash MUST match the signed metadata
    assert_eq!(
        shard_results.len(),
        metadata.shards.len(),
        "SHARD_LEN_MISMATCH"
    );

    let mut final_hasher = Sha256::new();
    for (i, (shard_orig, shard_edited)) in shard_results.iter().enumerate() {
        if shard_orig != &metadata.shards[i] {
            panic!("HARD_LINK_FAILURE_AT_SHARD_{}", i);
        }
        final_hasher.update(shard_edited);
    }
    let output_image_hash: [u8; 32] = final_hasher.finalize().into();

    // 4. Trust Anchor
    let mut pub_key_hasher = Sha256::new();
    pub_key_hasher.update(&signature.public_key);
    let pub_key_hash: [u8; 32] = pub_key_hasher.finalize().into();
    let root_ca_hash = [0u8; 32]; // placeholder for real Root CA verification

    // 5. Final Public Commitment
    commit(&PublicValues {
        original_image_hash: metadata.image_hash,
        pub_key_hash,
        root_ca_hash,
        edit_types: vec!["ParallelAOT".to_string()],
        output_image_hash,
    });
}
