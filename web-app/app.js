// ===== Brevis Vera Web App =====
const API = '';  // Same-origin

// ===== State =====
let state = {
    sessionId: null,
    rootCaHash: null,
    signedPhotoReady: false,
    editedReady: false,
    proofReady: false,
    verifyImageFile: null,
    verifyProofFile: null,
};

// ===== Tab Navigation =====
document.querySelectorAll('.tab-btn').forEach(btn => {
    btn.addEventListener('click', () => {
        const tab = btn.dataset.tab;
        switchTab(tab);
    });
});

function switchTab(tab) {
    document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
    document.querySelectorAll('.tab-panel').forEach(p => p.classList.remove('active'));
    document.getElementById(`tab-${tab}`).classList.add('active');
    document.getElementById(`panel-${tab}`).classList.add('active');
}

// ===== Upload & Sign =====
const dropZone = document.getElementById('dropZone');
const fileInput = document.getElementById('fileInput');
const previewImg = document.getElementById('previewImg');
const uploadPreview = document.getElementById('uploadPreview');
const imageInfo = document.getElementById('imageInfo');
const signBtn = document.getElementById('signBtn');

let uploadedFile = null;

dropZone.addEventListener('click', () => fileInput.click());
dropZone.addEventListener('dragover', e => { e.preventDefault(); dropZone.classList.add('drag-over'); });
dropZone.addEventListener('dragleave', () => dropZone.classList.remove('drag-over'));
dropZone.addEventListener('drop', e => {
    e.preventDefault();
    dropZone.classList.remove('drag-over');
    if (e.dataTransfer.files.length) handleUploadFile(e.dataTransfer.files[0]);
});
fileInput.addEventListener('change', () => {
    if (fileInput.files.length) handleUploadFile(fileInput.files[0]);
});

function handleUploadFile(file) {
    uploadedFile = file;
    const url = URL.createObjectURL(file);
    previewImg.src = url;
    uploadPreview.style.display = 'block';
    dropZone.style.display = 'none';

    const img = new Image();
    img.onload = () => {
        imageInfo.textContent = `${img.width} × ${img.height} • ${(file.size / 1024 / 1024).toFixed(1)} MB • ${file.name}`;
    };
    img.src = url;

    signBtn.disabled = false;
}

signBtn.addEventListener('click', async () => {
    if (!uploadedFile) return;
    signBtn.disabled = true;
    signBtn.innerHTML = '<span class="btn-icon" style="animation: spin 1s linear infinite">⏳</span> Generating keys & signing...';

    try {
        // Step 1: Keygen
        const keyRes = await fetch(`${API}/api/keygen`, { method: 'POST' });
        if (!keyRes.ok) {
            const errText = await keyRes.text();
            throw new Error(`Keygen failed (${keyRes.status}): ${errText || keyRes.statusText}`);
        }
        const keyData = await keyRes.json();
        state.sessionId = keyData.session_id;
        state.rootCaHash = keyData.root_ca_hash;

        // Step 2: Sign
        const formData = new FormData();
        formData.append('session_id', state.sessionId);
        formData.append('image', uploadedFile);

        const signRes = await fetch(`${API}/api/sign`, { method: 'POST', body: formData });
        if (!signRes.ok) {
            const errText = await signRes.text();
            throw new Error(`Sign failed (${signRes.status}): ${errText || signRes.statusText}`);
        }
        const signData = await signRes.json();

        // Show results
        document.getElementById('rootCaHash').textContent = keyData.root_ca_hash;
        document.getElementById('devicePkHash').textContent = keyData.device_pubkey_hash;
        document.getElementById('numShards').textContent = `${signData.num_shards} shards • ${signData.width}×${signData.height}`;
        document.getElementById('signResults').style.display = 'flex';

        state.signedPhotoReady = true;

        signBtn.innerHTML = '<span class="btn-icon">✅</span> Signed Successfully';
    } catch (err) {
        signBtn.innerHTML = '<span class="btn-icon">❌</span> Error: ' + err.message;
        signBtn.disabled = false;
    }
});

document.getElementById('goToEditBtn').addEventListener('click', () => {
    if (state.signedPhotoReady) {
        switchTab('edit');
        loadEditPreview();
    }
});

// ===== Edit & Prove =====
function loadEditPreview() {
    if (!state.sessionId) return;
    document.getElementById('editPreviewImg').src = `${API}/api/data/${state.sessionId}/original_preview`;
    document.getElementById('editContainer').style.display = 'grid';
    document.getElementById('proveSection').style.display = 'block';
}

// Crop toggle
document.getElementById('cropEnable').addEventListener('change', (e) => {
    document.getElementById('cropInputs').style.display = e.target.checked ? 'grid' : 'none';
});

