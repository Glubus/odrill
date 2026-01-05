/**
 * Settings Page JavaScript
 */

// ============================================
// Section Navigation
// ============================================

function initNavigation() {
    document.querySelectorAll('.settings-nav a').forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const section = link.dataset.section;
            
            document.querySelectorAll('.settings-nav a').forEach(l => l.classList.remove('active'));
            document.querySelectorAll('.settings-section').forEach(s => s.classList.remove('active'));
            
            link.classList.add('active');
            document.getElementById(section).classList.add('active');
            
            history.pushState(null, '', `#${section}`);
        });
    });

    // Handle initial hash
    if (window.location.hash) {
        const section = window.location.hash.slice(1);
        const link = document.querySelector(`[data-section="${section}"]`);
        if (link) link.click();
    }
}

// ============================================
// Avatar Upload & Cropper
// ============================================

let cropper = null;
let currentFile = null;

function initAvatarUpload(userPid) {
    const avatarInput = document.getElementById('avatarInput');
    const modal = document.getElementById('cropperModal');
    const imageToCrop = document.getElementById('imageToCrop');

    if (!avatarInput) return;

    avatarInput.addEventListener('change', function(e) {
        if (e.target.files && e.target.files.length > 0) {
            currentFile = e.target.files[0];
            const reader = new FileReader();
            reader.onload = function(evt) {
                imageToCrop.src = evt.target.result;
                openCropper();
            };
            reader.readAsDataURL(currentFile);
        }
    });

    window.openCropper = function() {
        modal.classList.add('active');
        if (cropper) cropper.destroy();
        cropper = new Cropper(imageToCrop, {
            aspectRatio: 1,
            viewMode: 1,
            autoCropArea: 0.8,
            minCropBoxWidth: 50,
            minCropBoxHeight: 50,
        });
    };

    window.closeCropper = function() {
        modal.classList.remove('active');
        if (cropper) { cropper.destroy(); cropper = null; }
        avatarInput.value = '';
    };

    window.uploadCropped = async function() {
        if (!cropper) return;
        const data = cropper.getData(true);
        const formData = new FormData();
        formData.append('file', currentFile);
        formData.append('x', data.x);
        formData.append('y', data.y);
        formData.append('width', data.width);
        formData.append('height', data.height);

        try {
            const response = await fetch('/api/user/avatar', { method: 'POST', body: formData });
            if (response.ok) {
                document.getElementById('currentAvatar').src = `/uploads/avatars/${userPid}.webp?t=${Date.now()}`;
                closeCropper();
            } else {
                alert('Failed to upload avatar');
            }
        } catch (error) {
            console.error(error);
            alert('Error uploading avatar');
        }
    };
}

// ============================================
// API Keys
// ============================================

async function loadApiKeys() {
    try {
        const response = await fetch('/api/user/api-keys');
        const keys = await response.json();
        renderApiKeys(keys);
    } catch (error) {
        console.error('Failed to load API keys:', error);
        document.getElementById('apiKeysList').innerHTML = '<p style="color: var(--error);">Failed to load keys</p>';
    }
}

function renderApiKeys(keys) {
    const container = document.getElementById('apiKeysList');
    if (!container) return;
    
    if (keys.length === 0) {
        container.innerHTML = '<p style="color: var(--text-muted);">No API keys yet. Create one to get started.</p>';
        return;
    }
    container.innerHTML = keys.map(key => `
        <div class="api-key-card">
            <div class="api-key-info">
                <h4>${key.name}</h4>
                <p>Created ${new Date(key.created_at).toLocaleDateString()} • Used ${key.usage_count} times</p>
                <div class="api-key-permissions">
                    ${(Array.isArray(key.permissions) ? key.permissions : []).map(p => `<span>${p}</span>`).join('')}
                </div>
            </div>
            <button class="btn btn-secondary btn-sm" onclick="revokeKey(${key.id})" style="color: var(--error);">
                <i data-lucide="trash-2"></i>
            </button>
        </div>
    `).join('');
    lucide.createIcons();
}

window.revokeKey = async function(id) {
    if (!confirm('Are you sure you want to revoke this API key?')) return;
    try {
        await fetch(`/api/user/api-keys/${id}`, { method: 'DELETE' });
        loadApiKeys();
    } catch (error) {
        console.error('Failed to revoke key:', error);
        alert('Failed to revoke key');
    }
};

window.showCreateKeyModal = function() {
    document.getElementById('createKeyModal').classList.add('active');
};

window.closeCreateKeyModal = function() {
    document.getElementById('createKeyModal').classList.remove('active');
    document.getElementById('createKeyForm').reset();
    document.getElementById('expireValueGroup').style.display = 'none';
};

window.copyNewKey = function() {
    const keyValue = document.getElementById('newKeyValue').textContent;
    navigator.clipboard.writeText(keyValue).then(() => {
        // Visual feedback could be added here
    });
};

function initCreateKeyForm() {
    const expireOnSelect = document.getElementById('expireOn');
    const form = document.getElementById('createKeyForm');
    
    if (!expireOnSelect || !form) return;

    expireOnSelect.addEventListener('change', (e) => {
        const group = document.getElementById('expireValueGroup');
        const label = document.getElementById('expireValueLabel');
        if (e.target.value === 'Never') {
            group.style.display = 'none';
        } else {
            group.style.display = 'block';
            label.textContent = e.target.value === 'Date' ? 'Days until expiry' : 'Maximum uses';
        }
    });

    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        const name = document.getElementById('keyName').value;
        const permissions = Array.from(document.querySelectorAll('input[name="permissions"]:checked')).map(c => c.value);
        const expireOn = document.getElementById('expireOn').value;
        let expireValue = null;
        
        if (expireOn === 'Date') {
            const days = parseInt(document.getElementById('expireValue').value) || 30;
            expireValue = Math.floor(Date.now() / 1000) + (days * 86400);
        } else if (expireOn === 'Usage') {
            expireValue = parseInt(document.getElementById('expireValue').value) || 100;
        }

        try {
            const response = await fetch('/api/user/api-keys', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name, permissions, expire_on: expireOn, expire_value: expireValue })
            });
            const result = await response.json();
            
            if (result.key) {
                document.getElementById('newKeyValue').textContent = result.key;
                document.getElementById('newKeyDisplay').style.display = 'block';
                lucide.createIcons();
            }
            
            closeCreateKeyModal();
            loadApiKeys();
        } catch (error) {
            console.error('Failed to create key:', error);
            alert('Failed to create key');
        }
    });
}

// ============================================
// Delete Account
// ============================================

function initDeleteAccount() {
    const confirmInput = document.getElementById('deleteConfirm');
    const deleteBtn = document.getElementById('deleteAccountBtn');
    
    if (!confirmInput || !deleteBtn) return;

    confirmInput.addEventListener('input', (e) => {
        deleteBtn.disabled = e.target.value !== 'DELETE MY ACCOUNT';
    });
}

// ============================================
// Initialize
// ============================================

function initSettings(userPid) {
    initNavigation();
    initAvatarUpload(userPid);
    initCreateKeyForm();
    initDeleteAccount();
    loadApiKeys();
    lucide.createIcons();
}
