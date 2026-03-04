# Brevis Vera — Core Architecture Q&A

## Q1: Is the C2PA verification performed inside the ZKVM Program? If not, how do we ensure the image input to the Pico ZKVM Program is the same image verified by an external C2PA check?

**Answer: C2PA (or mock ECDSA P-256) signature verification must be executed INSIDE the ZKVM.**

### Reasoning

If signature verification occurs outside the ZKVM (e.g., in standard Rust code), the ZK Proof can only prove: "I received a set of pixels and performed a crop on them." It **cannot prove** that those pixels originated from a genuine hardware-signed photograph. An attacker could forge an image, claim "the signature was verified externally," and feed the fake data into the ZKVM.

### Implementation

The **original image, along with its signature and metadata**, is provided as a **Private Witness** to the ZKVM. The ZKVM Guest Program logic includes:

1.  **Metadata Parsing**: Extracting the signature values (r, s) and the hash of the original image.
2.  **Signature Verification**: Running the ECDSA P-256 verification algorithm within the ZKVM circuit to validate the signature against the hardware public key.
3.  **Hash Integrity**: Ensuring the SHA-256 hash of the provided raw image bytes matches the hash protected by the verified signature.

Only if all three steps pass inside the ZKVM is the resulting proof cryptographically meaningful. It proves: "This logic was executed based on an **authentic image verified by hardware signature**."

The signer's public key (or its hash) is committed as a **Public Output**, allowing verifiers to check if the public key belongs to a trusted device (e.g., a Sony camera).

---

## Q2: If we provide a UI for operations like cropping and then save the resulting image, how does Pico prove those UI operations? Does it need to re-run the logic in ZKVM?

**Answer: Yes. This is known in ZK proofs as "Deterministic Replay."**

### Reasoning

Directly comparing a "Pre-edit" and "Post-edit" image in ZK to prove "compliance" is extremely difficult. Pixel differences are too ambiguous; a ZKVM cannot distinguish between a "legal crop" and an "AI-generated face swap" simply by looking at the delta.

The standard approach is: **Do not compare images; replay the process.**

### Full Workflow

```
┌─────────────────────────────────────────────────────────────┐
│  UI Layer (User-Facing)                                     │
│                                                             │
│  1. User uploads original signed image                      │
│  2. User interacts with UI (drag crop box, adjust brightness)│
│  3. UI records parameters: [Crop(x:100, y:100, w:500, h:500)]│
│  4. UI generates preview via local lib (e.g. Photon), saves  │
│     the edited image                                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼ Submission: Original Image + Signature + Operation List
┌─────────────────────────────────────────────────────────────┐
│  ZKVM Layer (Pico Guest Program)                             │
│                                                             │
│  1. Verify ECDSA P-256 signature of original image          │
│  2. Deterministically replay the same operations in memory  │
│  3. Compute the SHA-256 hash of the replayed result         │
│  4. Commit Public Output:                                   │
│     - Signer's Public Key Hash (Proof of Origin)            │
│     - List of Edit Types (Proof of what happened)           │
│     - Hash of Edited Image (Proof of Result Integrity)      │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼ Output: Proof + Public Output
┌─────────────────────────────────────────────────────────────┐
│  Verification Layer (Verifier)                              │
│                                                             │
│  1. Use Pico SDK to verify the mathematical soundness       │
│  2. Independently hash the received edited image            │
│  3. Compare: computed hash == hash in Proof Public Output?  │
│  4. Check if Public Key belongs to a trusted device         │
│  5. Output final verdict: ✅ Verified or ❌ Rejected         │
└─────────────────────────────────────────────────────────────┘
```

### Key Takeaways

-   **Hash Comparison occurs on the Verifier side**, not inside the ZKVM. The ZKVM commits the resulting hash as a Public Output. The verifier calculates their own hash of the received image and checks it against the proof.

-   **Edit Parameters (e.g., specific crop coordinates) remain Private**. The Public Output only exposes the "Types" of operations (e.g., "Crop and Brightness adjustment performed"), hiding the exact details as per the Product Brief requirements.

-   **Operations must be Deterministic**. Same Input + Same Parameters = Same Pixels. This rules out filters with unseeded randomness (like random grain/noise) unless the seed is provided as part of the parameters.

-   **Simplicity is crucial**. Re-running pixel processing in a ZKVM consumes significantly more computational resources (Constraints) than a standard CPU.

### Cryptographic Lifecycle Proof

| Property | Guaranteed By |
|----------|---------------|
| Authenticity of Source | Signature verification inside ZKVM |
| Limited Scope of Edits | Deterministic replay inside ZKVM |
| No External Tampering | Hash comparison (Result vs. Proof Output) |
| Privacy of Edit Details | Zero-knowledge nature of ZK (Params are Private) |
| Non-Forgeability | Soundness property of the ZK Proof |
