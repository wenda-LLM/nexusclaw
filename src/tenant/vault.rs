use anyhow::{bail, Context, Result};
use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, Nonce};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const ENCRYPTION_KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultEntry {
    pub id: String,
    pub tenant_id: String,
    pub user_id: String,
    pub name: String,
    pub credential_type: String,
    pub encrypted_value: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VaultStoreData {
    pub entries: Vec<VaultEntry>,
    pub version: u32,
}

pub struct VaultStore {
    data: VaultStoreData,
    path: std::path::PathBuf,
    encryption_key: [u8; ENCRYPTION_KEY_LEN],
    cipher: ChaCha20Poly1305,
}

impl VaultStore {
    pub fn new(base_path: &Path, encryption_key: &[u8; ENCRYPTION_KEY_LEN]) -> Self {
        let path = base_path.join("vault_store.json");
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            VaultStoreData::default()
        };
        let cipher = ChaCha20Poly1305::new(Key::from_slice(encryption_key));
        Self {
            data,
            path,
            encryption_key: *encryption_key,
            cipher,
        }
    }

    pub fn create(encryption_key: &[u8; ENCRYPTION_KEY_LEN]) -> Self {
        Self::new(std::path::Path::new("."), encryption_key)
    }

    pub fn set_encryption_key(&mut self, key: [u8; ENCRYPTION_KEY_LEN]) {
        self.encryption_key = key;
        self.cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    }

    pub fn store(
        &mut self,
        tenant_id: String,
        user_id: String,
        name: String,
        credential_type: String,
        plaintext: &str,
        metadata: Option<HashMap<String, serde_json::Value>>,
        expires_at: Option<String>,
    ) -> Result<VaultEntry> {
        let encrypted_value = self.encrypt(plaintext)?;
        let now = chrono::Utc::now().to_rfc3339();

        if let Some(existing) = self
            .data
            .entries
            .iter_mut()
            .find(|e| e.tenant_id == tenant_id && e.user_id == user_id && e.name == name)
        {
            existing.encrypted_value = encrypted_value;
            existing.metadata = metadata;
            existing.updated_at = now.clone();
            existing.expires_at = expires_at;
            let result = existing.clone();
            self.save()?;
            return Ok(result);
        }

        let entry = VaultEntry {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id,
            user_id,
            name,
            credential_type,
            encrypted_value,
            metadata,
            created_at: now.clone(),
            updated_at: now,
            expires_at,
        };
        self.data.entries.push(entry.clone());
        self.save()?;
        Ok(entry)
    }

    #[inline]
    pub fn get(&self, id: &str) -> Option<&VaultEntry> {
        self.data.entries.iter().find(|e| e.id == id)
    }

    #[inline]
    pub fn get_by_user(&self, user_id: &str) -> Vec<&VaultEntry> {
        self.data
            .entries
            .iter()
            .filter(|e| e.user_id == user_id)
            .collect()
    }

    #[inline]
    pub fn get_by_tenant(&self, tenant_id: &str) -> Vec<&VaultEntry> {
        self.data
            .entries
            .iter()
            .filter(|e| e.tenant_id == tenant_id)
            .collect()
    }

    pub fn get_decrypted(&self, id: &str) -> Result<String> {
        let entry = self.get(id).context("Entry not found")?;
        self.decrypt(&entry.encrypted_value)
    }

    pub fn delete(&mut self, id: &str) -> Result<()> {
        self.data.entries.retain(|e| e.id != id);
        self.save()
    }

    pub fn delete_by_user(&mut self, user_id: &str) -> Result<()> {
        self.data.entries.retain(|e| e.user_id != user_id);
        self.save()
    }

    #[inline]
    fn encrypt(&self, plaintext: &str) -> Result<String> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {e}"))?;

        let mut blob = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        blob.extend_from_slice(&nonce);
        blob.extend_from_slice(&ciphertext);

        use base64::{engine::general_purpose::STANDARD, Engine as _};
        Ok(STANDARD.encode(&blob))
    }

    #[inline]
    fn decrypt(&self, value: &str) -> Result<String> {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let blob = STANDARD.decode(value).context("Base64 decode failed")?;
        anyhow::ensure!(blob.len() > NONCE_LEN, "Encrypted value too short");

        let (nonce_bytes, ciphertext) = blob.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))?;

        String::from_utf8(plaintext).context("Decrypted value is not valid UTF-8")
    }

    #[inline]
    fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, serde_json::to_string(&self.data)?)?;
        Ok(())
    }
}

pub fn derive_key(password: &str, salt: &[u8]) -> [u8; ENCRYPTION_KEY_LEN] {
    use argon2::{Argon2, PasswordHasher};
    let mut output = [0u8; ENCRYPTION_KEY_LEN];
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut output)
        .expect("Key derivation failed");
    output
}