// Brightness toggle
document.getElementById('brightnessEnable').addEventListener('change', (e) => {
    document.getElementById('brightnessInputs').style.display = e.target.checked ? 'flex' : 'none';
});

// Brightness slider
document.getElementById('brightnessSlider').addEventListener('input', (e) => {
    document.getElementById('brightnessValue').textContent = e.target.value;
});

// Apply edits
document.getElementById('applyEditBtn').addEventListener('click', async () => {
    const ops = buildOpsString();
    const applyBtn = document.getElementById('applyEditBtn');
    applyBtn.disabled = true;
    applyBtn.textContent = 'Applying...';

    try {
        const res = await fetch(`${API}/api/edit`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ session_id: state.sessionId, ops })
        });
        const data = await res.json();

        document.getElementById('editedImg').src = `${API}${data.edited_image_url}?t=${Date.now()}`;
        document.getElementById('editedPreview').style.display = 'block';
        state.editedReady = true;

        applyBtn.textContent = '✅ Edits Applied';
    } catch (err) {
        applyBtn.textContent = '❌ Error';
        applyBtn.disabled = false;
    }
});

function buildOpsString() {
    const parts = [];
    if (document.getElementById('cropEnable').checked) {
        const x = document.getElementById('cropX').value || 0;
        const y = document.getElementById('cropY').value || 0;
        const w = document.getElementById('cropW').value || 100;
        const h = document.getElementById('cropH').value || 100;
        parts.push(`crop:${x},${y},${w},${h}`);
    }
    if (document.getElementById('brightnessEnable').checked) {
        const v = document.getElementById('brightnessSlider').value;
        if (v != 0) parts.push(`brightness:${v}`);
    }
    return parts.join(';');
}

// Prove
document.getElementById('proveBtn').addEventListener('click', async () => {
    const proveBtn = document.getElementById('proveBtn');
    const progressBar = document.getElementById('progressBar');
    const progressFill = document.getElementById('progressFill');
    const progressText = document.getElementById('progressText');
    const realProof = document.getElementById('realProofToggle').checked;

    proveBtn.disabled = true;
    proveBtn.innerHTML = '<span class="btn-icon" style="animation: spin 1s linear infinite">⏳</span> Starting proof...';
    progressBar.style.display = 'block';

    try {
        // Start proving
        const res = await fetch(`${API}/api/prove`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ session_id: state.sessionId, real_proof: realProof })
        });
        const { job_id } = await res.json();

        // Poll status
        let done = false;
        while (!done) {
            await sleep(1000);
            const statusRes = await fetch(`${API}/api/prove/status/${job_id}`);
            const statusData = await statusRes.json();

            if (statusData.progress !== undefined) {
                progressFill.style.width = `${statusData.progress}%`;
                const pct = statusData.progress;
                progressText.textContent = statusData.message
                    ? `${statusData.message} (${pct}%)`
                    : `${pct}%`;
            }

            if (statusData.status === 'done') {
                done = true;
                showProveResults(statusData);
            } else if (statusData.status === 'error') {
                throw new Error(statusData.error || 'Proving failed');
            }
        }
    } catch (err) {
        proveBtn.innerHTML = '<span class="btn-icon">❌</span> Error: ' + err.message;
        proveBtn.disabled = false;
    }
});

function showProveResults(data) {
    const proveBtn = document.getElementById('proveBtn');
    const progressBar = document.getElementById('progressBar');

    proveBtn.innerHTML = '<span class="btn-icon">✅</span> Proof Generated';
    progressBar.style.display = 'none';

    document.getElementById('proofType').textContent = data.proof_type || 'Emulated';
    document.getElementById('origHash').textContent = data.public_values?.original_image_hash || '';
    document.getElementById('editedHash').textContent = data.public_values?.output_image_hash || '';
    document.getElementById('proveTime').textContent = `${(data.proving_time_ms / 1000).toFixed(1)}s`;

    if (data.proof_download_url) {
        const dlBtn = document.getElementById('downloadProofBtn');
        dlBtn.href = `${API}${data.proof_download_url}`;
    }

    document.getElementById('proveResults').style.display = 'flex';
    state.proofReady = true;
}

document.getElementById('goToVerifyBtn').addEventListener('click', () => switchTab('verify'));

// ===== Verify =====
const verifyImageDrop = document.getElementById('verifyImageDrop');
const verifyProofDrop = document.getElementById('verifyProofDrop');
const verifyImageInput = document.getElementById('verifyImageInput');
const verifyProofInput = document.getElementById('verifyProofInput');
const verifyBtn = document.getElementById('verifyBtn');

