# Brevis Vera — 开发计划 (SDD 方法论)

## 方法论

采用 Spec-Driven Development：
1. 先写 Spec → 2. AI 执行 → 3. 测试验证 → 4. 迭代 Spec → 循环

**不搞多 agent 并行**。单一 agent 保持上下文一致性，按 spec 串行推进。

---

## 总览

```
Step 1: 环境搭建 + Spike 验证
Step 2: 写 Spec v1（数据结构 + 接口）
Step 3: 实现 Mock 签名 + 图片编辑
Step 4: Spike ZK 电路可行性
Step 5: 迭代 Spec v2（根据 spike 结果修正）
Step 6: 实现 ZK 电路 + Prover
Step 7: 实现 Verifier CLI
Step 8: 端到端集成测试
Step 9: (Bonus) Web UI
```

---

## Step 1: 环境搭建 + Spike 验证 ⏱️ ~1-2h

**目的：** 确认工具链能用，摸清 Pico ZKVM 的能力边界

### 任务
- [ ] 安装 Rust nightly-2025-08-04
- [ ] 安装 Pico CLI
- [ ] 用 `cargo pico new --template basic` 创建测试项目
- [ ] 跑通 Fibonacci 示例：build → prove → verify 全流程
- [ ] 阅读 Pico examples 目录，了解 IO、commit 的 API 用法

### Spike 问题（需要回答）
1. Pico ZKVM 的程序 IO 怎么传入（stdin? 函数参数?）
2. 公开输出 (public values) 怎么 commit？
3. 证明文件格式是什么？怎么做独立验证？
4. `no_std` 环境下哪些 crate 能用？

### 产出
- spike_report.md：记录上述问题的答案

### 🚀 测试与验证 (Test)
- [ ] **执行命令**: `cd Fibonacci/app && cargo pico build && cd ../prover && cargo run --release`
- [ ] **断言**: 终端输出显示 `Verification succeeded`。
- [ ] **二进制检查**: 确保 `app/elf` 目录下生成了最新的 ELF 文件。

---

## Step 2: 写 Spec v1 ⏱️ ~2-3h

**目的：** 定义所有数据结构和模块接口，让后续 AI 实现有精确指令

### 需要定义的内容

#### 2.1 数据结构
```
- PhotoMetadata: 设备ID、拍摄时间、图片尺寸、图片SHA-256哈希
- Signature: ECDSA P-256 签名值 (r, s) + 公钥
- SignedPhoto: 原始图片字节 + PhotoMetadata + Signature
- EditOperation: 枚举 (Crop{x,y,w,h}, Brightness{delta}, ...)
- EditManifest: Vec<EditOperation> — 编辑操作序列
- ProofPackage: 编辑后图片 + ZK Proof + 公开输出
```

#### 2.2 模块接口
```
mock-signer:
  - generate_keypair() -> (PrivateKey, PublicKey)
  - sign_photo(photo_bytes, metadata, private_key) -> SignedPhoto
  - verify_signature(signed_photo, public_key) -> bool

editor:
  - apply_edits(image_bytes, edit_manifest) -> edited_bytes
  - (内部) crop(image, x, y, w, h) -> image
  - (内部) adjust_brightness(image, delta) -> image

zk-circuit (app/):
  - 输入: SignedPhoto + EditManifest
  - 输出 (public): 公钥哈希 + 编辑类型列表 + 编辑后图片哈希
  - 验证逻辑: 签名合法 + 编辑结果匹配 + 无额外修改

prover:
  - prove(signed_photo, edit_manifest) -> ProofPackage

verifier:
  - verify(proof_package) -> AuthenticityVerdict
```

#### 2.3 文件格式
- 签名数据：JSON
- 证明文件：Pico 原生格式 + 自定义 metadata JSON
- 编辑后图片：PNG/JPEG

### 产出
- spec-v1.md：完整的 spec 文档

### 🚀 测试与验证 (Test)
- [ ] **一致性检查**: 确保 `SignedPhoto` 的定义能够涵盖 `PhotoMetadata`、`Signature` 和 `ImageBytes`。
- [ ] **AI 交叉评审**: 让 AI agent 模拟三个层的角色（Signer, Editor, ZKVM），检查数据流动是否存在悖论或缺失字段。

---

## Step 3: 实现 Mock 签名 + 图片编辑 ⏱️ ~3-4h

