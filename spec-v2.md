# Brevis Vera Spec v2: ZKVM 深度集成与优化

在完成 Step 4 Spike 后，我们确定了核心逻辑（P-256 签名）在 Pico ZKVM 中是可行的。基于此，我们细化 Spec 如下。

## 1. 像素处理逻辑 (Pixel Logic in ZKVM)

由于 `image` crate 的复杂性，ZKVM 内部将采用精简的像素处理方法：
- **格式**: 图片输入为 `Vec<u8>` (RGB8/RGBA8 裸像素数据)。
- **操作**:
  - `Crop`: 直接从像素数组中计算偏移量并提取子数组。
  - `Brightness`: 对每个像素分量叠加 `delta`，并进行 `0-255` 的截断处理。
- **一致性**: Host 端（Editor CLI）必须使用完全相同的算法，以确保最终计算的 `output_image_hash` 一致。

## 2. 隐私性增强 (Privacy vs Public)

- **Private Witness**: 
  - `SignedPhoto` (包括原始像素、元数据、签名)。
  - `EditManifest.operations` 的具体参数（如 `x`, `y`, `delta`）。
- **Public Commit**:
  - `PhotoMetadata.image_hash` 的验证结果。
  - `Signature.public_key` 的哈希。
  - `EditManifest.operations` 的**动作名称**列表（类型）。
  - 编辑后图片的 `sha256_hash`。

## 3. ZKVM 代码结构 (app/src/main.rs)

```rust
fn main() {
    let signed_photo: SignedPhoto = read_as();
    let manifest: EditManifest = read_as();

    // 1. 验证签名 (Original Authenticity)
    verify_p256(signed_photo.public_key, signed_photo.metadata, signed_photo.signature);

    // 2. 验证图片与元数据对应 (Integrity)
    assert_eq!(sha256(signed_photo.image_bytes), signed_photo.metadata.image_hash);

    // 3. 执行编辑 (Replay)
    let mut current_pixels = signed_photo.image_bytes;
    let mut edit_types = vec![];
    for op in manifest.operations {
        edit_types.push(op.name());
        current_pixels = match op {
            EditOperation::Crop { .. } => apply_crop(current_pixels, ...),
            EditOperation::AdjustBrightness { .. } => apply_brightness(current_pixels, ...),
        };
    }

    // 4. 提交公开信息
    commit(PublicValues {
        pub_key_hash: sha256(signed_photo.public_key),
        edit_types,
        output_image_hash: sha256(current_pixels),
    });
}
```

## 4. 性能考量 (Performance)

- **图片缩放**: 为了保证原型流畅，建议输入的原始图片不宜超过 512x512。
- **操作限制**: 暂定最多支持 5 个连续操作。

---

## 🚀 Step 5 验证

- [x] SPIKE 证明了 P-256 签名开销约 12M cycles。
- [ ] 确保 `lib` 库中的 `EditOperation` 支持在 `no_std` 下执行。
