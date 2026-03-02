# Brevis Vera — Development Plan (SDD Methodology)

## Methodology

This project follows **Spec-Driven Development (SDD)**:
1. Write/Iterate Spec → 2. AI Execution → 3. Testing & Validation → 4. Refine Spec → Loop

**No multi-agent parallelism**. A single agent maintains context consistency and advances sequentially according to the spec.

---

## Overview

```
Step 1: Environment Setup + Spike Validation
Step 2: Write Spec v1 (Data Structures + Interfaces)
Step 3: Implement Mock Signer + Image Editing
Step 4: Spike ZK Circuit Feasibility
Step 5: Iterate Spec v2 (Refine based on spike results)
Step 6: Implement ZK Circuit + Prover
Step 7: Implement Verifier CLI
Step 8: End-to-End Integration Testing
Step 9: (Bonus) Web UI
```

---

## Step 1: Environment Setup + Spike Validation ⏱️ ~1-2h

**Goal:** Confirm toolchain functionality and understand the capabilities of Pico ZKVM.

### Tasks
- [x] Install Rust nightly-2025-08-04
- [x] Install Pico CLI
- [x] Create test project using `cargo pico new --template basic`
- [x] Run Fibonacci example: build → prove → verify full lifecycle
- [x] Review Pico examples directory to understand IO and commit APIs

### Spike Questions (To be answered)
1. How is program IO handled in Pico ZKVM (stdin? function arguments?)
2. How are public values committed?
3. What is the proof file format? How is independent verification performed?
4. Which crates are compatible in a `no_std` environment?

### Output
- `spike_report.md`: Answers to the above questions.

### 🚀 Test & Verification
- [x] **Command**: `cd Fibonacci/app && cargo pico build && cd ../prover && cargo run --release`
- [x] **Assertion**: Terminal output shows `Verification succeeded`.
- [x] **Binary Check**: Ensure the latest ELF is generated in `app/elf`.

---

## Step 2: Write Spec v1 ⏱️ ~2-3h

**Goal:** Define all data structures and module interfaces to provide precise instructions for AI implementation.

### Definitions Required

#### 2.1 Data Structures
- `PhotoMetadata`: Device ID, timestamp, dimensions, SHA-256 image hash.
- `Signature`: ECDSA P-256 signature (r, s) + public key.
- `SignedPhoto`: Raw image bytes + PhotoMetadata + Signature.
- `EditOperation`: Enum (Crop{x,y,w,h}, Brightness{delta}, ...).
- `EditManifest`: `Vec<EditOperation>` — Sequence of edit operations.
- `ProofPackage`: Edited image + ZK Proof + Public Commitments.

#### 2.2 Module Interfaces
- **mock-signer**: Keygen, sign_photo, verify_signature.
- **editor**: `apply_edits(image_bytes, edit_manifest) -> edited_bytes`.
- **zk-circuit (app/)**: Verify signature, verify image integrity, replay edits, commit public values.
- **prover**: `prove(signed_photo, edit_manifest) -> ProofPackage`.
- **verifier**: `verify(proof_package) -> AuthenticityVerdict`.

### Output
- `spec-v1.md`: Complete specification document.

---

## Step 3: Implement Mock Signer + Image Editing ⏱️ ~3-4h

**Goal:** Implement non-ZK modules first to validate data flow.

### 3.1 Mock Signer
- [x] Implement ECDSA P-256 key generation (`p256` crate).
- [x] Implement `sign_photo`: SHA-256(metadata) → ECDSA Signature.
- [x] Implement `verify_signature`.
- [x] CLI: `brevis-vera sign --image DSC00050.JPG --output signed_photo.json`

### 3.2 Image Editor
- [x] Implement `Crop` (Mandatory).
- [x] Implement second transformation (Brightness).
- [x] CLI: `brevis-vera edit --input signed_photo.json --ops "crop:10,10,200,200" --output edited.png`

---

## Step 4: Spike ZK Circuit Feasibility ⏱️ ~2-3h

**Goal:** Validate critical technical points before building the main circuit.

### Tasks
- [x] **Spike A:** Test SHA-256 performance inside Pico ZKVM.
- [x] **Spike B:** Test ECDSA P-256 verification inside ZKVM.
- [x] **Spike C:** Measure execution time for `crop` on a small image (e.g., 32x32).
- [x] **Spike D:** Determine IO limits for passing raw bytes (ImageBytes) into ZKVM.

### Output
- `spike_zk_report.md`: Conclusions and spec adjustments.

---

## Step 5: Iterate Spec v2 ⏱️ ~1-2h

**Goal:** Refine the specification based on Step 4 results.

### Refinements
- Limit image size in spec if performance is a bottleneck.
- Specify manual implementation of certain algorithms if crates are incompatible.
- Define final ZK public input/output structure.

### Output
- `spec-v2.md`: Revised executable spec.

---

## Step 6: Implement ZK Circuit + Prover ⏱️ ~6-10h

**Goal:** Core implementation based on `spec-v2.md`.

### 6.1 ZK Circuit (app/)
- [x] Read inputs (SignedPhoto + Manifest).
- [x] Verify signature in ZKVM.
- [x] Replay edits in ZKVM.
- [x] Compute output image hash.
- [x] Commit public outputs.

### 6.2 Prover
- [x] Prepare input data.
- [x] Call Pico SDK to execute and generate proof.
- [x] Save `ProofPackage`.

---

## Step 7: Implement Verifier CLI ⏱️ ~2-3h

**Goal:** Independent verification tool.
- [x] Load `ProofPackage`.
- [x] Call Pico SDK to verify proof.
- [x] Check public outputs (hash match, trusted key).
- [x] Output `AuthenticityVerdict`.

---

## Step 8: End-to-End Integration Testing ⏱️ ~1-2h

**Goal:** Run the full workflow.
1. Sign → 2. Edit → 3. Prove → 4. Verify.

### Output
- ✅ `test_e2e.sh`: Automated test script covering all cases.

---

## Step 9: (Bonus) Web UI ⏱️ ~4-6h

- [x] Standalone Web Verifier (`web-verifier.html`).
- [x] Visualized proof results and edit history timeline.

---

## Schedule Overview

| Step | Content | Time | Type |
|------|------|------|------|
| 1 | Setup + Spike | 1-2h | Discovery |
| 2 | Spec v1 | 2-3h | **Spec** |
| 3 | Mock Signer + Edit | 3-4h | Implementation |
| 4 | ZK Spike | 2-3h | Discovery |
| 5 | Spec v2 | 1-2h | **Spec** |
| 6 | ZK Circuit + Prover | 6-10h | Implementation |
| 7 | Verifier | 2-3h | Implementation |
| 8 | Integration Test | 1-2h | Testing |
| 9 | Web UI (Bonus) | 4-6h | Implementation |
| **Total** | | **18-29h** | |

> Spec writing accounts for ~20% of time but determines ~80% of execution quality. This is SDD.
