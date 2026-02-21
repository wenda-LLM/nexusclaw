use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub tenant_id: String,
    pub user_id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub created_at: String,
    pub started_at: Option<String>,
    pub resources: ContainerResources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerStatus {
    Pending,
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
}

impl Default for ContainerStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerResources {
    pub cpu_limit: Option<u32>,
    pub memory_limit: Option<u64>,
    pub disk_limit: Option<u64>,
}

impl Default for ContainerResources {
    fn default() -> Self {
        Self {
            cpu_limit: Some(2),
            memory_limit: Some(1024),
            disk_limit: Some(5120),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContainerStoreData {
    pub containers: Vec<Container>,
}

pub struct ContainerManager {
    data: ContainerStoreData,
    path: std::path::PathBuf,
}

impl ContainerManager {
    #[inline]
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("containers.json");
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            ContainerStoreData::default()
        };
        Self { data, path }
    }

    pub fn create(
        &mut self,
        tenant_id: String,
        user_id: String,
        name: String,
        image: String,
    ) -> Result<Container> {
        let container = Container {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id,
            user_id,
            name,
            image,
            status: ContainerStatus::Pending,
            created_at: chrono::Utc::now().to_rfc3339(),
            started_at: None,
            resources: ContainerResources::default(),
        };
        self.data.containers.push(container.clone());
        self.save()?;
        Ok(container)
    }

    #[inline]
    pub fn get(&self, id: &str) -> Option<&Container> {
        self.data.containers.iter().find(|c| c.id == id)
    }

    #[inline]
    pub fn list_by_user(&self, user_id: &str) -> Vec<&Container> {
        self.data
            .containers
            .iter()
            .filter(|c| c.user_id == user_id)
            .collect()
    }

    #[inline]
    pub fn list_by_tenant(&self, tenant_id: &str) -> Vec<&Container> {
        self.data
            .containers
            .iter()
            .filter(|c| c.tenant_id == tenant_id)
            .collect()
    }

    pub fn start(&mut self, id: &str) -> Result<()> {
        if let Some(c) = self.data.containers.iter_mut().find(|c| c.id == id) {
            c.status = ContainerStatus::Starting;
            c.started_at = Some(chrono::Utc::now().to_rfc3339());
            self.save()?;
        }
        Ok(())
    }

    pub fn stop(&mut self, id: &str) -> Result<()> {
        if let Some(c) = self.data.containers.iter_mut().find(|c| c.id == id) {
            c.status = ContainerStatus::Stopping;
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
