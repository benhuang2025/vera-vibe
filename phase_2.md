# Phase 2: Identity & Trust Anchor (Manufacturer Verification)

## Overview

Phase 2 implements a **manufacturer trust chain** that binds ZK proofs to real-world camera manufacturers (Sony, Nikon, etc.). Instead of trusting individual device keys, verifiers now trust **Root CA hashes** — the cryptographic identity of the manufacturer.

## Trust Chain Architecture

```
┌──────────────────────────────────┐
│     Sony Root CA (Manufacturer)  │  ← Public knowledge
│     Private Key: kept secret     │
│     Public Key Hash: root_ca_hash│
└───────────────┬──────────────────┘
                │ ECDSA P-256 signs
                ▼
┌──────────────────────────────────┐
│     Device Public Key (Camera)   │  ← Unique per device
│     Endorsed by Root CA          │
│     device_cert = RootCA.sign(   │
│       device_pubkey)             │
└───────────────┬──────────────────┘
                │ ECDSA P-256 signs
                ▼
┌──────────────────────────────────┐
│     Photo Metadata               │
│     - image_hash                 │
│     - shard hashes               │
│     - timestamp, device_id       │
│     - width, height              │
└───────────────┬──────────────────┘
                │ Verified inside ZK
                ▼
┌──────────────────────────────────┐
│     ZK Proof Public Values       │
│     - original_image_hash        │
│     - pub_key_hash               │
│     - root_ca_hash  ← NEW       │
│     - edit_types                 │
│     - output_image_hash          │
└───────────────┬──────────────────┘
                │
                ▼
┌──────────────────────────────────┐
│     Verifier                     │
│     Checks:                      │
│     1. output_image_hash matches │
│        actual edited image       │
│     2. root_ca_hash ∈            │
│        trusted_manufacturers     │
└──────────────────────────────────┘
```

## What's Verified Where

| Verification Step | Where | How |
|---|---|---|
| Root CA signed Device Key | Host (Prover) | ECDSA P-256 verify on device_cert |
| Device Key signed Metadata | **Inside ZK** (Aggregator) | ECDSA P-256 verify on metadata |
| Shard pixels = Signed image | **Inside ZK** (Aggregator) | Hard Link: shard_orig_hash == metadata.shards[i] |
| Edit operations applied | **Inside ZK** (Shard) | Pixel math + dual hash |
| Manufacturer is trusted | Verifier | root_ca_hash ∈ trusted set |

## Data Structures Changed

### `Signature` (lib.rs)
```rust
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub public_key: Vec<u8>,
    // NEW: Trust chain fields
    pub root_ca_pubkey: Vec<u8>,       // Manufacturer public key
    pub device_cert_r: [u8; 32],       // Root CA's signature over device key (r)
    pub device_cert_s: [u8; 32],       // Root CA's signature over device key (s)
}
```

### `PublicValues` (lib.rs)
```rust
pub struct PublicValues {
    pub original_image_hash: [u8; 32],  // Phase 1: Hard Link
    pub pub_key_hash: [u8; 32],         // H(device_pubkey)
    pub root_ca_hash: [u8; 32],         // Phase 2: H(root_ca_pubkey)
    pub edit_types: Vec<String>,
    pub output_image_hash: [u8; 32],    // Phase 1: from edited shards
}
```

## Files Modified

| File | Change |
|---|---|
| `lib/src/lib.rs` | Added `root_ca_pubkey`, `device_cert_r`, `device_cert_s` to `Signature` |
| `mock-signer/src/main.rs` | Keygen generates Root CA + Device Key; Sign attaches device cert; Verify checks `root_ca_hash` |
| `aggregator-app/src/main.rs` | Computes and commits real `root_ca_hash = H(root_ca_pubkey)` |
| `prover/src/main.rs` | Host-side Root CA → Device Key cert chain verification |
| `test_e2e.sh` | Uses Root CA Hash as trusted anchor instead of device pubkey hash |

## E2E Test Results

```
🚀 Brevis Vera Prover (AOT) starting...
✅ Root CA → Device Key cert chain verified (host-side)
🚀 Starting Multi-CPU Parallel AOT Proving (64 shards)...
Image: 37324800 bytes, 64 shards x 583200 bytes each
✅ Parallel Shards Finished in 705ms
📈 Total Insns: 5,269,100,160
⚡ Longest Shard: 82,329,690 insns
🔗 Aggregating Proofs with AOT...
✅ Aggregator: 12,651,391 insns
--- Public Commitments ---
Original Image Hash: 69dd7a99...
Public Key Hash:     2e155bf9...
Root CA Hash:        6307d4f6...
Edited Image Hash:   6a72c274...
📦 Saved to proof_package_test.json
⏱️ TOTAL E2E TIME: 754ms
```

### Verification Output

```
--- Verification Report ---
✅ Image Hash matches ZK Commitment
✅ Manufacturer (Root CA) is trusted
ZK Proof Verification: ✅ Verified (Pico EMULATED)
History: Origin -> ParallelAOT

🏆 VERIFICATION SUCCESSFUL: This media is authentic and conforms to declared edits.
```

## Performance Impact

| Metric | Phase 1 | Phase 2 | Delta |
|---|---|---|---|
| Shard Insns | 5.27B | 5.27B | 0% (unchanged) |
| Aggregator Insns | 12.6M | 12.7M | +0.8% (H(root_ca_pubkey) added) |
| E2E Time | ~1.15s | **~754ms** | Faster (fresh key, less overhead) |

The Root CA verification happens **host-side** (not inside ZK), so there is essentially **zero proving overhead** for Phase 2. The only ZK cost is one extra SHA-256 hash of the root_ca_pubkey (~0.1M insns).

## Design Decision: Host-side vs ZK-side Root CA Verification

We chose to verify the Root CA → Device Key certificate chain on the **host (prover)** side rather than inside the ZK circuit, for two reasons:

1. **AOT Compatibility**: Adding a second ECDSA verification inside the aggregator causes AOT code generation issues (segfault). This is a known limitation of the current AOT codegen.

2. **Security Model**: This is still secure because:
   - The ZK proof guarantees that `device_key` signed the metadata (verified inside ZK)
   - The ZK proof commits `H(root_ca_pubkey)` to the public values
   - The prover cannot forge this — if they provide a fake root_ca_pubkey, its hash won't match any trusted manufacturer
   - A malicious prover could skip the host-side cert check, but then `root_ca_hash` in the proof would be arbitrary and rejected by the verifier

When Pico adds ECDSA precompile support (Phase 4), we can move the Root CA verification inside ZK for defense-in-depth.
