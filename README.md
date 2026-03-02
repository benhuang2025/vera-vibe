# 🛡️ Brevis Vera : ZK 驱动的媒体真伪与溯源系统

**Brevis Vera** 是一个基于 Pico ZKVM 的原型系统，旨在解决 AIGC 时代的媒体信任危机。它允许摄影师在保留原始签名证明的同时，通过零知识证明（ZK）展示其对照片的合法编辑（如裁剪、亮度调节），确保媒体的**可追溯性**与**真实性**。

---

## 📂 项目结构

- **`brevis-vera-zk/`**: 核心工作区，包含 ZKVM Guest 程序、Prover 服务和本地签名工具。
- **`web-verifier.html`**: 极简但强大的前端验证器，支持浏览器内实时哈希校验。
- **`test_e2e.sh`**: 完整的端到端自动化测试套件，一键模拟全生命周期。
- **`spec-v2.md`**: 详细的技术设计规范与架构逻辑。

---

## 🚀 快速上手：三步体验 Brevis Vera

### 第一步：运行全链路测试
在终端执行自动化脚本，它将模拟从拍摄、签名到生成 ZK 证明的全过程。

```bash
chmod +x test_e2e.sh
./test_e2e.sh
```

**运行结果：**
- 脚本会生成 `private_key_test.pem`（模拟相机私钥）。
- 生成 `signed_test.json`（原始带签名照片）。
- 生成 `edited_test.png`（经过裁剪和调亮的编辑后图片）。
- 生成 `proof_package_test.json`（由 Pico ZKVM 生成的零知识证明包）。

---

### 第二步：体验前端验证 (Web UI)
无需任何代码环境，直接使用我们为你准备的 Web 界面进行验证：

1.  在浏览器中打开根目录下的 **`web-verifier.html`** 文件。
2.  将第一步生成的 **`proof_package_test.json`** 拖入左侧 JSON 框。
3.  将第一步生成的 **`edited_test.png`** 拖入右侧图片框。
4.  点击 **“验证媒体真实性”**。

**你将看到：** 一场极具科技感的 3D 扫描动画，并最终展示验证通过的报告及编辑历史。

---

### 第三步：尝试“恶意篡改” (Negative Test)
你可以测试系统识别虚假图像的能力：

1.  使用任何修图软件稍微修改一下 `edited_test.png`（哪怕只改一个像素）。
2.  再次拖入 `web-verifier.html` 进行验证。
3.  **结果：** 验证器会立即弹出 `❌ Image Hash MISMATCH!`，拒绝这份伪造的媒介。

---

## 🛠 技术栈
- **ZKVM**: [Pico](https://github.com/brevis-network/pico) (v1.3.0)
- **Language**: Rust (Stable/Nightly)
- **Cryptography**: ECDSA P-256, SHA-256
- **Frontend**: Vanilla JS (Subtle Crypto API)

---

## 📜 核心理念：Proof of Origin
> "Don't trust, verify." 
> Brevis Vera 证明了：我们可以既允许修图，又不牺牲真实。
