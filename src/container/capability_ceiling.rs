use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PermissionLevel {
    None,
    Read,
    Write,
    Admin,
    SuperAdmin,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CeilingRole {
    User,
    Developer,
    Admin,
    Owner,
}

impl Default for CeilingRole {
    fn default() -> Self {
        Self::User
    }
}

pub const PERMISSION_LEVELS: &[PermissionLevel] = &[
    PermissionLevel::None,
    PermissionLevel::Read,
    PermissionLevel::Write,
    PermissionLevel::Admin,
    PermissionLevel::SuperAdmin,
];

pub const CEILING_ROLES: &[CeilingRole] = &[
    CeilingRole::User,
    CeilingRole::Developer,
    CeilingRole::Admin,
    CeilingRole::Owner,
];

pub const DEFAULT_AGENT_CEILING: PermissionLevel = PermissionLevel::Write;

#[inline]
pub fn get_permission_order(level: PermissionLevel) -> u8 {
    match level {
        PermissionLevel::None => 0,
        PermissionLevel::Read => 1,
        PermissionLevel::Write => 2,
        PermissionLevel::Admin => 3,
        PermissionLevel::SuperAdmin => 4,
    }
}

#[inline]
pub fn is_within_ceiling(requested: PermissionLevel, ceiling: PermissionLevel) -> bool {
    get_permission_order(requested) <= get_permission_order(ceiling)
}

#[inline]
pub fn get_ceiling_for_role(role: CeilingRole) -> PermissionLevel {
    match role {
        CeilingRole::User => PermissionLevel::Read,
        CeilingRole::Developer => PermissionLevel::Write,
        CeilingRole::Admin => PermissionLevel::Admin,
        CeilingRole::Owner => PermissionLevel::SuperAdmin,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCeilingConfig {
    pub user_id: String,
    pub ceiling: PermissionLevel,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRequest {
    pub id: String,
    pub user_id: String,
    pub requested_level: PermissionLevel,
    pub reason: String,
    pub status: String,
    pub created_at: String,
    pub resolved_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CeilingStoreData {
    pub ceilings: HashMap<String, AgentCeilingConfig>,
    pub escalation_requests: Vec<EscalationRequest>,
}

pub struct CeilingManager {
    data: CeilingStoreData,
    path: std::path::PathBuf,
}

impl CeilingManager {
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("ceilings.json");
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            CeilingStoreData::default()
        };
        Self { data, path }
    }

    pub fn set_ceiling(&mut self, user_id: &str, ceiling: PermissionLevel) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        let config = AgentCeilingConfig {
            user_id: user_id.to_string(),
            ceiling,
            created_at: now.clone(),
            updated_at: now,
        };
        self.data.ceilings.insert(user_id.to_string(), config);
        self.save()
    }

    pub fn get_ceiling(&self, user_id: &str) -> PermissionLevel {
        self.data
            .ceilings
            .get(user_id)
            .map(|c| c.ceiling)
            .unwrap_or(DEFAULT_AGENT_CEILING)
    }

    pub fn list_ceilings(&self) -> Vec<&AgentCeilingConfig> {
        self.data.ceilings.values().collect()
    }

    pub fn remove_ceiling(&mut self, user_id: &str) -> Result<()> {
        self.data.ceilings.remove(user_id);
        self.save()
    }

    pub fn check_permission(&self, user_id: &str, requested: PermissionLevel) -> Result<()> {
        let ceiling = self.get_ceiling(user_id);
        if !is_within_ceiling(requested, ceiling) {
            bail!(
                "Permission denied: requested {:?} exceeds ceiling {:?}",
                requested,
                ceiling
            );
        }
        Ok(())
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&self.path, serde_json::to_string(&self.data)?)?;
        Ok(())
    }
}
