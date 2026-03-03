# Brevis Vera: ZK-Driven Media Authenticity & Traceability

**Brevis Vera** is a prototype system built on Pico ZKVM that combats the "identity crisis" of media in the AIGC era. It allows photographers to prove the authenticity of their work while performing legitimate edits (crop, brightness, contrast, grayscale, rotation). These edits are verified via Zero-Knowledge Proofs (ZKP), ensuring the final media is both **traceable** and **authentic** without exposing sensitive raw data.

---

## Project Structure

- **`brevis-vera-zk/`**: Core workspace containing ZKVM guest programs, prover, and tools.
  - **`lib/`**: Shared data structures, pixel operations, block-based commitment logic
  - **`shard-app/`**: ZK guest program — per-block hashing and per-byte edit operations
  - **`aggregator-app/`**: ZK guest program — ECDSA P-256 signature verification, hard link enforcement, final hash aggregation
  - **`app/`**: Monolithic (non-sharded) ZK guest program for reference
  - **`prover/`**: Host-side orchestrator for parallel AOT proving + STARK proof generation
  - **`mock-signer/`**: CLI tool for keygen, signing, editing, and verifying
  - **`web-server/`**: Axum backend serving APIs and static files
  - **`shard-aot/`** & **`aggregator-aot/`**: AOT-compiled native code for fast emulation
  - **`vm/`**: Pico VM runtime (forked from [brevis-network/pico](https://github.com/brevis-network/pico))
- **`web-app/`**: Frontend (HTML/CSS/JS) for the web interface
- **`images/`**: Source media and raw photographs
- **`test_e2e.sh`**: End-to-end automated test suite

---

## Web App

A full web interface for Upload & Sign, Edit & Prove, and Verify:

```bash
cd brevis-vera-zk
cargo run --release --bin web-server
```

Open **http://localhost:3000** (or **http://\<server-ip\>:3000** for remote access).

---

## Architecture

```
                    Root CA (Manufacturer)
                         │ signs (host-side verified)
                         ▼
                    Device Key (Camera)
                         │ signs (verified inside ZK)
                         ▼
              Photo Metadata + Image Commitment
          (SHA256 of 570 block hashes, 64KB each)
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
     Shard 0         Shard 1    ...  Shard N     ← parallel AOT emulation
   (blocks→hashes) (blocks→hashes) (blocks→hashes)
     + edits         + edits         + edits
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                    Aggregator (ZK)               ← STARK proof generated here
              • Verify ECDSA P-256 signature
              • Hard Link: commitment match
              • Compute output_image_hash
              • Commit root_ca_hash
                         │
                         ▼
                   Proof Package
         (verifiable by anyone, no trust required)
```

### Security Properties (verified inside ZK)
- **Hard Link**: Block-based image commitment must match the signed metadata
- **Identity**: Device key ECDSA P-256 signature is verified inside the ZK circuit
- **Trust Anchor**: Root CA hash is committed so verifiers can check manufacturer identity
- **Edit Integrity**: Output hash is computed from actual edited pixel data
- **Privacy**: Edit parameters (crop coordinates, brightness delta, etc.) stay private

---

## Supported Edit Operations

| Operation | In-ZK Proving | Description |
|-----------|--------------|-------------|
| **Crop** | Host-level | Reduces image dimensions |
| **AdjustBrightness** | ✅ In shard | Adds delta to each RGB channel |
| **AdjustContrast** | ✅ In shard | Scales each channel around midpoint |
| **Grayscale** | Host-level | Converts RGB to grayscale |
| **Rotate90** | Host-level | 90° clockwise rotation |

---

## Quick Start

### Prerequisites

1. **Rust nightly toolchain** (`nightly-2025-08-04`)
   ```bash
   rustup install nightly-2025-08-04
   rustup default nightly-2025-08-04
   ```

2. **Pico CLI** (only needed if modifying guest code)
   ```bash
   cargo install cargo-pico
   ```

### Clone and Run

```bash
git clone https://github.com/benhuang2025/vera-vibe.git
cd vera-vibe

# Run the full end-to-end test (fast, emulation mode)
chmod +x test_e2e.sh
./test_e2e.sh

# Run with real STARK proof generation (slower, cryptographically verifiable)
./test_e2e.sh --real-proof
```

> **Note**: The repository includes all pre-built artifacts (ELF binaries, AOT chunks), so you can run E2E tests immediately after cloning.

---

## Manual Step-by-Step

```bash
cd brevis-vera-zk

# 1. Generate Root CA + Device keys
cargo run --release --bin mock-signer -- keygen --output keys/private_key.pem

# 2. Sign a photo
cargo run --release --bin mock-signer -- sign \
  --image ../images/DSC00056.JPG \
  --key keys/private_key.pem \
  --output signed_photo.json

# 3. Edit (combine multiple ops with semicolons)
cargo run --release --bin mock-signer -- edit \
  --input signed_photo.json \
  --ops "contrast:120;brightness:10" \
  --output edited.png \
  --manifest edit_manifest.json

# Available ops: crop:x,y,w,h  brightness:delta  contrast:factor
#                grayscale      rotate90

# 4. Generate ZK Proof
cd prover
cargo run --release --bin prover -- \
  --photo ../signed_photo.json \
  --manifest ../edit_manifest.json \
  --output ../proof_package.json \
  --edited-image ../edited.png
# Add --real-proof for cryptographic STARK proof
cd ..

# 5. Verify
cargo run --release --bin mock-signer -- verify \
  --package proof_package.json \
  --image edited.png \
  --trusted-pubkey-hash <ROOT_CA_HASH_FROM_STEP_1>
```

---

## Performance

Tested on a 128-vCPU Linux server with a 4320×2880 image (37MB raw pixels, 570 blocks):

| Mode | Shard Time | Aggregator | Total E2E |
|------|-----------|------------|-----------|
| **Emulation** | 340 ms | 10 ms | **~835 ms** |
| **Real STARK** | 340 ms | 31 s | **~32 s** |

Key optimizations:
- **SHA256 Precompile**: Activated via `brevis-network/hashes` fork — all SHA256 operations route through Pico's built-in hardware acceleration
- **Plan D (Block-Based Commitment)**: 64KB fixed blocks decouple camera signing from prover sharding. Aggregator receives 36KB of block hashes instead of 37MB of raw pixels
- **Parallel AOT**: 124 shards across all CPU cores via Rayon

See [PERFORMANCE_BENCHMARK.md](PERFORMANCE_BENCHMARK.md) for detailed benchmarks.

---

## Developer Guide: Rebuilding from Source

If you modify the ZK guest programs, rebuild ELF binaries and regenerate AOT chunks:

```bash
# 1. Build ZK guest ELFs
cd brevis-vera-zk/shard-app && cargo pico build
cd ../aggregator-app && cargo pico build

# 2. Regenerate AOT chunks (requires pico_tools)
cd ..
../pico_tools/target/release/generate_crates \
  shard-app/elf/riscv32im-pico-zkvm-elf shard-aot
../pico_tools/target/release/generate_crates \
  aggregator-app/elf/riscv32im-pico-zkvm-elf aggregator-aot

# 3. Fix AOT naming (workspace requires unique crate names)
# shard-aot chunks: rename pico-aot-chunk-* → shard-aot-chunk-*
# aggregator-aot chunks: rename pico-aot-chunk-* → agg-aot-chunk-*

# 4. Build and test
cargo build --release
cd .. && ./test_e2e.sh
```

---

## Tech Stack
- **ZKVM**: [Pico](https://github.com/brevis-network/pico) with AOT compilation + SHA256 precompile
- **Language**: Rust (guest programs, prover, CLI, web server)
- **Frontend**: HTML/CSS/JS (vanilla, no framework)
- **Cryptography**: ECDSA P-256 (device signature + Root CA cert chain), SHA-256 (precompile-accelerated)
- **Parallelism**: Rayon + per-core sharding (auto-detects CPU count)
- **Web Server**: Axum

---

## Core Philosophy: Proof of Origin
> "Don't trust, verify."
> Brevis Vera proves that we can allow digital creativity without sacrificing truth.
