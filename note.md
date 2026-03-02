In my understanding, the popular development workflow is SDD (Spec-Driven Development), and the tools we are using now are OpenSpec + Claude UI + MiniMax 2.5.
This prototyping will not involve multi-agents doing separate works and then integrating. Instead, we define and iterate on the spec, which gives the AI agent exact instructions on what to do.

Clarification for this tournament: Since our ZKVM implementation currently lacks true cryptographic privacy features, strict privacy is not required. However, we should implement the specific operation parameters as private inputs to demonstrate and convey the architectural intent of privacy-preserving media editing.

关于 Pico ZKVM 处理 C2PA 大图哈希性能问题的反馈与方案建议
1. 现状痛点 (Echoing Elliot & Jie Tang): 在 ZKVM (RISC-V) 内部对 3MB+ 的原图进行全量 SHA-256 哈希确实会导致处理周期（Cycles）爆炸。在 Pico 没有 SHA-256 Precompile（硬件加速）的情况下，单机模拟确实很难跑出来。

2. 我们的避坑方案: Development Scaling (开发环境缩放协议) 为了确保端到端流程能跑通且可演示，建议采用以下“分层验证”策略：

实施方法：我们在 mock-signer 签名阶段，并没有直接哈希全量原始像素，而是先将图片降采样至 256x256 (RGB8)，然后再进行 P-256 签名。
计算量对比：
3MB 原图：哈希开销极大，可能触发 OOM 或超长等待。
256x256 图片：数据量约 192KB。在 Pico 里的总周期约为 24 Million Cycles。这个量级在普通笔记本 CPU 上用 emulate 模式可以在秒级跑完，逻辑验证非常丝滑。
安全合规性建议：在实际 C2PA 行业标准中，相机硬件本身也经常对内容的低分辨率哈希 (Lower-res hash) 或 预览图断言 (Thumbnail Assertion) 进行签名。因此，在 ZK 证明中验证一个“采样后的签名图”，在逻辑完备性上与原图是一致的。
3. 对生产部署的思考 (Path to Production):

Demo 阶段：坚持使用 256x256 或 512x512 的采样比例。这能保证我们的端到端闭环（签名验证 + 像素编辑重放 + 输出哈希比对）是 0 Mock 运行的。
生产阶段：一旦上线 GPU Server 或 Pico 更新了 SHA-256 Precompile，我们只需将 mock-signer 里的 thumbnail 缩放比例改回原图尺寸即可，底层的 ZK 电路逻辑完全不需要改动。
总结建议： 建议大家先把“像素粒度”降下来，跑通 “签名自洽 + 编辑重放” 的完整逻辑。性能是个工程问题（靠 GPU/加速），但逻辑闭环才是我们 ZK-C2PA 项目的核心竞争力。这样大家就都能在本机跑出 Proof 了。