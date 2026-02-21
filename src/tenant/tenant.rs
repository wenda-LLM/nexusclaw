use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub domain: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub settings: TenantSettings,
    pub status: TenantStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantSettings {
    pub max_users: Option<u32>,
    pub max_containers: Option<u32>,
    pub allow_signup: bool,
    pub mfa_required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TenantStatus {
    Active,
    Suspended,
    Deleted,
}

impl Default for TenantStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TenantStats {
    pub user_count: u32,
    pub group_count: u32,
    pub container_count: u32,
    pub storage_used_bytes: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TenantStoreData {
    pub tenants: Vec<Tenant>,
    pub next_id: u32,
}

pub struct TenantStore {
    data: TenantStoreData,
    path: std::path::PathBuf,
}

impl TenantStore {
    #[inline]
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("tenants.json");
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            TenantStoreData::default()
        };
        Self { data, path }
    }

    #[inline]
    pub fn create(&mut self, name: String, domain: Option<String>) -> Result<Tenant> {
        self.create_with_settings(name, domain, TenantSettings::default())
    }

    pub fn create_with_settings(
        &mut self,
        name: String,
        domain: Option<String>,
        settings: TenantSettings,
    ) -> Result<Tenant> {
        let now = chrono::Utc::now().to_rfc3339();
        self.data.next_id += 1;
        let tenant = Tenant {
            id: format!("tenant_{}", self.data.next_id),
            name,
            domain,
            created_at: now.clone(),
            updated_at: now,
            settings,
            status: TenantStatus::Active,
        };
        self.data.tenants.push(tenant.clone());
        self.save()?;
        Ok(tenant)
    }

    #[inline]
    pub fn get(&self, id: &str) -> Option<&Tenant> {
        self.data.tenants.iter().find(|t| t.id == id)
    }

    #[inline]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Tenant> {
        self.data.tenants.iter_mut().find(|t| t.id == id)
    }

    #[inline]
    pub fn list(&self) -> &[Tenant] {
        &self.data.tenants
    }

    #[inline]
    pub fn update(
        &mut self,
        id: &str,
        name: Option<String>,
        domain: Option<String>,
        settings: Option<TenantSettings>,
    ) -> Result<()> {
        if let Some(tenant) = self.get_mut(id) {
            if let Some(n) = name {
                tenant.name = n;
            }
            if let Some(d) = domain {
                tenant.domain = Some(d);
            }
            if let Some(s) = settings {
                tenant.settings = s;
            }
            tenant.updated_at = chrono::Utc::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
    }

    #[inline]
    pub fn delete(&mut self, id: &str) -> Result<()> {
        if let Some(tenant) = self.get_mut(id) {
            tenant.status = TenantStatus::Deleted;
            tenant.updated_at = chrono::Utc::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
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
