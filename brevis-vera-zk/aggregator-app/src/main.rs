#![no_main]

pico_sdk::entrypoint!(main);
use brevis_vera_lib::{PhotoMetadata, PublicValues, Signature, compute_image_commitment};
use p256::ecdsa::{Signature as SignatureEcdsa, VerifyingKey, signature::Verifier};
use pico_sdk::io::{commit, read_as};
use sha2::{Digest, Sha256};

pub fn main() {
    // 1. Read Inputs — only block hashes, NOT raw pixels
    let metadata: PhotoMetadata = read_as();
    let signature: Signature = read_as();
    let all_orig_block_hashes: Vec<[u8; 32]> = read_as();
    let all_edited_block_hashes: Vec<[u8; 32]> = read_as();

    // 2. Identity Verification: ECDSA P-256 device signature check
    let meta_bytes = metadata.to_bytes();
    let vk = VerifyingKey::from_sec1_bytes(&signature.public_key).expect("VK_INIT_FAIL");
    let r_bytes = p256::FieldBytes::from(signature.r);
    let s_bytes = p256::FieldBytes::from(signature.s);
    let sig = SignatureEcdsa::from_scalars(r_bytes, s_bytes).expect("SIG_INIT_FAIL");
    vk.verify(&meta_bytes, &sig).expect("SIG_VERIFY_FAIL");

    // 2b. Trust Anchor
    let root_ca_hash: [u8; 32] = Sha256::digest(&signature.root_ca_pubkey).into();

    // 3. Hard Linkage: verify block hashes commitment matches the signed commitment
    let computed_commitment = compute_image_commitment(&all_orig_block_hashes);
    assert_eq!(
        computed_commitment, metadata.image_commitment,
        "COMMITMENT_MISMATCH"
    );

    // 4. Compute output image hash from edited block hashes
    let output_image_hash = compute_image_commitment(&all_edited_block_hashes);

    // 5. Compute device identity hash
    let pub_key_hash: [u8; 32] = Sha256::digest(&signature.public_key).into();

    // 6. Final Public Commitment
    commit(&PublicValues {
        original_image_commitment: metadata.image_commitment,
        pub_key_hash,
        root_ca_hash,
        edit_types: vec!["ParallelAOT".to_string()],
        output_image_hash,
    });
}
