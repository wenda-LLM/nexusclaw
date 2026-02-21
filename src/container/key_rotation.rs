use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedIdentity {
    pub key_id: String,
    pub public_key: String,
    pub created_at: String,
    pub archived: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivedKey {
    pub key_id: String,
    pub public_key: String,
    pub archived_at: String,
    pub private_key_encrypted: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyRotationState {
    pub current_key_id: String,
    pub identities: Vec<VersionedIdentity>,
    pub archived_keys: Vec<ArchivedKey>,
    pub last_rotation: String,
}

pub struct KeyRotationManager {
    state: KeyRotationState,
    path: std::path::PathBuf,
}

impl KeyRotationManager {
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("key_rotation.json");
        let state = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            KeyRotationState::default()
        };
        Self { state, path }
    }

    pub fn generate_versioned_identity(&mut self, public_key: String) -> Result<String> {
        let key_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let identity = VersionedIdentity {
            key_id: key_id.clone(),
            public_key,
            created_at: now,
            archived: false,
        };

        self.state.identities.push(identity);
        self.state.current_key_id = key_id;
        self.save()?;

        Ok(self.state.current_key_id.clone())
    }

    pub fn get_current_identity(&self) -> Option<&VersionedIdentity> {
        self.state
            .identities
            .iter()
            .find(|i| i.key_id == self.state.current_key_id && !i.archived)
    }

    pub fn rotate_keys(
        &mut self,
        new_public_key: String,
        new_private_key_encrypted: String,
    ) -> Result<String> {
        let current_key = self.get_current_identity().map(|i| i.public_key.clone());

        if let Some(current) = self
            .state
            .identities
            .iter_mut()
            .find(|i| i.key_id == self.state.current_key_id)
        {
            current.archived = true;
        }

        let new_key_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let archived = ArchivedKey {
            key_id: self.state.current_key_id.clone(),
            public_key: current_key.unwrap_or_default(),
            archived_at: now.clone(),
            private_key_encrypted: new_private_key_encrypted,
        };
        self.state.archived_keys.push(archived);

        let now2 = chrono::Utc::now().to_rfc3339();
        let identity = VersionedIdentity {
            key_id: new_key_id.clone(),
            public_key: new_public_key,
            created_at: now2.clone(),
            archived: false,
        };

        self.state.identities.push(identity);
        self.state.current_key_id = new_key_id.clone();
        self.state.last_rotation = now2;

        self.save()?;
        Ok(new_key_id)
    }

    pub fn list_identities(&self) -> Vec<&VersionedIdentity> {
        self.state.identities.iter().collect()
    }

    pub fn get_identity(&self, key_id: &str) -> Option<&VersionedIdentity> {
        self.state.identities.iter().find(|i| i.key_id == key_id)
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, serde_json::to_string(&self.state)?)?;
        Ok(())
    }
}

pub fn generate_key_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
