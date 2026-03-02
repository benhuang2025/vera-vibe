# 🛡️ Brevis Vera : ZK-Driven Media Authenticity & Traceability

**Brevis Vera** is a prototype system built on Pico ZKVM, designed to combat the "identity crisis" of media in the AIGC era. It allows photographers to prove the authenticity of their work while maintaining the ability to perform legitimate edits (e.g., cropping, brightness adjustment). These edits are verified via Zero-Knowledge Proofs (ZKP), ensuring that the final media is both **traceable** and **authentic** without exposing sensitive raw data.

---

## 📂 Project Structure

- **`brevis-vera-zk/`**: Core workspace containing ZKVM Guest programs, Prover service, and local signing tools.
  - **`lib/`**: Shared data structures (`PublicValues`, `Signature`, `ProofPackage`, etc.)
  - **`shard-app/`**: ZK guest program for per-shard pixel hashing and edit operations
  - **`aggregator-app/`**: ZK guest program for ECDSA signature verification, Hard Link enforcement, and final hash aggregation
  - **`prover/`**: Host-side orchestrator for parallel AOT proving
  - **`mock-signer/`**: CLI tool for keygen, signing, editing, and verifying
  - **`shard-aot/`** & **`aggregator-aot/`**: AOT-compiled native code for fast emulation
- **`images/`**: Directory for source media and raw photographs.
- **`test_e2e.sh`**: Complete end-to-end automated test suite.

---

## 🏗️ Architecture

```
                    Root CA (Manufacturer)
                         │ signs
                         ▼
                    Device Key (Camera)
                         │ signs
                         ▼
                    Photo Metadata
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
     Shard 0         Shard 1    ...  Shard N
   (pixels→hash)  (pixels→hash)   (pixels→hash)
          │              │              │
          └──────────────┼──────────────┘
                         ▼
                    Aggregator (ZK)
              • Verify ECDSA signature
              • Hard Link: orig hashes = signed hashes
              • Compute output_image_hash
                         │
                         ▼
                   Proof Package
            (original_hash, output_hash,
             root_ca_hash, pub_key_hash)
```

### Security Properties (verified inside ZK)
- **Hard Link**: Each shard's original pixel hash must match the signed metadata
- **Identity**: Device key signature is verified inside the ZK circuit
- **Trust Anchor**: Root CA hash is committed so verifiers can check manufacturer identity
- **Edit Integrity**: Output hash is computed from actual edited pixel data

---

## 🚀 Quick Start

### Prerequisites
- Rust (nightly toolchain `nightly-2025-08-04`)
- [Pico CLI](https://github.com/brevis-network/pico): `cargo install cargo-pico`

### Run the End-to-End Test
```bash
chmod +x test_e2e.sh
./test_e2e.sh
```

**What happens:**
1. **Keygen** — Generates a Root CA key + Device key pair, outputs `Trusted Root CA Hash`
2. **Sign** — Signs a photo with the device key, auto-detects CPU cores for shard count
3. **Edit** — Applies edit operations (or none) and produces `edited_test.png`
4. **ZK Prove** — Runs parallel AOT proving across all CPU cores (~750ms for 37MB image)
5. **Verify (3 cases)**:
   - ✅ Positive: correct image + correct Root CA hash → `VERIFICATION SUCCESSFUL`
   - ✅ Negative: tampered image → `Image Hash MISMATCH!`
   - ✅ Negative: wrong manufacturer key → `Signer is UNTRUSTED!`

### Manual Step-by-Step

```bash
cd brevis-vera-zk

# 1. Generate Root CA + Device keys
cargo run --bin mock-signer -- keygen --output private_key_test.pem
# Outputs: Trusted Root CA Hash (save this for verification)

# 2. Sign a photo
cargo run --bin mock-signer -- sign \
  --image ../images/DSC00056.JPG \
  --key private_key_test.pem \
  --output ../signed_test.json

# 3. Edit (or pass empty ops for no edits)
cargo run --bin mock-signer -- edit \
  --input ../signed_test.json \
  --ops "" \
  --output ../edited_test.png \
  --manifest ../edit_manifest_test.json

# 4. Generate ZK Proof (parallel AOT)
cd prover
cargo run --release --bin prover -- \
  --photo ../../signed_test.json \
  --manifest ../../edit_manifest_test.json \
  --output ../../proof_package_test.json \
  --edited-image ../../edited_test.png
cd ..

# 5. Verify
cargo run --bin mock-signer -- verify \
  --package ../proof_package_test.json \
  --image ../edited_test.png \
  --trusted-pubkey-hash <ROOT_CA_HASH_FROM_STEP_1>
```

---

## 🖼️ Use Your Own Photos

1. Place your photo (JPG or PNG) into the **`images/`** directory.
2. Open **`test_e2e.sh`** and update the `SOURCE_IMAGE` variable to your filename.
3. Run `./test_e2e.sh`.

---

## ⚡ Performance

Tested on a 124-core server with a 4320×2880 (37MB raw) photo:

| Stage | Time | Instructions |
|---|---|---|
| Parallel Shards (124 cores) | ~700ms | 5.27B total |
| Aggregator (ECDSA + Hard Link) | ~50ms | 12.6M |
| **Total E2E** | **~750ms** | — |

The prover auto-detects CPU cores and creates one shard per core for maximum parallelism. On a Mac with fewer cores, it will use all available cores automatically.

---

## 🛠 Tech Stack
- **ZKVM**: [Pico](https://github.com/brevis-network/pico) with AOT compilation
- **Language**: Rust
- **Cryptography**: ECDSA P-256 (device signature + Root CA cert chain), SHA-256
- **Parallelism**: Rayon + per-core sharding

---

## 📜 Core Philosophy: Proof of Origin
> "Don't trust, verify."
> Brevis Vera proves that we can allow digital creativity without sacrificing truth.
