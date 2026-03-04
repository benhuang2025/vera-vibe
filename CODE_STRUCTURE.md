# Brevis Vera — Code Structure

## Workspace Layout

```
vera-vibe/
├── brevis-vera-zk/              # Rust workspace root
│   ├── lib/                     # Shared types, pixel operations, block commitment
│   ├── shard-app/               # ZK guest: per-shard block hashing + edits
│   ├── aggregator-app/          # ZK guest: ECDSA verify + commitment check
│   ├── app/                     # ZK guest: monolithic (non-sharded, reference)
│   ├── prover/                  # Host: parallel AOT orchestrator + STARK proof
│   ├── mock-signer/             # CLI: keygen, sign, edit, verify
│   ├── web-server/              # Axum HTTP API backend
│   ├── shard-aot/               # AOT-compiled shard runtime
│   ├── aggregator-aot/          # AOT-compiled aggregator runtime
│   ├── vm/                      # Pico ZKVM runtime (forked from brevis-network/pico)
│   ├── aot-runtime/             # AOT emulator core
│   └── derive/                  # Procedural macros
├── web-app/                     # Frontend (HTML/CSS/JS)
├── images/                      # Source photos
└── test_e2e.sh                  # End-to-end test script
```

---

## Data Flow Overview

```
[Camera / mock-signer sign]
        │
        │  image → pixels → block_hashes → image_commitment
        │  metadata + commitment → ECDSA sign → SignedPhoto
        ▼
[mock-signer edit]
        │
        │  ops (crop, brightness, contrast, grayscale, rotate90)
        │  → pixel_utils → edited pixels + EditManifest
        ▼
[prover]
        │
        ├─→ Shard 0..N (parallel AOT)
        │     pixel_segment + ops → orig/edited block hashes
        │
        └─→ Aggregator (AOT or STARK)
              metadata + signature + all block hashes
              → ECDSA verify + commitment check → PublicValues
        ▼
[mock-signer verify / web-server verify]
        │
        │  edited image → block_hashes → hash comparison
        │  root_ca_hash trust check
        │  optional STARK proof verification
        ▼
      VERIFICATION RESULT
```

---

## `lib/src/lib.rs` — Shared Library

The core data types and utility functions used by all binaries.

### Constants

| Name | Value | Purpose |
|------|-------|---------|
| `BLOCK_SIZE` | 65536 (64KB) | Protocol-level block size for image chunking |

### Structs

| Struct | Fields | Purpose |
|--------|--------|---------|
| `PhotoMetadata` | device_id, timestamp, width, height, image_commitment | Camera-signed metadata |
| `Signature` | r, s, public_key, root_ca_pubkey, device_cert_r/s | ECDSA P-256 signature + cert chain |
| `SignedPhoto` | image_bytes, metadata, signature | Complete signed photo bundle |
| `EditManifest` | operations: Vec\<EditOperation\> | List of edits applied |
| `PublicValues` | original_image_commitment, pub_key_hash, root_ca_hash, edit_types, output_image_hash | ZK public outputs |
| `ProofPackage` | edited_image, proof, public_values | Deliverable to verifier |

### Enum: `EditOperation`

| Variant | Parameters | In-ZK |
|---------|-----------|-------|
| `Crop` | x, y, width, height | Host-level |
| `AdjustBrightness` | delta: i16 | ✅ In shard |
| `AdjustContrast` | factor: u16 | ✅ In shard |
| `Grayscale` | — | Host-level |
| `Rotate90` | — | Host-level |

### Functions

| Function | Signature | Purpose |
|----------|-----------|---------|
| `compute_block_hashes` | `(&[u8]) → Vec<[u8;32]>` | SHA256 each 64KB block of pixel data |
| `compute_image_commitment` | `(&[[u8;32]]) → [u8;32]` | SHA256 of concatenated block hashes |
| `PhotoMetadata::to_bytes` | `(&self) → Vec<u8>` | Deterministic serialization for signing |

### `pixel_utils` Module

