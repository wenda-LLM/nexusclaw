use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupVaultEntry {
    pub id: String,
    pub group_id: String,
    pub name: String,
    pub encrypted_value: String,
    pub created_by: String,
    pub created_at: String,
    pub threshold: u8,
    pub approvals: Vec<Approval>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub user_id: String,
    pub approved_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupVaultData {
    pub entries: Vec<GroupVaultEntry>,
}

pub struct GroupVaultStore {
    data: GroupVaultData,
    path: std::path::PathBuf,
}

impl GroupVaultStore {
    #[inline]
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("group_vault.json");
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            GroupVaultData::default()
        };
        Self { data, path }
    }

    pub fn create(
        &mut self,
        group_id: String,
        name: String,
        encrypted_value: String,
        created_by: String,
        threshold: u8,
    ) -> Result<GroupVaultEntry> {
        let entry = GroupVaultEntry {
            id: uuid::Uuid::new_v4().to_string(),
            group_id,
            name,
            encrypted_value,
            created_by,
            created_at: chrono::Utc::now().to_rfc3339(),
            threshold,
            approvals: Vec::new(),
        };
        self.data.entries.push(entry.clone());
        self.save()?;
        Ok(entry)
    }

    #[inline]
    pub fn get(&self, id: &str) -> Option<&GroupVaultEntry> {
        self.data.entries.iter().find(|e| e.id == id)
    }

    #[inline]
    pub fn list_by_group(&self, group_id: &str) -> Vec<&GroupVaultEntry> {
        self.data
            .entries
            .iter()
            .filter(|e| e.group_id == group_id)
            .collect()
    }

    pub fn approve(&mut self, entry_id: &str, user_id: String) -> Result<bool> {
        let approval_count = {
            let entry = self
                .data
                .entries
                .iter_mut()
                .find(|e| e.id == entry_id)
                .ok_or_else(|| anyhow::anyhow!("Entry not found"))?;
            if !entry.approvals.iter().any(|a| a.user_id == user_id) {
                entry.approvals.push(Approval {
                    user_id,
                    approved_at: chrono::Utc::now().to_rfc3339(),
                });
            }
            entry.approvals.len()
        };

        let threshold = {
            if let Some(entry) = self.data.entries.iter_mut().find(|e| e.id == entry_id) {
                entry.threshold
            } else {
                return Ok(false);
            }
        };

        self.save()?;
        Ok(approval_count >= threshold as usize)
    }

    #[inline]
    pub fn is_unlocked(&self, entry_id: &str) -> bool {
        self.get(entry_id)
            .map(|e| e.approvals.len() >= e.threshold as usize)
            .unwrap_or(false)
    }

    pub fn delete(&mut self, id: &str) -> Result<()> {
        self.data.entries.retain(|e| e.id != id);
        self.save()
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
