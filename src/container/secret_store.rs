use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chacha20poly1305::aead::{Aead, KeyInit, OsRng};
use chacha20poly1305::{AeadCore, ChaCha20Poly1305, Key, Nonce};
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const ED25519_KEY_LEN: usize = 32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: String,
    pub email: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub public_key: String,
    pub private_key: Box<[u8; ED25519_KEY_LEN]>,
    pub public_key_pem: String,
    pub encryption_public_key: Option<String>,
    pub encryption_private_key: Option<Box<[u8; ED25519_KEY_LEN]>>,
    pub encryption_public_key_pem: Option<String>,
    pub algorithm: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharingTier {
    Live,
    Cached,
    Delegated,
}

impl Default for SharingTier {
    fn default() -> Self {
        Self::Live
    }
}

pub struct MultiTenantSecretStore {
    base_path: PathBuf,
    enabled: bool,
    identity: Option<Identity>,
    integrations: HashMap<String, Integration>,
    #[allow(dead_code)]
    cipher: Option<ChaCha20Poly1305>,
}

impl MultiTenantSecretStore {
    pub fn new(zeroclaw_dir: &std::path::Path, enabled: bool) -> Self {
        let base_path = zeroclaw_dir.join("vault");
        Self {
            base_path,
            enabled,
            identity: None,
            integrations: HashMap::new(),
            cipher: None,
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if let Some(parent) = self.base_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let identity_path = self.base_path.join("identity.json");
        if identity_path.exists() {
        } else {
            let identity = self.generate_identity_keypairs()?;
            fs::write(&identity_path, serde_json::to_string_pretty(&identity)?)?;
            self.identity = Some(identity);
        }

        self.load_integrations()?;

        let key_bytes = self.load_or_create_key()?;
        self.cipher = Some(ChaCha20Poly1305::new(Key::from_slice(&key_bytes)));

        Ok(())
    }

    fn generate_identity_keypairs(&self) -> Result<Identity> {
        let mut private_key_bytes = [0u8; 32];
        getrandom::getrandom(&mut private_key_bytes)
            .map_err(|e| anyhow::anyhow!("RNG failed: {}", e))?;
        let signing_key = SigningKey::from_bytes(&private_key_bytes);
        let verifying_key = signing_key.verifying_key();

        let mut encryption_secret_bytes = [0u8; 32];
        getrandom::getrandom(&mut encryption_secret_bytes)
            .map_err(|e| anyhow::anyhow!("RNG failed: {}", e))?;
        let encryption_secret = StaticSecret::from(encryption_secret_bytes);
        let encryption_public = X25519PublicKey::from(&encryption_secret);

        Ok(Identity {
            public_key: STANDARD.encode(verifying_key.as_bytes()),
            private_key: Box::new(private_key_bytes),
            public_key_pem: format_pem_public_key(verifying_key.as_bytes()),
            encryption_public_key: Some(STANDARD.encode(encryption_public.as_bytes())),
            encryption_private_key: Some(Box::new(encryption_secret_bytes)),
            encryption_public_key_pem: Some(format_pem_x25519_public_key(
                encryption_public.as_bytes(),
            )),
            algorithm: "Ed25519",
        })
    }

    fn load_integrations(&mut self) -> Result<()> {
        let integrations_path = self.base_path.join("integrations.json");
        if integrations_path.exists() {
            let data = fs::read_to_string(&integrations_path)?;
            self.integrations = serde_json::from_str(&data)?;
        }
        Ok(())
    }

    fn save_integrations(&self) -> Result<()> {
        let integrations_path = self.base_path.join("integrations.json");
        fs::write(
            &integrations_path,
            serde_json::to_string_pretty(&self.integrations)?,
        )?;
        Ok(())
    }

    #[inline]
    pub fn set_integration(&mut self, name: &str, integration: Integration) -> Result<()> {
        self.integrations.insert(name.to_string(), integration);
        self.save_integrations()
    }

    #[inline]
    pub fn get_integration(&self, name: &str) -> Option<&Integration> {
        self.integrations.get(name)
    }

    #[inline]
    pub fn list_integrations(&self) -> Vec<&String> {
        self.integrations.keys().collect()
    }

    #[inline]
    pub fn get_identity(&self) -> Option<&Identity> {
        self.identity.as_ref()
    }

    #[inline]
    pub fn encrypt_data(&self, plaintext: &[u8]) -> Result<String> {
        if !self.enabled {
            return Ok(STANDARD.encode(plaintext));
        }

        let key_bytes = self.load_or_create_key()?;
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {e}"))?;

        let mut blob = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        blob.extend_from_slice(&nonce);
        blob.extend_from_slice(&ciphertext);

        Ok(STANDARD.encode(&blob))
    }

    #[inline]
    pub fn decrypt_data(&self, value: &str) -> Result<Vec<u8>> {
        if !self.enabled {
            return STANDARD.decode(value).context("Base64 decode failed");
        }

        let blob = STANDARD.decode(value).context("Base64 decode failed")?;
        anyhow::ensure!(blob.len() > NONCE_LEN, "Encrypted value too short");

        let (nonce_bytes, ciphertext) = blob.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);
        let key_bytes = self.load_or_create_key()?;
        let key = Key::from_slice(&key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))
    }

    fn load_or_create_key(&self) -> Result<[u8; KEY_LEN]> {
        let key_path = self.base_path.join(".vault_key");
        if key_path.exists() {
            let hex_key = fs::read_to_string(&key_path)?;
            let decoded = STANDARD
                .decode(hex_key.trim())
                .context("Key decode failed")?;
            let mut key = [0u8; KEY_LEN];
            key.copy_from_slice(&decoded);
            Ok(key)
        } else {
            let mut key = [0u8; KEY_LEN];
            getrandom::getrandom(&mut key).map_err(|e| anyhow::anyhow!("RNG failed: {}", e))?;
            fs::write(&key_path, STANDARD.encode(&key))?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&key_path, fs::Permissions::from_mode(0o600))?;
            }
            Ok(key)
        }
    }
}

#[inline]
fn format_pem_public_key(key: &[u8; ED25519_KEY_LEN]) -> String {
    let der = format_ed25519_der(key);
    let b64 = STANDARD.encode(&der);
    format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
        b64
    )
}

#[inline]
fn format_pem_x25519_public_key(key: &[u8; 32]) -> String {
    let der = format_x25519_der(key);
    let b64 = STANDARD.encode(&der);
    format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----",
        b64
    )
}

#[inline]
fn format_ed25519_der(key: &[u8; ED25519_KEY_LEN]) -> Vec<u8> {
    let prefix: [u8; 12] = [
        0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x70, 0x03, 0x21, 0x00,
    ];
    let mut der = Vec::with_capacity(12 + ED25519_KEY_LEN);
    der.extend_from_slice(&prefix);
    der.extend_from_slice(key);
    der
}

#[inline]
fn format_x25519_der(key: &[u8; 32]) -> Vec<u8> {
    let prefix: [u8; 12] = [
        0x30, 0x2a, 0x30, 0x05, 0x06, 0x03, 0x2b, 0x65, 0x6e, 0x03, 0x21, 0x00,
    ];
    let mut der = Vec::with_capacity(12 + 32);
    der.extend_from_slice(&prefix);
    der.extend_from_slice(key);
    der
}