| Function | Purpose |
|----------|---------|
| `apply_crop(pixels, w, h, x, y, new_w, new_h)` | Extract rectangular region |
| `apply_brightness(pixels, delta)` | Add delta to each channel, clamped to 0–255 |
| `apply_contrast(pixels, factor)` | Scale channels around midpoint (factor=100 = no change) |
| `apply_grayscale(pixels)` | RGB → gray via (77R + 150G + 29B) / 256 |
| `apply_rotate90(pixels, w, h)` | 90° clockwise rotation, swaps w↔h |

---

## `shard-app/src/main.rs` — ZK Shard Guest

Runs inside the ZKVM. Processes one pixel segment and outputs block hashes.

### Input (deserialized via `read_as`)
```
(pixel_segment: Vec<u8>, edit_ops: Vec<EditOperation>, num_blocks: usize)
```

### Logic
1. Split `pixel_segment` into `num_blocks` blocks (64KB each)
2. Compute `orig_block_hashes` — SHA256 of each original block
3. Apply edit operations to pixel data (only `AdjustBrightness` and `AdjustContrast`)
4. Compute `edited_block_hashes` — SHA256 of each edited block
5. Commit `(orig_block_hashes, edited_block_hashes)`

---

## `aggregator-app/src/main.rs` — ZK Aggregator Guest

Runs inside the ZKVM. The critical trust anchor — verifies identity and computes commitments.

### Inputs (deserialized via `read_as`)
1. `PhotoMetadata`
2. `Signature`
3. `all_orig_block_hashes: Vec<[u8;32]>`
4. `all_edited_block_hashes: Vec<[u8;32]>`

### Logic
1. **ECDSA P-256 Verify** — verify device signature on metadata (software `p256` crate)
2. **Trust Anchor** — `root_ca_hash = SHA256(root_ca_pubkey)`
3. **Hard Link** — assert `compute_image_commitment(orig_hashes) == metadata.image_commitment`
4. **Output Hash** — `output_image_hash = compute_image_commitment(edited_hashes)`
5. **Identity** — `pub_key_hash = SHA256(public_key)`
6. **Commit** `PublicValues` as ZK public output

### Why only block hashes?
The aggregator receives ~36KB of hashes instead of ~37MB of raw pixels. This eliminates the I/O deserialization bottleneck that would make STARK proof generation infeasible.

---

## `prover/src/main.rs` — Host-Side Orchestrator

The main proving binary. Runs on the server, coordinates shards and aggregator.

### CLI Arguments
| Arg | Default | Purpose |
|-----|---------|---------|
| `--photo` | signed_photo.json | Signed photo path |
| `--manifest` | edit_manifest.json | Edit manifest path |
| `--output` | proof_package.json | Output path |
| `--edited-image` | edited.png | Edited image path |
| `--real-proof` | false | Generate STARK proof |

### Flow
1. Load `SignedPhoto` and `EditManifest`
2. **Host-side cert chain verification** — Root CA → Device Key
3. **Sharding (Plan D)**:
   - `num_shards = num_cpus::get()` (e.g., 124)
   - `blocks_per_shard = ceil(570 / 124) = 5`
   - If manifest has host-level ops (crop/grayscale/rotate90): apply on host, compute hashes directly
   - Otherwise: distribute pixel segments to parallel shards via Rayon
4. **Shard execution** — each shard runs `shard-app` ELF via `AotEmulatorCore`
5. **Flatten** — collect all orig/edited block hashes from shard outputs
6. **Aggregator**:
   - **Emulation mode**: run aggregator-app via `AotEmulatorCore`
   - **Real STARK**: run `RiscvProver` → `meta_proof` → verify → save `stark_proof.bin` + `stark_vk.bin`
7. **Save** `ProofPackage`

---

## `app/src/main.rs` — Monolithic ZK Guest (Reference)

A non-sharded version that processes the entire image in one ZK program. Used for testing and as a reference implementation.

### Inputs
1. `SignedPhoto`
2. `EditManifest`

