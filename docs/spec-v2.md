# Brevis Vera Spec v2: ZKVM Integration & Optimization

Following the Step 4 Spike, we have confirmed that the core logic (P-256 signature verification) is feasible within the Pico ZKVM. Based on these findings, we refine the specifications as follows:

## 1. Pixel Logic in ZKVM

Due to the complexity of the full `image` crate, the ZKVM guest will use a streamlined pixel processing approach:
- **Format**: Image input is `Vec<u8>` (Raw RGB8/RGBA8 pixel data).
- **Operations**:
  - `Crop`: Calculate offsets and extract sub-arrays directly from the pixel buffer.
  - `Brightness`: Apply a `delta` to each pixel component with saturation clipping (0-255).
- **Consistency**: The Host side (Editor CLI) must use the exact same algorithm to ensure the computed `output_image_hash` matches perfectly.

## 2. Privacy Enhancement (Private vs. Public)

- **Private Witness**: 
  - `SignedPhoto` (Including original pixels, metadata, and signature).
  - Specific parameters for `EditManifest.operations` (e.g., coordinates `x`, `y`, and brightness value `delta`).
- **Public Commitments**:
  - Verification result of `PhotoMetadata.image_hash`.
  - Hash of the `Signature.public_key`.
  - A list of **Operation Names** (Types) performed from `EditManifest.operations`.
  - The `sha256_hash` of the final edited image.

## 3. ZKVM Guest Structure (app/src/main.rs)

```rust
fn main() {
    let signed_photo: SignedPhoto = read_as();
    let manifest: EditManifest = read_as();

    // 1. Verify Original Authenticity
    verify_p256(signed_photo.public_key, signed_photo.metadata, signed_photo.signature);

    // 2. Verify Data Integrity
    assert_eq!(sha256(signed_photo.image_bytes), signed_photo.metadata.image_hash);

    // 3. Replay Edits (Deterministic Execution)
    let mut current_pixels = signed_photo.image_bytes;
    let mut edit_types = vec![];
    for op in manifest.operations {
        edit_types.push(op.name());
        current_pixels = match op {
            EditOperation::Crop { .. } => apply_crop(current_pixels, ...),
            EditOperation::AdjustBrightness { .. } => apply_brightness(current_pixels, ...),
        };
    }

    // 4. Commit Public Values
    commit(PublicValues {
        pub_key_hash: sha256(signed_photo.public_key),
        edit_types,
        output_image_hash: sha256(current_pixels),
    });
}
```

## 4. Performance Considerations

- **Image Scaling**: To ensure smooth prototype performance, input images should ideally be scaled to 512x512 or smaller (Current Dev Scale: 256x256).
- **Operation Limit**: Initially restricted to a maximum of 5 consecutive edit operations.

---

## 🚀 Step 5 Validation

- [x] SPIKE confirmed P-256 signature cost is approximately 12M cycles.
- [x] Ensure `lib` crate `EditOperation` supports `no_std` execution for ZKVM compatibility.
