Brevis Vera: Combating AI Deepfake with ZK-Powered Media Authenticity Attestation
Seeing is not Believing
Not long ago, a photo carried a presumption of truth. Today it carries nothing but a question mark. A Reuters Report found that 58% of people worldwide worry about what is real and what is fake online. Suspicion is becoming the default mode of media consumption. The first reply to any viral video or photos on X now is almost always “@grok, is this real?”
So, maybe we can use AI to fight AI? If generative models create deepfakes, perhaps other models can detect them. However, this is not really working. A 2025 evaluation of leading multimodal models, including ChatGPT, Claude, Gemini, and Grok, concluded they are “not yet dependable as standalone detection systems” for deepfake images. A more recent benchmark on deepfake video reasoning found that state-of-the-art vision-language models struggle significantly with temporal inconsistencies and achieve low detection accuracy without heavy task-specific training. Even specialized detectors degrade sharply on real-world, in-the-wild data with many systems performing only marginally better than random guessing.
More importantly, using AI to detect AI is fundamentally a never-ending arms race. Generators improve. Detectors need retraining. The cycle repeats indefinitely. 
Brevis Vera: Show me the Proof
If detection is a never-ending cat-and-mouse game, provenance is the only answer.
Instead of asking whether an image looks real, we should ask a simpler question:
Can this image prove where it came from?
We introduce Brevis Vera — an end-to-end digital media authenticity attestation system that allows anyone to verify that a published image or video originated from a real-world capture event on a real device.
The solution begins at the moment of capture.
Capture: Hardware-Backed Authenticity
A growing number of device manufacturers now support the C2PA (Coalition for Content Provenance and Authenticity) standard. With C2PA, a device can cryptographically sign media at the time it is captured. This on-device signature binds the content to the hardware, producing tamper-evident provenance metadata.
That signature answers the first question of our problem here:
Was this captured by a real camera on a real device?
But this is only the starting point.
Because in the real world, raw media is rarely what gets finally published.
The Editing Problem
Journalists crop images. Creators blur faces. Editors redact private information. Exposure and color are adjusted. Subtitles and annotations are added. Finally, everything is compressed for faster mobile loading time. 
These edits are legitimate and necessary.
Yet the moment you modify a signed image, the original hardware signature no longer directly applies. Even a simple crop breaks the cryptographic binding between the signed raw file and the published version.
So we face a paradox:
We want authenticity.
We need editing.
But editing breaks authenticity.
Bridging the Gap with Zero-Knowledge Proofs
This is where Brevis Vera changes the model.
Brevis Vera integrates with open-source editing libraries, beginning with Photon, and uses Brevis Pico ZKVM to generate a zero-knowledge proof of the editing process itself.
When an editor modifies the media with a supported media processing software, Brevis Vera:
Takes the original C2PA-signed media metadata and raw media as input


Executes the allowed transformations


Generates a zero-knowledge proof inside Brevis Pico ZKVM of the whole transformation process.


The proof mathematically attests that:
The output media is derived from the signed original


Only permitted transformations were applied


No hidden or malicious edits were introduced
The proof is generated locally by the editor and can be verified independently by anyone.
Why This Matters
Brevis Vera preserves what today’s systems cannot:
Privacy of the original raw content


Privacy of the editorial workflow


Cryptographic proof of real-world origin


Verification without centralized intermediaries


Full transparency through open-source design


High efficiency powered by Brevis Pico ZKVM


Instead of guessing whether something is real, Brevis Vera allows anyone to verify that it came from reality — and that it was transformed only in legitimate, provable ways.
Launching Soon
Brevis Vera Beta will be launching later this month.
The first release integrates with the Photon image editing library and supports a wide range of common transformations. We are currently in discussions with popular image and video editing consumer-facing applications to bring Brevis Vera directly into widely used creative tools.
If you are interested in integrating Brevis Vera into your product or workflow, reach out through our partner form.