### Logic
1. Compute block hashes + image commitment; assert matches signed commitment
2. Verify ECDSA signature
3. Replay ALL edit operations (including crop, grayscale, rotate90)
4. Compute output image hash
5. Commit `PublicValues`

---

## `mock-signer/src/main.rs` — CLI Tool

Simulates the camera and verifier roles.

### Commands

**`keygen --output <path>`**
- Generate Root CA key + Device key (P-256)
- Root CA signs device public key → device cert
- Output: PEM key file + cert JSON + root CA hash

**`sign --image <path> --key <path> --output <path>`**
- Load image → decode to RGB → compute block hashes → image commitment
- Build `PhotoMetadata` and sign with device key
- Output: `SignedPhoto` JSON

**`edit --input <path> --ops <string> --output <path> --manifest <path>`**
- Parse ops string: `crop:x,y,w,h;brightness:10;contrast:120;grayscale;rotate90`
- Apply each op via `pixel_utils`
- Output: edited PNG + `EditManifest` JSON

**`verify --package <path> --image <path> --trusted-pubkey-hash <hex>`**
- Load proof package and edited image
- Compute output hash via block commitment
- Compare with `public_values.output_image_hash`
- Check `root_ca_hash` against trusted hash
- If `stark_proof.bin` exists: verify STARK proof via `RiscvMachine`

---

## `web-server/src/main.rs` — Axum Backend

### State
- `AppState` holds `data_dir`, `zk_dir`, `sessions` (HashMap), `jobs` (HashMap)
- Each session stores keys, signed photo, manifest, proof path

### API Endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/api/health` | GET | Health check |
| `/api/keygen` | POST | Create session + keys → session_id, root_ca_hash |
| `/api/sign` | POST | Upload image (multipart) → sign → num_blocks, commitment |
| `/api/edit` | POST | Apply ops → edited.png URL |
| `/api/prove` | POST | Start async prove job → job_id |
| `/api/prove/status/{id}` | GET | Poll job progress/result |
| `/api/verify` | POST | Upload image + proof (multipart) → verdict |
| `/api/data/{session}/{file}` | GET | Serve session files |

### Key Functions

| Function | Purpose |
|----------|---------|
| `run_prove_job` | Blocking task: shard + aggregate + optional STARK |
| `parse_ops` | Parse ops string → Vec\<EditOperation\> |
| `edit_photo` | Apply edits, save edited.png and manifest |
| `verify_photo` | Block-hash the uploaded image, compare with proof |

---

## `web-app/` — Frontend

### Files
- `index.html` — Three-tab layout (Upload & Sign, Edit & Prove, Verify)
- `app.js` — All interaction logic
- `style.css` — Dark theme with glass-morphism

### `app.js` Key Functions

| Function | Purpose |
|----------|---------|
| `handleUploadFile(file)` | Preview image, enable sign button |
| `buildOpsString()` | Combine UI selections into `crop:x,y,w,h;brightness:N;...` |
| `showProveResults(data)` | Display proof type, hashes, timing, download link |
| `showVerdict(data)` | Render verification checks and pass/fail banner |
| `setupDropBox(el, input, onFile)` | Reusable drag-and-drop handler |

### UI Flow
1. **Upload & Sign** → drag/drop photo → POST keygen + sign → show root CA hash + block count
2. **Edit & Prove** → toggle edits → Apply → POST edit → see preview → POST prove → poll status
3. **Verify** → drop image + proof JSON → optional root CA hash → POST verify → verdict

---

## `test_e2e.sh` — E2E Test Script

### Usage
```bash
./test_e2e.sh              # Emulation mode
./test_e2e.sh --real-proof  # Real STARK proof
```

### Steps
1. `mock-signer keygen` → capture ROOT_CA_HASH
2. `mock-signer sign` → signed_photo.json
3. `mock-signer edit --ops ""` → passthrough edited image
4. `prover` → proof_package.json (+ optional STARK)
5. **Verify A** (positive): correct image + correct hash → SUCCESSFUL
6. **Verify B** (tampered): brightness-altered image → MISMATCH
7. **Verify C** (wrong key): wrong root CA hash → UNTRUSTED