**目的：** 先做不涉及 ZK 的两个模块，验证数据流

### 3.1 Mock Signer
- [ ] 实现 ECDSA P-256 密钥生成 (用 `p256` crate)
- [ ] 实现 sign_photo：SHA-256(metadata) → ECDSA 签名
- [ ] 实现 verify_signature
- [ ] CLI: `brevis-vera sign --image DSC00050.JPG --output signed_photo.json`
- [ ] 测试：签名 → 验证通过；篡改 → 验证失败

### 3.2 Image Editor
- [ ] 实现 Crop（必须）
- [ ] 实现第二种变换（建议：灰度/亮度，选简单的，ZKVM 里好复现）
- [ ] CLI: `brevis-vera edit --input signed_photo.json --ops "crop:10,10,200,200" --output edited.png`
- [ ] 测试：编辑前后图片对比正确

### 关键原则
- **编辑操作要足够简单**，因为同样的逻辑要在 ZKVM 里重新实现一遍
- 建议选择像素级操作（而非依赖复杂库的滤镜），这样 ZKVM 内手写比较容易

### 产出
- 可工作的 mock-signer 和 editor 模块
- 端到端测试：签名 → 编辑 → 输出都 OK

### 🚀 测试与验证 (Test)
- [ ] **单元测试 (Unit Tests)**: 
  - `test_signature_roundtrip`: 签名后立即验证，必须通过。
  - `test_signature_tamper`: 修改 Metadata 中的一个字节，验证必须失败。
  - `test_crop_dimensions`: 输入 100x100 图片裁剪 10x10，输出尺寸必须精准。
- [ ] **端到端脚本**: 
  ```bash
  ./target/debug/brevis-vera sign test.jpg signed.json
  ./target/debug/brevis-vera edit signed.json "crop:0,0,10,10" edited.png
  ```
  手动检查 `edited.png` 是否符合预期。

---

## Step 4: Spike ZK 电路可行性 ⏱️ ~2-3h

**目的：** 在真正写大电路前，先验证几个关键技术点

### 需要验证的问题
- [ ] **Spike A:** 在 Pico ZKVM 内能跑 SHA-256 吗？性能如何？
- [ ] **Spike B:** 在 Pico ZKVM 内能跑 ECDSA P-256 验证吗？用什么库？
  - 试试 `p256` crate 能否在 RISC-V 目标编译
  - 如果不行，备选方案是什么？
- [ ] **Spike C:** 传入一张小图片（比如 32x32），在 ZKVM 内做 crop，需要多久？
- [ ] **Spike D:** 如何把 Vec<u8>（图片字节）传入 ZKVM？IO 大小有限制吗？

### 产出
- spike_zk_report.md：每个问题的结论 + 对 spec 的修正建议

### 🚀 测试与验证 (Test)
- [ ] **性能基准**: 记录 32x32 像素图片在 ZKVM 内运行 SHA-256 的耗时。
- [ ] **兼容性验证**: 如果 `p256` 无法在 RISC-V 目标下编译，立即执行 `cargo check --target riscv32im-pico-zkvm-elf`。

---

## Step 5: 迭代 Spec v2 ⏱️ ~1-2h

**目的：** 根据 Step 4 的 spike 结果修正 spec

### 可能的修正
- 如果 P-256 太慢 → 改用 Ed25519 或哈希签名
- 如果大图片不可行 → spec 中限制图片大小
- 如果某些 crate 不兼容 → spec 中明确手写哪些算法
- 调整 ZK 电路的公开输入/输出定义

### 产出
- spec-v2.md：修正后的可执行 spec

### 🚀 测试与验证 (Test)
- [ ] **追溯检查**: 检查每个 Step 4 中发现的 Limitation 是否都在 Spec v2 中得到了规避或明确说明。

---

## Step 6: 实现 ZK 电路 + Prover ⏱️ ~6-10h

**目的：** 核心实现，按 spec-v2 执行

### 6.1 ZK Circuit (app/)
按照 spec-v2 实现：
- [ ] 读取输入（原始图片 + 签名 + 编辑操作）
- [ ] ZKVM 内验证签名
- [ ] ZKVM 内执行编辑操作
- [ ] 计算编辑后图片哈希
- [ ] commit 公开输出

