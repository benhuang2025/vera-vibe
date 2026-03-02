# Brevis Vera Spec v1: 数据结构与核心接口

## 1. 核心模型 (Core Models)

所有数据结构应支持 Serde 序列化，以便在模块间传输。

### 1.1 PhotoMetadata
记录照片的原始元数据。
```rust
struct PhotoMetadata {
    device_id: String,      // 模拟设备唯一标识
    timestamp: u64,         // 拍摄时间戳
    width: u32,             // 原始宽度
    height: u32,            // 原始高度
    image_hash: [u8; 32],   // 原始图片的 SHA-256 哈希
}
```

### 1.2 Signature
存储 ECDSA P-256 签名信息。
```rust
struct Signature {
    r: [u8; 32],            // 签名分量 r
    s: [u8; 32],            // 签名分量 s
    public_key: Vec<u8>,    // 对应公钥 (SEC1 格式)
}
```

### 1.3 SignedPhoto
完整的原始签名包。
```rust
struct SignedPhoto {
    image_bytes: Vec<u8>,
    metadata: PhotoMetadata,
    signature: Signature,
}
```

### 1.4 EditOperation
定义的编辑操作枚举。
```rust
enum EditOperation {
    Crop { x: u32, y: u32, width: u32, height: u32 },
    AdjustBrightness { delta: i16 }, // -255 到 255
}
```

### 1.5 EditManifest
一个有序的编辑指令集。
```rust
struct EditManifest {
    operations: Vec<EditOperation>,
}
```

### 1.6 ProofPackage
发送给验证者的最终产出物。
```rust
struct ProofPackage {
    edited_image: Vec<u8>,
    proof: Vec<u8>,          // Pico ZKVM 生成的证明
    public_values: PublicValues,
}

struct PublicValues {
    pub_key_hash: [u8; 32],  // 证明中使用的公钥哈希
    edit_types: Vec<String>, // 隐藏参数后的操作类型列表
    output_image_hash: [u8; 32], // 编辑后图片的哈希
}
```

---

## 2. 模块接口 (Module Interfaces)

### 2.1 Mock Signer (Capture Layer)
负责模拟相机拍摄并签名的过程。
- `fn generate_keypair() -> (SecretKey, PublicKey)`
- `fn sign(image: &[u8], device_id: &str) -> SignedPhoto`
- `fn verify_signed_photo(signed: &SignedPhoto) -> bool`

### 2.2 Editor (Editing Layer)
负责在 UI 侧或本地执行编辑。
- `fn apply_manifest(image: &[u8], manifest: &EditManifest) -> Vec<u8>`
- `fn crop(image: &DynamicImage, x, y, w, h) -> DynamicImage`
- `fn adjust_brightness(image: &DynamicImage, delta: i16) -> DynamicImage`

### 2.3 ZK Guest Program (app/)
这是运行在 Pico ZKVM 内的代码。
- **Inputs (Private)**: `SignedPhoto`, `EditManifest`
- **Logic**:
    1. 计算 `SignedPhoto.image_bytes` 的哈希，对比 `metadata.image_hash`。
    2. 使用 `metadata.image_hash` 和 `signature` 进行 ECDSA P-256 验证。
    3. 依次执行 `EditManifest` 中的操作。
    4. 计算最终像素的哈希。
- **Commit (Public)**: `PublicValues` (公钥、编辑类型、结果哈希)。

### 2.4 Prover
主机端程序，负责启动 ZKVM。
- `fn generate_proof(signed: SignedPhoto, manifest: EditManifest) -> ProofPackage`

### 2.5 Verifier
独立验证端。
- `fn verify(pkg: ProofPackage, trusted_keys: &[PublicKeyHash]) -> bool`

---

## 3. 存储与传输格式

- **Mock 数据**: 存储为 `.json` 文件。
- **中间图片**: 原始照片建议使用 `DSC00050.JPG`，编辑后输出为 `.png` 以避免损失压缩导致的哈希不一致。
- **证明**: 序列化为二进制 `.bin`。

---

## 4. 🚀 Step 2 测试与验证 (Test)

- [ ] **结构检查**: 确保 `lib` 库中能够编译这些结构体定义（Step 3 将会完成此操作）。
- [ ] **零知识性校验**: 检查 `PublicValues` 中是否确实不包含 `Crop` 的具体坐标。
