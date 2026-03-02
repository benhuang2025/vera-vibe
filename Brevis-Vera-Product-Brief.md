Brevis Vera Product Brief
Overview
Brevis Vera (please see more details in the blog-post.md) is an end-to-end digital media authenticity attestation system that:
Verifies hardware-backed authenticity signatures (C2PA-style provenance)


Allows editing of digital media


Generates a Zero-Knowledge Proof of the editing process using Pico ZKVM


Enables third-party verification without trusting the editor


Your task is to build a working prototype that demonstrates this full flow.
The emphasis is not on perfect production-readiness, but on:
Functional architecture


Clear user flow


Demonstrable ZK-backed proof generation and verification


Effective use of AI-assisted development tools



Required Functionalities
Your prototype must include:
1️⃣ Capture & Provenance Layer
Integrate C2PA-style signed media input. 
Accept a media file with provenance metadata. 
Verify authenticity signature and cert chain before editing


Caveat: I am working on getting a real C2PA-signed media, for the time being, you can use a ECDSA (ECDSA P-256) signed mock photo metadata as the signature verification step and skip the X.509 certificate chain authentication logic (just verify one signature). I should be able to get a real C2PA-signed photo very soon. 



2️⃣ Editing Layer
Integrate with an open-source image editing library or user-facing apps (Photon is a good example of an Rust-based image processing lib, but with no native UI. Feel free choose a one you see fit)
Support at least 2 transformations with image Crop being mandatory. 
An UI to upload and edit is not mandatory. You can also programmatically edit the image to demonstrate the whole flow. 


3️⃣ ZK Proof Generation (Core Requirement)
Use Pico ZKVM to generate a proof that:


The edited output derives from the signed original
Certain types of transformations were applied. However, hide the details of such an operation. For example, no need to retain information such as original size before cropping. 
No additional modifications occurred


The proof must be verifiable independently.

4️⃣ Verification Layer
Provide a verificationCLI/web UI that:
Verifies the edited image along with the ZK Proof to attest that it came from an real-world device capture and have gone through certain kinds of edits. 


Outputs a clear authenticity verdict




Optional Bonus Features
Friendly UX that is demo-able with a consumer facing UI. 
Real C2PA integration for Sony Devices (will provide real signed image)
More types of edits
Video support
Performance benchmarking

🏁 Evaluation Criteria
Judging will be based on the following dimensions:
Completeness
Does the full capture → edit → prove → verify flow work? 
Are all required components implemented correctly?
How close is this to actual production-readiness?


Vibe Coding Excellence
Effective use of AI-assisted development tools
How much you actually understand the AI-generated product architecture and code structure
Every team must present their AI workflow and demonstrate how they used vibe coding to accelerate development.
UX & Clarity
Is the user flow intuitive?


Future production-readiness
Is this ready to be extended to a real full product?




