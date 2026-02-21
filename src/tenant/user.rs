use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub tenant_id: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub role: UserRole,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_login: Option<String>,
    pub status: UserStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    User,
    Developer,
    Admin,
    Owner,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Pending,
    Suspended,
    Deleted,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub created_at: String,
    pub expires_at: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStoreData {
    pub users: Vec<User>,
    pub sessions: Vec<UserSession>,
    pub allowlist: Vec<String>,
}

pub struct UserStore {
    data: UserStoreData,
    path: std::path::PathBuf,
}

impl UserStore {
    pub fn is_empty(&self) -> bool {
        self.data.users.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &User> {
        self.data.users.iter()
    }
}

impl Clone for UserStore {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            path: self.path.clone(),
        }
    }
}

impl UserStore {
    #[inline]
    pub fn new(base_path: &Path) -> Self {
        let path = base_path.join("users.json");
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            UserStoreData::default()
        };
        Self { data, path }
    }

    pub fn create(
        &mut self,
        tenant_id: String,
        email: String,
        password_hash: String,
        role: UserRole,
    ) -> Result<User> {
        let now = chrono::Utc::now().to_rfc3339();
        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id,
            email,
            password_hash,
            display_name: None,
            role,
            mfa_enabled: false,
            mfa_secret: None,
            created_at: now.clone(),
            updated_at: now,
            last_login: None,
            status: UserStatus::Pending,
        };
        self.data.users.push(user.clone());
        self.save()?;
        Ok(user)
    }

    pub fn create_with_user(&mut self, user: User) -> Result<User> {
        self.data.users.push(user.clone());
        self.save()?;
        Ok(user)
    }

    #[inline]
    pub fn get(&self, id: &str) -> Option<&User> {
        self.data.users.iter().find(|u| u.id == id)
    }

    #[inline]
    pub fn get_by_email(&self, email: &str) -> Option<&User> {
        self.data.users.iter().find(|u| u.email == email)
    }

    #[inline]
    pub fn get_by_email_tenant(&self, email: &str, tenant_id: &str) -> Option<&User> {
        self.data
            .users
            .iter()
            .find(|u| u.email == email && u.tenant_id == tenant_id)
    }

    #[inline]
    pub fn list_by_tenant(&self, tenant_id: &str) -> Vec<&User> {
        self.data
            .users
            .iter()
            .filter(|u| u.tenant_id == tenant_id)
            .collect()
    }

    pub fn assign_tenant(&mut self, user_id: &str, tenant_id: String) -> Result<()> {
        if let Some(user) = self.get_mut(user_id) {
            user.tenant_id = tenant_id;
            user.updated_at = chrono::Utc::now().to_rfc3339();
            self.save()?;
            Ok(())
        } else {
            anyhow::bail!("User not found")
        }
    }

    pub fn update_role(&mut self, user_id: &str, role: UserRole) -> Result<()> {
        if let Some(user) = self.get_mut(user_id) {
            user.role = role;
            user.updated_at = chrono::Utc::now().to_rfc3339();
            self.save()?;
            Ok(())
        } else {
            anyhow::bail!("User not found")
        }
    }

    #[inline]
    pub fn get_mut(&mut self, id: &str) -> Option<&mut User> {
        self.data.users.iter_mut().find(|u| u.id == id)
    }

    pub fn update(
        &mut self,
        id: &str,
        display_name: Option<String>,
        role: Option<UserRole>,
        mfa_enabled: Option<bool>,
    ) -> Result<()> {
        if let Some(user) = self.get_mut(id) {
            if let Some(n) = display_name {
                user.display_name = Some(n);
            }
            if let Some(r) = role {
                user.role = r;
            }
            if let Some(m) = mfa_enabled {
                user.mfa_enabled = m;
            }
            user.updated_at = chrono::Utc::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
    }

    #[inline]
    pub fn delete(&mut self, id: &str) -> Result<()> {
        if let Some(user) = self.get_mut(id) {
            user.status = UserStatus::Deleted;
            user.updated_at = chrono::Utc::now().to_rfc3339();
            self.save()?;
        }
        Ok(())
    }

    pub fn create_session(&mut self, user_id: String, expires_in_secs: u64) -> Result<UserSession> {
        let now = chrono::Utc::now();
        let session = UserSession {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            token: generate_session_token(),
            created_at: now.to_rfc3339(),
            expires_at: (now + chrono::Duration::seconds(expires_in_secs as i64)).to_rfc3339(),
            ip_address: None,
            user_agent: None,
        };
        self.data.sessions.retain(|s| {
            if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(&s.expires_at) {
                expires > chrono::Utc::now()
            } else {
                false
            }
        });
        self.data.sessions.push(session.clone());
        self.save()?;
        Ok(session)
    }

    pub fn validate_session(&self, token: &str) -> Option<&UserSession> {
        let now = chrono::Utc::now();
        self.data.sessions.iter().find(|s| {
            s.token == token
                && chrono::DateTime::parse_from_rfc3339(&s.expires_at)
                    .map(|e| e > now)
                    .unwrap_or(false)
        })
    }

    pub fn revoke_session(&mut self, token: &str) -> Result<()> {
        self.data.sessions.retain(|s| s.token != token);
        self.save()
    }

    #[inline]
    pub fn allowlist_add(&mut self, email: String) -> Result<()> {
        if !self.data.allowlist.contains(&email) {
            self.data.allowlist.push(email);
            self.save()?;
        }
        Ok(())
    }

    #[inline]
    pub fn allowlist_remove(&mut self, email: &str) -> Result<()> {
        self.data.allowlist.retain(|e| e != email);
        self.save()
    }

    #[inline]
    pub fn is_allowlisted(&self, email: &str) -> bool {
        self.data.allowlist.is_empty() || self.data.allowlist.contains(&email.to_string())
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

fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    getrandom::getrandom(&mut bytes).expect("RNG failed");
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes)
}
