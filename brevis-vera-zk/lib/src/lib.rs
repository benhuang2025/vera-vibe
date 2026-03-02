use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoMetadata {
    pub device_id: String,
    pub timestamp: u64,
    pub width: u32,
    pub height: u32,
    pub image_hash: [u8; 32],
    pub shards: Vec<[u8; 32]>, // Hashes of each pixel segment
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub public_key: Vec<u8>,
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
    pub pub_key_hash: [u8; 32],
    pub edit_types: Vec<String>,
    pub output_image_hash: [u8; 32],
    pub shard_hashes: Vec<[u8; 32]>, // Hash of each output shard
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
    /// Canonical serialization for ZK signing
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.device_id.as_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes.extend_from_slice(&self.width.to_le_bytes());
        bytes.extend_from_slice(&self.height.to_le_bytes());
        bytes.extend_from_slice(&self.image_hash);
        for shard in &self.shards {
            bytes.extend_from_slice(shard);
        }
        bytes
    }
}