### 6.2 Prover
- [ ] 准备输入数据（序列化 SignedPhoto + EditManifest）
- [ ] 调用 Pico SDK 执行 + 生成证明
- [ ] 保存 ProofPackage
- [ ] CLI: `brevis-vera prove --signed-photo signed.json --ops "crop:10,10,200,200" --output proof_pkg/`

### 迭代策略
1. 先用最小输入（4x4 像素 + 假签名）跑通
2. 再换真实签名
3. 再换更大图片
4. 最后加第二种编辑操作

### 产出
- 可工作的 ZK 电路 + Prover

### 🚀 测试与验证 (Test)
- [ ] **ZKVM 内部断言**: 在电路代码中使用 `assert!` 检查中间计算结果（如 crop 后的像素点）。
- [ ] **生成证明测试**:
  ```bash
  cargo run --bin prover -- --input signed.json --ops "crop:..." --output test_proof.bin
  ```
  确认 `test_proof.bin` 存在且非空。

---

## Step 7: 实现 Verifier CLI ⏱️ ~2-3h

- [ ] 加载 ProofPackage
- [ ] 调用 Pico SDK 验证证明
- [ ] 检查公开输出（图片哈希匹配、公钥合法、编辑类型合理）
- [ ] 输出 AuthenticityVerdict
- [ ] CLI: `brevis-vera verify --proof-pkg proof_pkg/ --image edited.png`

### 产出
- 可工作的 Verifier CLI

### 🚀 测试与验证 (Test)
- [ ] **正向验证**: 传入正确的编辑后图片和 Proof，必须返回 `Authenticity Verified`。
- [ ] **篡改验证 (Tamper Test)**:
  - 即使 Proof 正确，如果我用 PS 改了 `edited.png` 的一个像素，Verifier 必须检测出哈希不匹配并返回 `REJECT`。
  - 修改 `EditedManifest`（例如把 crop 范围改了），验证必须返回 `REJECT`。

---

## Step 8: 端到端集成测试 ⏱️ ~1-2h

跑通完整流程：
```bash
# 1. 签名
brevis-vera sign --image DSC00050.JPG --output signed.json

# 2. 编辑
brevis-vera edit --input signed.json --ops "crop:100,100,500,500;brightness:20" --output edited.png

# 3. 生成证明
brevis-vera prove --signed-photo signed.json --ops "crop:100,100,500,500;brightness:20" --output proof_pkg/

# 4. 验证
brevis-vera verify --proof-pkg proof_pkg/ --image edited.png
# ✅ Authenticity Verified!
```

- [ ] Happy path 通过
- [ ] 篡改图片 → 验证失败
- [ ] 篡改证明 → 验证失败
- [ ] 用 DSC00050.JPG 演示一次完整流程

### 🚀 测试与验证 (Test)
- [ ] **自动化脚本**: 编写一个 `e2e_test.sh`，运行全流程：
  ```bash
  ./scripts/e2e_test.sh
  # 内部流程: Sign -> Edit -> Prove -> Verify
  # 如果任何一步退出码不为 0 或 Verify 结果不为 Verified，则脚本失败
  ```

---

## Step 9: (Bonus) Web UI ⏱️ ~4-6h

- [ ] 简单的 Web 前端：上传 → 编辑 → 证明 → 验证
- [ ] 后端 API (Axum/Actix)
- [ ] 可视化展示证明结果

### 🚀 测试与验证 (Test)
- [ ] **手动集成测试**: 浏览器上传 -> 实时显示预览 -> 生成证明（后端） -> 页面显示“绿色打钩”。

---

## 时间总览

| Step | 内容 | 时间 | 类型 |
|------|------|------|------|
| 1 | 环境搭建 + Spike | 1-2h | 探索 |
| 2 | Spec v1 | 2-3h | **Spec** |
| 3 | Mock 签名 + 编辑 | 3-4h | 实现 |
| 4 | ZK Spike | 2-3h | 探索 |
| 5 | Spec v2 | 1-2h | **Spec** |
| 6 | ZK 电路 + Prover | 6-10h | 实现 |
| 7 | Verifier | 2-3h | 实现 |
| 8 | 集成测试 | 1-2h | 测试 |
| 9 | Web UI (Bonus) | 4-6h | 实现 |
| **合计** | | **18-29h** | |

> Spec 写作占 ~20% 时间，但决定了 ~80% 的执行质量。这就是 SDD。
