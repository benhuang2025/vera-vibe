# Brevis Vera — Performance Benchmark

## Test Environment

| Parameter | Value |
|-----------|-------|
| CPU | 64-core (128 vCPU) |
| RAM | 256 GB |
| OS | Linux 5.15.0-113-generic |
| Rust | nightly-2025-08-04 |
| Pico ZKVM | v1.2.2 (KoalaBear field) |
| Execution Mode | AOT (Ahead-of-Time compiled guest programs) |

## Test Image

| Parameter | Value |
|-----------|-------|
| Source | Sony A7IV (C2PA-signed) |
| Resolution | 4320 × 2880 |
| Color | RGB8 (3 bytes/pixel) |
| Raw pixel size | 37,324,800 bytes (~37 MB) |
| Block count | 570 blocks (64KB each) |

---

## Optimizations Implemented

### 1. Parallel Sharding (AOT)

The 37MB raw pixel data is split into 124 shards (one per CPU core), each running as an independent AOT-compiled ZKVM program via Rayon. This turns a prohibitively long single-threaded computation into a sub-second parallel one.

### 2. Block-Based Commitment (Plan D)

The image is divided into fixed-size 64KB blocks. Each block is hashed with SHA256. The camera signs `SHA256(block_hash_0 || ... || block_hash_569)` as the image commitment. This decouples the camera signing step from the prover's shard count, and more importantly — the Aggregator only receives **36KB of block hashes** instead of **37MB of raw pixels**, eliminating the I/O deserialization bottleneck that previously caused Real STARK proof generation to hang.

### 3. SHA256 Precompile (Plan A, Phase 1)

Replaced the standard `sha2` crate with the `brevis-network/hashes` fork, which auto-detects the Pico ZKVM environment and routes SHA256 operations through hardware-accelerated syscalls. All guest programs benefit automatically.

| Metric | Before | After | Change |
|--------|--------|-------|--------|
| Aggregator Instructions | ~16M | ~13.7M | **-14%** |
| Emulation E2E | ~800 ms | ~560 ms | **-30%** |

### 4. P-256 Precompile Attempt (Plan A, Phase 2)

Implemented ECDSA P-256 verification via `Secp256r1Point` precompile. Emulation mode showed a 23% instruction reduction in the Aggregator, but Real STARK mode hit a VM constraint bug (division-by-zero in EC point doubling). Reverted to software `p256` crate — awaiting Pico VM fix. Once resolved, the Aggregator's ~10M ECDSA instructions would drop to ~100K.

---

## Current Performance

### Emulation Mode (fast iteration, no cryptographic proof)

| Metric | Brightness Only | Contrast + Brightness |
|--------|----------------|----------------------|
| Total Shard Instructions | 1,532,908,960 | 2,172,285,550 |
| Longest Shard | 13,458,197 insns | 19,212,465 insns |
| Parallel Shard Time (124 cores) | 337 ms | 379 ms |
| Aggregator Instructions | 13,753,103 | 13,753,103 |
| **Total E2E** | **~835 ms** | **~848 ms** |

### Real STARK Proof Mode (cryptographically verifiable)

| Metric | Value |
|--------|-------|
| Shard Emulation (124 cores) | ~340 ms |
| STARK Proof Generation (Aggregator) | ~31 s |
| STARK Proof Verification | < 2 s |
| **Total E2E** | **~32 s** |
| Proof Size (stark_proof.bin) | ~50 MB |

---

## Supported Edit Operations

| Operation | In-ZK Proved | Description |
|-----------|-------------|-------------|
| Crop | Host-level | Reduces image dimensions |
| AdjustBrightness | ✅ In shard | Adds delta to each RGB channel |
| AdjustContrast | ✅ In shard | Scales each channel around midpoint |
| Grayscale | Host-level | Converts RGB to grayscale (needs pixel alignment) |
| Rotate90 | Host-level | 90° clockwise rotation (needs full image context) |

"Host-level" = applied before ZK proving, demonstrated in editing layer.
"In shard" = proved inside the ZKVM with cryptographic guarantees.

---

## Scalability Notes

- **Shard count scales with CPU cores**: 124 shards on 128 vCPU, each independently provable.
- **Block size is a protocol constant** (64KB): Independent of hardware, ensuring reproducibility across different provers.
- **Adding new per-byte operations**: Only requires a match arm in shard-app — no architectural changes needed.
