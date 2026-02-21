use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCredential {
    pub name: String,
    pub credential_type: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

pub trait CredentialProvider: Send + Sync {
    fn get_credential(&self, name: &str) -> Option<&AgentCredential>;
    fn list_credentials(&self) -> Vec<&AgentCredential>;
    fn store_credential(&mut self, credential: AgentCredential) -> Result<()>;
    fn remove_credential(&mut self, name: &str) -> Result<()>;
}

pub struct LocalCredentialProvider {
    credentials: HashMap<String, AgentCredential>,
    path: std::path::PathBuf,
}

impl LocalCredentialProvider {
    #[inline]
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("agent_credentials.json");
        let credentials = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Self { credentials, path }
    }

    #[inline]
    pub fn store(&mut self, credential: AgentCredential) -> Result<()> {
        self.credentials.insert(credential.name.clone(), credential);
        self.save()
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<&AgentCredential> {
        self.credentials.get(name)
    }

    #[inline]
    pub fn remove(&mut self, name: &str) -> Result<()> {
        self.credentials.remove(name);
        self.save()
    }

    #[inline]
    pub fn list(&self) -> Vec<&AgentCredential> {
        self.credentials.values().collect()
    }

    #[inline]
    pub fn is_expired(&self, name: &str) -> bool {
        if let Some(cred) = self.credentials.get(name) {
            if let Some(expires_at) = &cred.expires_at {
                if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(expires_at) {
                    return expires < chrono::Utc::now();
                }
            }
        }
        false
    }

    #[inline]
    fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, serde_json::to_string(&self.credentials)?)?;
        Ok(())
    }
}

impl CredentialProvider for LocalCredentialProvider {
    #[inline]
    fn get_credential(&self, name: &str) -> Option<&AgentCredential> {
        self.get(name)
    }

    #[inline]
    fn list_credentials(&self) -> Vec<&AgentCredential> {
        self.list()
    }

    #[inline]
    fn store_credential(&mut self, credential: AgentCredential) -> Result<()> {
        self.store(credential)
    }

    #[inline]
    fn remove_credential(&mut self, name: &str) -> Result<()> {
        self.remove(name)
    }
}
