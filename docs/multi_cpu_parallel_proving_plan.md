# 🚀 Brevis Vera: Multi-CPU Parallel Proving Implementation Plan

该方案旨在通过分布式并行计算和递归聚合技术，使 Brevis Vera 能够处理原尺寸（3MB+）图片，并将端到端证明时间缩短至分钟级甚至更低。

## 1. 核心瓶颈分析
*   **计算开销**: 3MB 像素数据的 SHA-256 哈希（无硬件加速）预计产生 1B+ Cycles。
*   **内存压力**: 单个递归层级或超大 Proof 的内存开销巨大（Plonky3 架构下通常 100GB+）。
*   **机器优势**: 124 核 CPU 提供极高的并行宽度，920GB 内存足以支撑多个并行 Prover 实例。

---

## 2. 架构方案：分片递归证明 (Sharding & Recursive Join)

我们将不再尝试在一个 ZKVM 实例中处理整个 3MB 图片，而是采用 **“分而治之”** 的策略：

### 第一阶段：数据分片 (Sharding)
将 3MB 的原始像素数据切分为 $N$ 个分片（例如 16 或 32 个，每个约 100KB-200KB）。
*   每个分片计算一个局部哈希。
*   `mock-signer` 签名不再针对整图哈希，而是针对一张“分片哈希表”或“Merkle Root”。

### 第二阶段：并行局部证明 (Parallel Leaf Proofs)
利用 **124 核 CPU**，同时启动 $N$ 个 Pico ZKVM 实例：
*   **任务**: 证明第 $i$ 个分片的像素数据与对应的局部哈希一致。
*   **并行度**: 每个核心处理 1-2 个 Pico 实例，总延迟取决于单片处理时间（约 20-30s）。

### 第三阶段：递归聚合 (Recursive Join)
将生成的 $N$ 个局部 Proof 递归聚合：
*   **Join 逻辑**: 一个聚合 ZKVM 实例接收 Proof A 和 Proof B，验证它们并输出一个合并的证明。
*   **最终证明**: 最终得到一个代表“整图已验证且符合原签名”的单一证明。

---

## 3. 详细实施步骤

### Phase 1: `mock-signer` 增强
1.  **分片逻辑**: 修改 `Commands::Sign`，支持将 `image_bytes` 按固定步长切片。
2.  **元数据重构**: 
    *   `PhotoMetadata` 增加 `shards: Vec<[u8; 32]>`。
    *   计算分片哈希的根（Merkle 或简单 Concat Hash）。
3.  **多文件输出**: 输出包含分片数据的扩展 JSON 结构。

### Phase 2: Guest 程序（ZK 逻辑）改造
1.  **轻量级 Guest**: 编写一个新的 `app_shard`，仅执行单个分片的哈希验证：
    *   Input: `pixel_segment`, `expected_segment_hash`.
    *   Logic: `assert_eq!(sha256(pixel_segment), expected_segment_hash)`.
2.  **编辑算子重放**: 若涉及裁剪（Crop），分片逻辑需根据坐标映射到具体分片。

### Phase 3: Prover 并行调度器 (Rust Implementation)
1.  **引入 `rayon`**: 在 `prover` 模块中使用并行迭代器：
    ```rust
    let proofs: Vec<_> = shards.into_par_iter().map(|s| {
        generate_pico_proof(client, s)
    }).collect();
    ```
2.  **内存分治**: 虽然有 920GB 内存，但仍建议限制并发证明数（如同时跑 32 个），防止极致情况下的 Swap 问题。

### Phase 4: 递归聚合实现
1.  **Pico Recursion**: 调用 `PicoProver::compress` 或 `merge` 接口。
2.  **证明链**: 实现一个简单的二叉树聚合算法，直到生成最终的 Root Proof。

---

## 4. 性能预期 (针对 3MB 原图)

| 方案 | 处理模式 | 预计延迟 |
| :--- | :--- | :--- |
| **基础版 (Current)** | 单机串行 Emulate | 10min+ (极易超时/崩溃) |
| **中级版 (Prove)** | 单机 124 核硬件加速 | 3~5min |
| **终极版 (Parallel)** | **16 分片并行 + 递归聚合** | **45s - 90s** |

---

## 5. 后续优化方向
*   **NUMA 亲和性**: 第二版将通过 `numactl` 或线程绑定，锁定 62 核至单一 NUMA 节点，进一步提升缓存命中率。
*   **SHA-256 Precompile**: 若能接入 Pico 的哈希加速指令，时间可进一步压缩到 10s 以内。