// Image drop
setupDropBox(verifyImageDrop, verifyImageInput, (file) => {
    state.verifyImageFile = file;
    document.getElementById('verifyImageStatus').textContent = `✅ ${file.name}`;
    verifyImageDrop.classList.add('has-file');
    checkVerifyReady();
});

// Proof drop
setupDropBox(verifyProofDrop, verifyProofInput, (file) => {
    state.verifyProofFile = file;
    document.getElementById('verifyProofStatus').textContent = `✅ ${file.name}`;
    verifyProofDrop.classList.add('has-file');
    checkVerifyReady();
});

function setupDropBox(dropEl, inputEl, onFile) {
    dropEl.addEventListener('click', () => inputEl.click());
    dropEl.addEventListener('dragover', e => { e.preventDefault(); dropEl.classList.add('drag-over'); });
    dropEl.addEventListener('dragleave', () => dropEl.classList.remove('drag-over'));
    dropEl.addEventListener('drop', e => {
        e.preventDefault();
        dropEl.classList.remove('drag-over');
        if (e.dataTransfer.files.length) onFile(e.dataTransfer.files[0]);
    });
    inputEl.addEventListener('change', () => {
        if (inputEl.files.length) onFile(inputEl.files[0]);
    });
}

function checkVerifyReady() {
    verifyBtn.disabled = !(state.verifyImageFile && state.verifyProofFile);
}

verifyBtn.addEventListener('click', async () => {
    if (!state.verifyImageFile || !state.verifyProofFile) return;

    verifyBtn.disabled = true;
    verifyBtn.innerHTML = '<span class="btn-icon" style="animation: spin 1s linear infinite">⏳</span> Verifying...';

    try {
        const formData = new FormData();
        formData.append('image', state.verifyImageFile);
        formData.append('proof_package', state.verifyProofFile);
        const trustedKey = document.getElementById('trustedKeyInput').value.trim();
        if (trustedKey) formData.append('trusted_root_ca_hash', trustedKey);

        const res = await fetch(`${API}/api/verify`, { method: 'POST', body: formData });
        const data = await res.json();

        if (!res.ok) {
            throw new Error(data.error || `Verification failed (${res.status})`);
        }

        showVerdict(data);
    } catch (err) {
        verifyBtn.innerHTML = '<span class="btn-icon">❌</span> Error: ' + err.message;
        verifyBtn.disabled = false;
    }
});

function showVerdict(data) {
    verifyBtn.innerHTML = '<span class="btn-icon">🔍</span> Verify Authenticity';
    verifyBtn.disabled = false;

    const checksEl = document.getElementById('verdictChecks');
    const bannerEl = document.getElementById('verdictBanner');
    const historyEl = document.getElementById('editHistory');

    checksEl.innerHTML = '';

    // Image hash check
    addCheck(checksEl, data.image_hash_match,
        data.image_hash_match ? 'Image Hash matches ZK commitment' : 'Image Hash MISMATCH!');

    // Manufacturer check
    if (data.manufacturer_trusted !== undefined) {
        addCheck(checksEl, data.manufacturer_trusted,
            data.manufacturer_trusted ? 'Manufacturer (Root CA) is trusted' : 'Signer is UNTRUSTED!');
    }

    // Proof check
    if (data.stark_verified) {
        addCheck(checksEl, true, 'ZK Proof Verified (Pico STARK)');
    } else if (data.emulated) {
        addCheck(checksEl, true, 'ZK Proof Verified (Pico EMULATED)');
    }

    // Banner
    const success = data.verdict === 'VERIFICATION_SUCCESSFUL';
    bannerEl.className = `verdict-banner ${success ? 'success' : 'fail'}`;
    bannerEl.textContent = success
        ? '🏆 VERIFICATION SUCCESSFUL'
        : '🚫 VERIFICATION FAILED';

    // Edit history (filter out internal labels like "ParallelAOT")
    if (data.edit_types && data.edit_types.length) {
        const userVisible = data.edit_types.filter(t => t !== 'ParallelAOT');
        if (userVisible.length) {
            historyEl.textContent = `Edit History: Origin → ${userVisible.join(' → ')}`;
        } else {
            historyEl.textContent = '';
        }
    }

    document.getElementById('verdictContainer').style.display = 'block';
}

function addCheck(container, pass, text) {
    const div = document.createElement('div');
    div.className = `verdict-check ${pass ? 'pass' : 'fail'}`;
    div.innerHTML = `<span class="check-icon">${pass ? '✅' : '❌'}</span><span>${text}</span>`;
    container.appendChild(div);
}

// ===== Utilities =====
function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }
