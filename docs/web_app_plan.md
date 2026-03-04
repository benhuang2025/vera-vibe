# Brevis Vera Web App — Detailed Implementation Plan

Tech: Rust Axum backend + plain HTML/JS/CSS frontend, deployed on this Linux server.

---

## Step 1: Axum Server Skeleton + Health Check

**What**: Create `web-server` crate, add Axum with CORS, serve a single `/api/health` endpoint.

**Files**:
- `brevis-vera-zk/web-server/Cargo.toml`
- `brevis-vera-zk/web-server/src/main.rs`
- Add `web-server` to workspace `Cargo.toml`

**Test**:
```bash
cargo run --release --bin web-server &
curl http://localhost:3000/api/health
# Expected: {"status":"ok"}
```

---

## Step 2: Static File Serving + Landing Page

**What**: Serve `web-app/` directory as static files. Create `index.html` with basic layout and navigation (3 tabs: Upload & Sign, Edit & Prove, Verify).

**Files**:
- `web-app/index.html` — shell with 3 tab buttons, empty content areas
- `web-app/style.css` — dark theme, layout, tab navigation
- Update `main.rs` to serve static files from `../web-app/`

**Test**:
```bash
# Server running on :3000
# Open browser: http://localhost:3000
# Expected: See dark-themed page with 3 clickable tabs, no errors in console
```

---

## Step 3: `/api/keygen` Endpoint

**What**: POST endpoint that generates Root CA + Device keys, stores them in `data/` dir, returns hashes.

**Files**:
- `web-server/src/handlers/keygen.rs`
- `web-server/src/state.rs` — shared AppState with data dir path

**Test**:
```bash
curl -X POST http://localhost:3000/api/keygen | jq
# Expected:
# {
#   "session_id": "uuid",
#   "root_ca_hash": "hex...",
#   "device_pubkey_hash": "hex..."
# }
# Verify: ls data/<session_id>/ → private_key.pem, root_ca.pem, device_cert.json
```

---

## Step 4: `/api/sign` Endpoint

**What**: POST multipart upload of image file + session_id. Signs the image with the session's device key. Returns signed photo metadata.

**Files**:
- `web-server/src/handlers/sign.rs`

**Test**:
```bash
curl -X POST http://localhost:3000/api/sign \
  -F "session_id=<from step 3>" \
  -F "image=@images/DSC00056.JPG" | jq
# Expected:
# {
#   "num_shards": 124,
#   "image_hash": "hex...",
#   "width": 4320,
#   "height": 2880
# }
# Verify: ls data/<session_id>/ → signed_photo.json exists
```

---

## Step 5: Upload & Sign Page (Frontend)

**What**: First tab UI — drag-and-drop image upload, "Generate Keys & Sign" button, display results.

**Files**:
- `web-app/js/upload.js` — upload logic, API calls
- Update `index.html` — Upload tab content
- Update `style.css` — drag-drop zone, image preview, result cards

**Test**:
```
# In browser: http://localhost:3000
# 1. Click "Upload & Sign" tab
# 2. Drag a JPG into the drop zone → see image preview with dimensions
# 3. Click "Generate Keys & Sign" → see loading spinner
# 4. See results: root_ca_hash, num_shards, image_hash displayed
# 5. Check browser console: no errors
```

---

## Step 6: `/api/edit` Endpoint

**What**: POST with session_id + edit operations string. Applies edits, saves edited image, returns preview URL.

**Files**:
- `web-server/src/handlers/edit.rs`

**Test**:
```bash
# Test 1: No edits (passthrough)
curl -X POST http://localhost:3000/api/edit \
  -H "Content-Type: application/json" \
  -d '{"session_id":"<id>", "ops":""}' | jq
# Expected: { "edited_image_url": "/data/<id>/edited.png", "width": 4320, "height": 2880 }

# Test 2: Crop + brightness
curl -X POST http://localhost:3000/api/edit \
  -H "Content-Type: application/json" \
  -d '{"session_id":"<id>", "ops":"crop:100,100,1000,800;brightness:20"}' | jq
# Expected: { "edited_image_url": "...", "width": 1000, "height": 800 }

# Verify: the edited image is viewable at the URL
curl -o /tmp/test_edit.png http://localhost:3000/data/<id>/edited.png
file /tmp/test_edit.png  # → PNG image data
```

---

## Step 7: `/api/prove` Endpoint

**What**: POST with session_id + real_proof flag. Runs AOT emulation (+ optional STARK proving). Since proving takes time, returns a job_id and status URL.

**Files**:
- `web-server/src/handlers/prove.rs`

**API Design**:
- `POST /api/prove` → `{ "job_id": "uuid" }` (starts async proving)
- `GET /api/prove/status/:job_id` → `{ "status": "running" | "done", "result": {...} }`

**Test**:
```bash
# Start proving (emulation mode, ~2s)
JOB=$(curl -s -X POST http://localhost:3000/api/prove \
  -H "Content-Type: application/json" \
  -d '{"session_id":"<id>", "real_proof": false}' | jq -r .job_id)

# Poll status
sleep 3
curl http://localhost:3000/api/prove/status/$JOB | jq
# Expected:
# {
#   "status": "done",
#   "public_values": {
#     "original_image_hash": "hex...",
#     "output_image_hash": "hex...",
#     "root_ca_hash": "hex...",
#     "pub_key_hash": "hex..."
#   },
#   "proving_time_ms": 1500,
#   "proof_download_url": "/data/<id>/proof_package.json"
# }
```

---

## Step 8: Edit & Prove Page (Frontend)

**What**: Second tab UI — edit controls (crop box, brightness slider), prove button with progress, results display.

**Files**:
- `web-app/js/edit.js` — edit controls, prove API calls, polling
- Update `index.html` — Edit tab content
- Update `style.css` — slider, crop overlay, progress bar, result cards

**Test**:
```
# In browser (after completing Step 5 upload):
# 1. Click "Edit & Prove" tab → see the uploaded image
# 2. Adjust brightness slider → see preview update
# 3. Draw crop rectangle on image → see dimensions
# 4. Click "Apply Edits" → see edited image preview
# 5. Click "Generate ZK Proof" (emulation mode) → see progress bar
# 6. After ~2s: see public values displayed (hashes)
# 7. See "Download Proof Package" button
# 8. Toggle "Real STARK Proof" checkbox → click prove → ~60s with progress
```

---

## Step 9: `/api/verify` Endpoint

**What**: POST multipart with edited image file + proof package JSON + optional trusted_root_ca_hash. Independent verification — does NOT need any session state.

**Files**:
- `web-server/src/handlers/verify.rs`

**Test**:
```bash
# Test 1: Valid image + correct key
curl -X POST http://localhost:3000/api/verify \
  -F "image=@edited_test.png" \
  -F "proof_package=@proof_package_test.json" \
  -F "trusted_root_ca_hash=<hash>" | jq
# Expected:
# {
#   "image_hash_match": true,
#   "manufacturer_trusted": true,
#   "stark_verified": true,  (or "emulated": true)
#   "verdict": "VERIFICATION_SUCCESSFUL"
# }

# Test 2: Tampered image
curl -X POST http://localhost:3000/api/verify \
  -F "image=@tampered_image.png" \
  -F "proof_package=@proof_package_test.json" \
  -F "trusted_root_ca_hash=<hash>" | jq
# Expected: { "image_hash_match": false, "verdict": "VERIFICATION_FAILED" }

# Test 3: Wrong key
curl -X POST http://localhost:3000/api/verify \
  -F "image=@edited_test.png" \
  -F "proof_package=@proof_package_test.json" \
  -F "trusted_root_ca_hash=0000...0000" | jq
# Expected: { "manufacturer_trusted": false, "verdict": "VERIFICATION_FAILED" }
```

---

## Step 10: Verify Page (Frontend)

**What**: Third tab UI — two file upload zones (image + proof JSON), trusted hash input, verify button, visual verdict.

**Files**:
- `web-app/js/verify.js` — file upload, verify API call, verdict display
- Update `index.html` — Verify tab content
- Update `style.css` — verdict banner (green/red), checkmark animations

**Test**:
```
# In browser:
# 1. Click "Verify" tab
# 2. Upload edited image → see image preview
# 3. Upload proof_package.json
# 4. Paste root_ca_hash into text field
# 5. Click "Verify" → see results:
#    ✅ Image Hash Match
#    ✅ Manufacturer Trusted
#    ✅ STARK Proof Verified (or Emulated badge)
#    🏆 VERIFICATION SUCCESSFUL (large green banner)
# 6. Upload a different (tampered) image → click verify:
#    ❌ Image Hash MISMATCH
#    🚫 VERIFICATION FAILED (large red banner)
```

---

## Step 11: UI Polish

**What**: Premium dark theme, animations, responsive layout.

**Details**:
- Dark background with subtle gradients
- Glassmorphism cards for results
- Smooth tab transitions (fade/slide)
- Animated checkmarks on verification success
- Progress bar with percentage for proving
- Mobile-responsive layout
- Google Font (Inter or Outfit)

**Test**:
```
# Visual inspection in browser:
# - Looks premium, not generic
# - Tab switching is smooth
# - Proving progress is animated
# - Verification verdict has impact (large, colored, animated)
# - Works on mobile viewport (Chrome DevTools → responsive mode)
```

---

## Step 12: Deploy to Public URL

**What**: Open port 3000, optionally add Nginx + domain.

**Steps**:
1. Run server with `cargo run --release --bin web-server`
2. Open firewall: `sudo ufw allow 3000` (or cloud security group)
3. Access via `http://<server-public-ip>:3000`

**Test**:
```bash
# From your Mac:
curl http://<server-ip>:3000/api/health
# Expected: {"status":"ok"}
# Open in browser: http://<server-ip>:3000 → full web app loads
```

**Deployment (Completed)**:
1. Build: `cd brevis-vera-zk && cargo build --release --bin web-server`
2. Run: `cargo run --release --bin web-server` (must run from `brevis-vera-zk/` directory)
3. Server listens on `http://0.0.0.0:3000` — accessible via `http://<server-public-ip>:3000`
4. Firewall: `sudo ufw allow 3000` (if ufw installed) or configure cloud security group

---

## Dependency Summary

Backend: `axum`, `tokio`, `tower-http` (cors, static files, multipart), `serde_json`, `uuid`
Frontend: Pure HTML + CSS + JavaScript (no npm, no build step)

## Order of Execution

```
Step 1  → Step 2  → Step 3  → Step 4  → Step 5
(server)  (static)  (keygen)  (sign)    (upload UI)
                                            ↓
Step 6  → Step 7  → Step 8  → Step 9  → Step 10 → Step 11 → Step 12
(edit)    (prove)   (edit UI)  (verify)  (verify UI) (polish)  (deploy)
```

Each step is independently testable. No step depends on a later step.
