use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Fixed block size for image hashing (protocol constant).
/// Camera and prover both use this to split pixels into blocks.
/// Independent of CPU core count — prover groups blocks into shards freely.
pub const BLOCK_SIZE: usize = 65536; // 64KB

/// Compute block hashes for an image's raw pixel data.
pub fn compute_block_hashes(pixels: &[u8]) -> Vec<[u8; 32]> {
    let num_blocks = (pixels.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;
    (0..num_blocks)
        .map(|i| {
            let s = i * BLOCK_SIZE;
            let e = std::cmp::min(s + BLOCK_SIZE, pixels.len());
            Sha256::digest(&pixels[s..e]).into()
        })
        .collect()
}

/// Compute image commitment from block hashes: SHA256(block_hash_0 || block_hash_1 || ...)
pub fn compute_image_commitment(block_hashes: &[[u8; 32]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for h in block_hashes {
        hasher.update(h);
    }
    hasher.finalize().into()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoMetadata {
    pub device_id: String,
    pub timestamp: u64,
    pub width: u32,
    pub height: u32,
    /// SHA256(block_hash_0 || block_hash_1 || ... || block_hash_N)
    /// where each block is BLOCK_SIZE bytes of raw pixels.
    pub image_commitment: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub public_key: Vec<u8>,
    pub root_ca_pubkey: Vec<u8>,
    pub device_cert_r: [u8; 32],
    pub device_cert_s: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignedPhoto {
    pub image_bytes: Vec<u8>,
    pub metadata: PhotoMetadata,
    pub signature: Signature,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EditOperation {
    Crop {
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    AdjustBrightness {
        delta: i16,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditManifest {
    pub operations: Vec<EditOperation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicValues {
    pub original_image_commitment: [u8; 32],
    pub pub_key_hash: [u8; 32],
    pub root_ca_hash: [u8; 32],
    pub edit_types: Vec<String>,
    pub output_image_hash: [u8; 32],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProofPackage {
    pub edited_image: Vec<u8>,
    pub proof: Vec<u8>,
    pub public_values: PublicValues,
}

impl EditOperation {
    pub fn name(&self) -> String {
        match self {
            EditOperation::Crop { .. } => "Crop".to_string(),
            EditOperation::AdjustBrightness { .. } => "AdjustBrightness".to_string(),
        }
    }
}

/// Lightweight pixel processing helper functions (used by both ZKVM and Host)
pub mod pixel_utils {
    pub fn apply_crop(
        pixels: &[u8],
        orig_w: u32,
        _orig_h: u32,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) -> Vec<u8> {
        let mut result = Vec::with_capacity((w * h * 3) as usize);
        for row in y..(y + h) {
            let start = ((row * orig_w + x) * 3) as usize;
            let end = start + (w * 3) as usize;
            result.extend_from_slice(&pixels[start..end]);
        }
        result
    }

    pub fn apply_brightness(pixels: &[u8], delta: i16) -> Vec<u8> {
        pixels
            .iter()
            .map(|&p| {
                let val = p as i16 + delta;
                if val < 0 {
                    0
                } else if val > 255 {
                    255
                } else {
                    val as u8
                }
            })
            .collect()
    }
}

impl PhotoMetadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.device_id.as_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes.extend_from_slice(&self.width.to_le_bytes());
        bytes.extend_from_slice(&self.height.to_le_bytes());
        bytes.extend_from_slice(&self.image_commitment);
        bytes
    }
}
