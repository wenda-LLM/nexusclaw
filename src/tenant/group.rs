use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
    pub settings: GroupSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupSettings {
    pub sharing_enabled: bool,
    pub require_approval: bool,
    pub max_members: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub user_id: String,
    pub role: GroupRole,
    pub joined_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GroupRole {
    Member,
    Admin,
    Owner,
}

impl Default for GroupRole {
    fn default() -> Self {
        Self::Member
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupStoreData {
    pub groups: Vec<Group>,
    pub members: HashMap<String, Vec<GroupMember>>,
}

pub struct GroupStore {
    data: GroupStoreData,
    path: std::path::PathBuf,
}

impl GroupStore {
    #[inline]
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("groups.json");
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            GroupStoreData::default()
        };
        Self { data, path }
    }

    pub fn create(
        &mut self,
        tenant_id: String,
        name: String,
        description: Option<String>,
        created_by: String,
    ) -> Result<Group> {
        let now = chrono::Utc::now().to_rfc3339();
        let group = Group {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id,
            name,
            description,
            created_by,
            created_at: now.clone(),
            updated_at: now,
            settings: GroupSettings::default(),
        };
        self.data.groups.push(group.clone());
        self.data.members.insert(group.id.clone(), vec![]);
        self.save()?;
        Ok(group)
    }

    #[inline]
    pub fn get(&self, id: &str) -> Option<&Group> {
        self.data.groups.iter().find(|g| g.id == id)
    }

    #[inline]
    pub fn list_by_tenant(&self, tenant_id: &str) -> Vec<&Group> {
        self.data
            .groups
            .iter()
            .filter(|g| g.tenant_id == tenant_id)
            .collect()
    }

    #[inline]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Group> {
        self.data.groups.iter_mut().find(|g| g.id == id)
    }

    pub fn add_member(&mut self, group_id: &str, user_id: String, role: GroupRole) -> Result<()> {
        let members = self
            .data
            .members
            .entry(group_id.to_string())
            .or_insert_with(Vec::new);
        if !members.iter().any(|m| m.user_id == user_id) {
            members.push(GroupMember {
                user_id,
                role,
                joined_at: chrono::Utc::now().to_rfc3339(),
            });
            if let Some(group) = self.get_mut(group_id) {
                group.updated_at = chrono::Utc::now().to_rfc3339();
            }
            self.save()?;
        }
        Ok(())
    }

    pub fn remove_member(&mut self, group_id: &str, user_id: &str) -> Result<()> {
        if let Some(members) = self.data.members.get_mut(group_id) {
            members.retain(|m| m.user_id != user_id);
            if let Some(group) = self.get_mut(group_id) {
                group.updated_at = chrono::Utc::now().to_rfc3339();
            }
            self.save()?;
        }
        Ok(())
    }

    #[inline]
    pub fn get_members(&self, group_id: &str) -> &[GroupMember] {
        self.data
            .members
            .get(group_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn is_member(&self, group_id: &str, user_id: &str) -> bool {
        self.data
            .members
            .get(group_id)
            .map(|m| m.iter().any(|mm| mm.user_id == user_id))
            .unwrap_or(false)
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
