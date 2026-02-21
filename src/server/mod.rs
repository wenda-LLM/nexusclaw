use crate::config::Config;
use crate::tenant::{ContainerManager, GroupStore, TenantStore, UserRole, UserStore, VaultStore};
use anyhow::Result;
use axum::{
    body::Body,
    extract::{ws, Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::Utc;
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

async fn serve_static(path: &str, dist_dir: &std::path::Path) -> Response<Body> {
    let file_path = if path.is_empty() || path == "/" {
        dist_dir.join("index.html")
    } else {
        let p = dist_dir.join(path);
        if p.exists() && p.is_file() {
            p
        } else {
            dist_dir.join("index.html")
        }
    };

    if file_path.exists() {
        let content = std::fs::read(&file_path).unwrap_or_default();
        let mime = if file_path.ends_with(".html") {
            "text/html"
        } else if file_path.ends_with(".js") {
            "application/javascript"
        } else if file_path.ends_with(".css") {
            "text/css"
        } else {
            "application/octet-stream"
        };
        Response::builder()
            .header("Content-Type", mime)
            .body(Body::from(content))
            .unwrap_or_default()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap_or_default()
    }
}

#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<UserStore>>,
    tenants: Arc<RwLock<TenantStore>>,
    groups: Arc<RwLock<GroupStore>>,
    vault: Arc<RwLock<VaultStore>>,
    containers: Arc<RwLock<ContainerManager>>,
    config: Arc<RwLock<Option<Config>>>,
    http_client: Client,
    chat_messages: Arc<RwLock<Vec<ChatMessage>>>,
    start_time: std::time::Instant,
}

#[derive(Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
    timestamp: i64,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    data: Option<T>,
    error: Option<String>,
}

impl<T> ApiResponse<T> {
    fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }
    fn err(msg: &str) -> Self {
        Self {
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

fn build_config_snapshot(config: &Config) -> serde_json::Value {
    let raw = fs::read_to_string(&config.config_path)
        .map(|content| content.trim().to_string())
        .unwrap_or_default();

    let hash = {
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        format!("{:x}", hasher.finalize())
    };

    serde_json::json!({
        "path": config.config_path.to_string_lossy().to_string(),
        "exists": true,
        "valid": true,
        "raw": raw,
        "hash": hash,
        "config": {
            "default_provider": config.default_provider,
            "default_model": config.default_model,
            "default_temperature": config.default_temperature,
            "model_routes": config.model_routes,
            "embedding_routes": config.embedding_routes,
            "observability": config.observability,
            "autonomy": config.autonomy,
            "runtime": config.runtime,
            "reliability": config.reliability,
            "scheduler": config.scheduler,
            "agent": config.agent,
            "heartbeat": config.heartbeat,
            "cron": config.cron,
            "channels_config": config.channels_config,
            "memory": config.memory,
            "storage": config.storage,
            "tunnel": config.tunnel,
            "gateway": config.gateway,
            "composio": config.composio,
            "secrets": config.secrets,
            "browser": config.browser,
            "http_request": config.http_request,
            "multimodal": config.multimodal,
            "web_search": config.web_search,
            "proxy": config.proxy,
            "identity": config.identity,
            "cost": config.cost,
            "peripherals": config.peripherals,
            "agents": config.agents,
            "hardware": config.hardware,
        },
        "issues": []
    })
}

#[derive(Serialize)]
struct ConfigSchemaResponse {
    schema: serde_json::Value,
    uiHints: serde_json::Value,
    version: String,
    generatedAt: String,
}

fn build_config_schema() -> ConfigSchemaResponse {
    ConfigSchemaResponse {
        schema: serde_json::json!({
            "type": "object",
            "properties": {
                "default_provider": { "type": "string", "description": "Default AI provider" },
                "default_model": { "type": "string", "description": "Default AI model" },
                "default_temperature": { "type": "number" },
                "model_routes": { "type": "array" },
                "embedding_routes": { "type": "array" }
            }
        }),
        uiHints: serde_json::json!({
            "default_provider": { "label": "默认提供商", "order": 1 },
            "default_model": { "label": "默认模型", "order": 2 },
            "default_temperature": { "label": "温度", "order": 3 }
        }),
        version: "1.0.0".to_string(),
        generatedAt: Utc::now().to_rfc3339(),
    }
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct VaultRequest {
    name: String,
    credential_type: String,
    value: String,
}

#[derive(Deserialize)]
struct GroupRequest {
    name: String,
}

#[derive(Deserialize)]
struct TenantRequest {
    name: String,
    domain: Option<String>,
    #[serde(default)]
    max_users: Option<u32>,
    #[serde(default)]
    max_containers: Option<u32>,
    #[serde(default)]
    allow_signup: bool,
    #[serde(default)]
    mfa_required: bool,
}

#[derive(Deserialize)]
struct TenantPatchRequest {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    domain: Option<String>,
    #[serde(default)]
    max_users: Option<u32>,
    #[serde(default)]
    max_containers: Option<u32>,
    #[serde(default)]
    allow_signup: Option<bool>,
    #[serde(default)]
    mfa_required: Option<bool>,
}

#[derive(Deserialize)]
struct GatewayRequest {
    #[serde(rename = "type")]
    msg_type: String,
    id: Option<String>,
    method: Option<String>,
    params: Option<serde_json::Value>,
    min_protocol: Option<u32>,
    max_protocol: Option<u32>,
    client: Option<serde_json::Value>,
    role: Option<String>,
    scopes: Option<Vec<String>>,
    auth: Option<serde_json::Value>,
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let user_data = {
        let users = state.users.read().await;
        users.get_by_email(&req.email).map(|u| {
            (
                u.id.clone(),
                u.email.clone(),
                u.role.clone(),
                u.password_hash.clone(),
                u.display_name.clone().unwrap_or_default(),
            )
        })
    };

    if let Some((user_id, user_email, user_role, password_hash, user_name)) = user_data {
        if verify_password(&req.password, &password_hash) {
            let mut usersw = state.users.write().await;
            if let Ok(session) = usersw.create_session(user_id.clone(), 604800) {
                let role_str = match user_role {
                    UserRole::Admin => "admin",
                    UserRole::Owner => "admin",
                    _ => "user",
                };
                return Json(ApiResponse::ok(
                    serde_json::json!({ "token": session.token, "user": { "email": user_email, "role": role_str, "id": user_id, "name": user_name } }),
                ));
            }
        }
    }
    Json(ApiResponse::err("Invalid credentials"))
}

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut users = state.users.write().await;
    let password_hash = hash_password(&req.password);

    let is_first_user = users.is_empty();
    let role = if is_first_user {
        UserRole::Admin
    } else {
        UserRole::User
    };

    match users.create(
        "default".to_string(),
        req.email.clone(),
        password_hash,
        role,
    ) {
        Ok(user) => {
            if let Ok(session) = users.create_session(user.id, 604800) {
                return Json(ApiResponse::ok(
                    serde_json::json!({ "token": session.token, "isFirstUser": is_first_user }),
                ));
            }
        }
        Err(_) => return Json(ApiResponse::err("Registration failed")),
    }
    Json(ApiResponse::err("Registration failed"))
}

async fn get_me(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Json<ApiResponse<serde_json::Value>> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    if let Some(token) = token {
        let users = state.users.read().await;
        if let Some(session) = users.validate_session(token) {
            if let Some(user) = users.get(&session.user_id) {
                let role_lower = match user.role {
                    crate::tenant::UserRole::Admin => "admin",
                    crate::tenant::UserRole::Owner => "admin",
                    _ => "user",
                };
                return Json(ApiResponse::ok(serde_json::json!({
                    "user": {
                        "email": user.email,
                        "role": role_lower,
                        "id": user.id,
                        "name": user.display_name.clone().unwrap_or_default(),
                        "tenant_id": user.tenant_id
                    },
                    "isAdmin": role_lower == "admin"
                })));
            }
        }
    }

    Json(ApiResponse::ok(
        serde_json::json!({ "user": null, "isAdmin": false }),
    ))
}

async fn list_vault(State(state): State<AppState>) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let vault = state.vault.read().await;
    let entries: Vec<_> = vault.get_by_user("demo").iter().map(|e| {
        serde_json::json!({ "id": e.id, "name": e.name, "credentialType": e.credential_type, "updatedAt": e.updated_at })
    }).collect();
    Json(ApiResponse::ok(entries))
}

async fn create_vault(
    State(state): State<AppState>,
    Json(req): Json<VaultRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut vault = state.vault.write().await;
    match vault.store(
        "demo".to_string(),
        "demo".to_string(),
        req.name,
        req.credential_type,
        &req.value,
        None,
        None,
    ) {
        Ok(id) => Json(ApiResponse::ok(serde_json::json!({ "id": id }))),
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

async fn delete_vault(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut vault = state.vault.write().await;
    vault.delete(&id);
    Json(ApiResponse::ok(serde_json::json!({ "deleted": true })))
}

async fn list_groups(State(state): State<AppState>) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let groups = state.groups.read().await;
    let list: Vec<_> = groups
        .list_by_tenant("default")
        .iter()
        .map(|g| serde_json::json!({ "id": g.id, "name": g.name }))
        .collect();
    Json(ApiResponse::ok(list))
}

async fn create_group(
    State(state): State<AppState>,
    Json(req): Json<GroupRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut groups = state.groups.write().await;
    match groups.create("default".to_string(), req.name, None, "system".to_string()) {
        Ok(group) => Json(ApiResponse::ok(
            serde_json::json!({ "id": group.id, "name": group.name }),
        )),
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

async fn list_containers(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let containers = state.containers.read().await;
    Json(ApiResponse::ok(vec![
        serde_json::json!({ "id": "demo", "status": "stopped" }),
    ]))
}

async fn admin_list_tenants(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let tenants = state.tenants.read().await;
    let users = state.users.read().await;
    let groups = state.groups.read().await;
    let containers = state.containers.read().await;

    let list: Vec<_> = tenants
        .list()
        .iter()
        .map(|t| {
            let user_count = users.list_by_tenant(&t.id).len();
            let group_count = groups.list_by_tenant(&t.id).len();
            let container_count = containers.list_by_tenant(&t.id).len();
            serde_json::json!({
                "id": t.id,
                "name": t.name,
                "domain": t.domain,
                "status": format!("{:?}", t.status),
                "createdAt": t.created_at,
                "updatedAt": t.updated_at,
                "settings": {
                    "maxUsers": t.settings.max_users,
                    "maxContainers": t.settings.max_containers,
                    "allowSignup": t.settings.allow_signup,
                    "mfaRequired": t.settings.mfa_required,
                },
                "stats": {
                    "userCount": user_count,
                    "groupCount": group_count,
                    "containerCount": container_count,
                }
            })
        })
        .collect();
    Json(ApiResponse::ok(list))
}

async fn admin_create_tenant(
    State(state): State<AppState>,
    Json(req): Json<TenantRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut tenants = state.tenants.write().await;
    let settings = crate::tenant::TenantSettings {
        max_users: req.max_users,
        max_containers: req.max_containers,
        allow_signup: req.allow_signup,
        mfa_required: req.mfa_required,
    };
    match tenants.create_with_settings(req.name, req.domain, settings) {
        Ok(tenant) => Json(ApiResponse::ok(serde_json::json!({
            "id": tenant.id,
            "name": tenant.name,
            "domain": tenant.domain,
            "status": format!("{:?}", tenant.status),
        }))),
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

async fn admin_update_tenant(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
    Json(req): Json<TenantPatchRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut tenants = state.tenants.write().await;
    let settings = if req.max_users.is_some()
        || req.max_containers.is_some()
        || req.allow_signup.is_some()
        || req.mfa_required.is_some()
    {
        Some(crate::tenant::TenantSettings {
            max_users: req.max_users,
            max_containers: req.max_containers,
            allow_signup: req.allow_signup.unwrap_or(true),
            mfa_required: req.mfa_required.unwrap_or(false),
        })
    } else {
        None
    };

    match tenants.update(&tenant_id, req.name, req.domain, settings) {
        Ok(_) => {
            if let Some(tenant) = tenants.get(&tenant_id) {
                Json(ApiResponse::ok(serde_json::json!({
                    "id": tenant.id,
                    "name": tenant.name,
                    "domain": tenant.domain,
                    "status": format!("{:?}", tenant.status),
                })))
            } else {
                Json(ApiResponse::err("Tenant not found"))
            }
        }
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

async fn admin_delete_tenant(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    if tenant_id == "tenant_1" {
        return Json(ApiResponse::err("Cannot delete default tenant"));
    }
    let mut tenants = state.tenants.write().await;
    match tenants.delete(&tenant_id) {
        Ok(_) => Json(ApiResponse::ok(serde_json::json!({ "deleted": true }))),
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

async fn admin_list_users(
    State(state): State<AppState>,
) -> Json<ApiResponse<Vec<serde_json::Value>>> {
    let users = state.users.read().await;
    let list: Vec<_> = users
        .iter()
        .map(|u| {
            serde_json::json!({
                "id": u.id,
                "email": u.email,
                "tenantId": u.tenant_id,
                "role": format!("{:?}", u.role),
                "status": format!("{:?}", u.status),
                "createdAt": u.created_at,
            })
        })
        .collect();
    Json(ApiResponse::ok(list))
}

#[derive(Deserialize)]
struct AssignUserRequest {
    tenant_id: String,
}

async fn admin_assign_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<AssignUserRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut users = state.users.write().await;
    match users.assign_tenant(&user_id, req.tenant_id) {
        Ok(_) => Json(ApiResponse::ok(serde_json::json!({ "assigned": true }))),
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

#[derive(Deserialize)]
struct UpdateUserRoleRequest {
    role: String,
}

async fn admin_update_user_role(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut users = state.users.write().await;
    let role = match req.role.as_str() {
        "Admin" => UserRole::Admin,
        "User" => UserRole::User,
        _ => return Json(ApiResponse::err("Invalid role")),
    };
    match users.update_role(&user_id, role) {
        Ok(_) => Json(ApiResponse::ok(serde_json::json!({ "updated": true }))),
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

async fn admin_create_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Json<ApiResponse<serde_json::Value>> {
    let mut users = state.users.write().await;
    let password_hash = hash_password(&req.password);

    match users.create(
        "default".to_string(),
        req.email.clone(),
        password_hash,
        UserRole::User,
    ) {
        Ok(user) => Json(ApiResponse::ok(serde_json::json!({
            "id": user.id,
            "email": user.email,
            "tenantId": user.tenant_id,
            "role": format!("{:?}", user.role),
        }))),
        Err(e) => Json(ApiResponse::err(&e.to_string())),
    }
}

async fn admin_stats(State(state): State<AppState>) -> Json<ApiResponse<serde_json::Value>> {
    let tenants = state.tenants.read().await;
    let users = state.users.read().await;
    let _containers = state.containers.read().await;
    Json(ApiResponse::ok(serde_json::json!({
        "tenants": tenants.list().len(),
        "users": users.iter().count(),
    })))
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

async fn handle_chat_proxy(Json(req): Json<ChatRequest>) -> Response<Body> {
    let config = match Config::load_or_init().await {
        Ok(cfg) => cfg,
        Err(e) => {
            return Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .body(Body::from(format!("Config error: {}", e)))
                .unwrap_or_else(|_| Response::new(Body::empty()));
        }
    };

    match crate::agent::loop_::run(config, Some(req.message), None, None, 0.7, vec![]).await {
        Ok(response) => Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({ "response": response }).to_string(),
            ))
            .unwrap_or_else(|_| Response::new(Body::empty())),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(e.to_string()))
            .unwrap_or_else(|_| Response::new(Body::empty())),
    }
}

fn verify_password(password: &str, hash: &str) -> bool {
    hash == hash_password(password)
}

fn hash_password(password: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

async fn ws_handler(ws: ws::WebSocketUpgrade, State(state): State<AppState>) -> Response<Body> {
    let start_time = state.start_time.elapsed().as_millis() as u64;
    ws.on_upgrade(move |socket| async move {
        let (mut send, mut recv) = socket.split();
        
        while let Some(msg) = recv.next().await {
            if let Ok(ws::Message::Text(text)) = msg {
                if let Ok(req) = serde_json::from_str::<GatewayRequest>(&text) {
                    let response = match req.method.as_deref() {
                        Some("connect") | None if req.msg_type != "req" => {
                            serde_json::json!({
                                "type": "hello-ok",
                                "protocol": 3,
                                "features": {
                                    "methods": [
                                        "agents.list", "chat.history", "chat.send", "chat.abort",
                                        "sessions.list", "sessions.patch", "sessions.delete",
                                        "channels.status", "config.get", "config.set", "config.apply", "config.patch", "config.schema",
                                        "cron.status", "cron.list", "cron.add", "cron.update", "cron.run", "cron.remove",
                                        "skills.status", "skills.update", "skills.install",
                                        "node.list", "logs.tail", "usage.cost", "exec.approvals.get",
                                        "exec.approvals.set", "update.run", "gateway.list_methods"
                                    ],
                                    "events": ["chat", "agents", "sessions", "channels", "cron", "exec"]
                                },
                                "snapshot": {
                                    "agents": [],
                                    "sessions": [],
                                    "channels": {},
                                    "cron": { "jobs": [] },
                                    "uptimeMs": start_time
                                },
                                "auth": {
                                    "role": "operator",
                                    "scopes": ["operator.admin", "operator.approvals", "operator.pairing"]
                                },
                                "policy": {
                                    "tickIntervalMs": 5000
                                }
                            })
                        }
                        Some("config.get") => {
                            let config_guard = state.config.read().await;
                            match config_guard.as_ref() {
                                Some(config) => {
                                    serde_json::json!({
                                        "type": "res",
                                        "id": req.id,
                                        "ok": true,
                                        "payload": build_config_snapshot(config)
                                    })
                                }
                                None => {
                                    serde_json::json!({
                                        "type": "res",
                                        "id": req.id,
                                        "ok": false,
                                        "error": "Config not loaded"
                                    })
                                }
                            }
                        }
                        Some("config.set") => {
                            let params = req.params.unwrap_or(serde_json::json!({}));
                            
                            let raw = params.get("raw").and_then(|v| v.as_str());
                            
                            if let Some(raw_content) = raw {
                                let mut config_guard = state.config.write().await;
                                if let Some(config) = config_guard.as_mut() {
                                    if let Some(expected_hash) = params.get("baseHash").and_then(|v| v.as_str()) {
                                        let current_raw = fs::read_to_string(&config.config_path)
                                            .map(|c| c.trim().to_string())
                                            .unwrap_or_default();
                                        let mut hasher = Sha256::new();
                                        hasher.update(current_raw.as_bytes());
                                        let current_hash = format!("{:x}", hasher.finalize());
                                        
                                        if current_hash != expected_hash {
                                            serde_json::json!({
                                                "type": "res",
                                                "id": req.id,
                                                "ok": false,
                                                "error": "配置已更改；请重新加载并重试"
                                            })
                                        } else if let Err(e) = fs::write(&config.config_path, raw_content) {
                                            serde_json::json!({
                                                "type": "res",
                                                "id": req.id,
                                                "ok": false,
                                                "error": format!("写入配置失败: {}", e)
                                            })
                                        } else {
                                            match Config::load_or_init().await {
                                                Ok(new_config) => {
                                                    *config_guard = Some(new_config);
                                                    serde_json::json!({
                                                        "type": "res",
                                                        "id": req.id,
                                                        "ok": true,
                                                        "payload": { "saved": true }
                                                    })
                                                }
                                                Err(e) => {
                                                    serde_json::json!({
                                                        "type": "res",
                                                        "id": req.id,
                                                        "ok": false,
                                                        "error": format!("重新加载配置失败: {}", e)
                                                    })
                                                }
                                            }
                                        }
                                    } else if let Err(e) = fs::write(&config.config_path, raw_content) {
                                        serde_json::json!({
                                            "type": "res",
                                            "id": req.id,
                                            "ok": false,
                                            "error": format!("写入配置失败: {}", e)
                                        })
                                    } else {
                                        match Config::load_or_init().await {
                                            Ok(new_config) => {
                                                *config_guard = Some(new_config);
                                                serde_json::json!({
                                                    "type": "res",
                                                    "id": req.id,
                                                    "ok": true,
                                                    "payload": { "saved": true }
                                                })
                                            }
                                            Err(e) => {
                                                serde_json::json!({
                                                    "type": "res",
                                                    "id": req.id,
                                                    "ok": false,
                                                    "error": format!("重新加载配置失败: {}", e)
                                                })
                                            }
                                        }
                                    }
                                } else {
                                    serde_json::json!({
                                        "type": "res",
                                        "id": req.id,
                                        "ok": false,
                                        "error": "配置未加载"
                                    })
                                }
                            } else if let Some(config_obj) = params.get("config") {
                                let mut config_guard = state.config.write().await;
                                if let Some(config) = config_guard.as_mut() {
                                    if let Some(provider) = config_obj.get("default_provider").and_then(|v| v.as_str()) {
                                        config.default_provider = Some(provider.to_string());
                                    }
                                    if let Some(model) = config_obj.get("default_model").and_then(|v| v.as_str()) {
                                        config.default_model = Some(model.to_string());
                                    }
                                    if let Some(temp) = config_obj.get("default_temperature").and_then(|v| v.as_f64()) {
                                        config.default_temperature = temp;
                                    }
                                    
                                    if let Err(e) = config.save().await {
                                        serde_json::json!({
                                            "type": "res",
                                            "id": req.id,
                                            "ok": false,
                                            "error": format!("保存配置失败: {}", e)
                                        })
                                    } else {
                                        serde_json::json!({
                                            "type": "res",
                                            "id": req.id,
                                            "ok": true,
                                            "payload": { "saved": true }
                                        })
                                    }
                                } else {
                                    serde_json::json!({
                                        "type": "res",
                                        "id": req.id,
                                        "ok": false,
                                        "error": "配置未加载"
                                    })
                                }
                            } else {
                                serde_json::json!({
                                    "type": "res",
                                    "id": req.id,
                                    "ok": false,
                                    "error": "缺少 raw 或 config 参数"
                                })
                            }
                        }
                        Some("config.schema") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": build_config_schema()
                            })
                        }
                        Some("config.apply") => {
                            let params = req.params.unwrap_or(serde_json::json!({}));
                            let raw = params.get("raw").and_then(|v| v.as_str());
                            
                            if let Some(raw_content) = raw {
                                let mut config_guard = state.config.write().await;
                                if let Some(config) = config_guard.as_mut() {
                                    if let Some(expected_hash) = params.get("baseHash").and_then(|v| v.as_str()) {
                                        let current_raw = fs::read_to_string(&config.config_path)
                                            .map(|c| c.trim().to_string())
                                            .unwrap_or_default();
                                        let mut hasher = Sha256::new();
                                        hasher.update(current_raw.as_bytes());
                                        let current_hash = format!("{:x}", hasher.finalize());
                                        
                                        if current_hash != expected_hash {
                                            serde_json::json!({
                                                "type": "res",
                                                "id": req.id,
                                                "ok": false,
                                                "error": "配置已更改；请重新加载并重试"
                                            })
                                        } else if let Err(e) = fs::write(&config.config_path, raw_content) {
                                            serde_json::json!({
                                                "type": "res",
                                                "id": req.id,
                                                "ok": false,
                                                "error": format!("写入配置失败: {}", e)
                                            })
                                        } else {
                                            serde_json::json!({
                                                "type": "res",
                                                "id": req.id,
                                                "ok": true,
                                                "payload": { "applied": true, "restartRequired": true }
                                            })
                                        }
                                    } else if let Err(e) = fs::write(&config.config_path, raw_content) {
                                        serde_json::json!({
                                            "type": "res",
                                            "id": req.id,
                                            "ok": false,
                                            "error": format!("写入配置失败: {}", e)
                                        })
                                    } else {
                                        serde_json::json!({
                                            "type": "res",
                                            "id": req.id,
                                            "ok": true,
                                            "payload": { "applied": true, "restartRequired": true }
                                        })
                                    }
                                } else {
                                    serde_json::json!({
                                        "type": "res",
                                        "id": req.id,
                                        "ok": false,
                                        "error": "配置未加载"
                                    })
                                }
                            } else {
                                serde_json::json!({
                                    "type": "res",
                                    "id": req.id,
                                    "ok": false,
                                    "error": "缺少 raw 参数"
                                })
                            }
                        }
                        Some("config.patch") => {
                            let params = req.params.unwrap_or(serde_json::json!({}));
                            let raw = params.get("raw").and_then(|v| v.as_str());
                            
                            if let Some(raw_content) = raw {
                                let mut config_guard = state.config.write().await;
                                if let Some(config) = config_guard.as_mut() {
                                    if let Err(e) = fs::write(&config.config_path, raw_content) {
                                        serde_json::json!({
                                            "type": "res",
                                            "id": req.id,
                                            "ok": false,
                                            "error": format!("写入配置失败: {}", e)
                                        })
                                    } else {
                                        match Config::load_or_init().await {
                                            Ok(new_config) => {
                                                *config_guard = Some(new_config);
                                                serde_json::json!({
                                                    "type": "res",
                                                    "id": req.id,
                                                    "ok": true,
                                                    "payload": { "patched": true }
                                                })
                                            }
                                            Err(e) => {
                                                serde_json::json!({
                                                    "type": "res",
                                                    "id": req.id,
                                                    "ok": false,
                                                    "error": format!("重新加载配置失败: {}", e)
                                                })
                                            }
                                        }
                                    }
                                } else {
                                    serde_json::json!({
                                        "type": "res",
                                        "id": req.id,
                                        "ok": false,
                                        "error": "配置未加载"
                                    })
                                }
                            } else {
                                serde_json::json!({
                                    "type": "res",
                                    "id": req.id,
                                    "ok": false,
                                    "error": "缺少 raw 参数"
                                })
                            }
                        }
                        Some("agents.list") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "agents": [] }
                            })
                        }
                        Some("sessions.list") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "sessions": [], "path": "" }
                            })
                        }
                        Some("sessions.patch") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "patched": true }
                            })
                        }
                        Some("sessions.delete") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "deleted": true }
                            })
                        }
                        Some("channels.status") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "channels": {}, "channelAccounts": [], "lastSuccessAt": null }
                            })
                        }
                        Some("cron.status") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "enabled": true, "jobs": 0, "nextWakeAtMs": null }
                            })
                        }
                        Some("cron.list") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "jobs": [] }
                            })
                        }
                        Some("cron.add") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "added": true }
                            })
                        }
                        Some("cron.update") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "updated": true }
                            })
                        }
                        Some("cron.remove") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "removed": true }
                            })
                        }
                        Some("cron.run") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "run": true }
                            })
                        }
                        Some("skills.status") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "skills": [], "workspaceSkills": [], "bundledSkills": [], "managedSkills": [] }
                            })
                        }
                        Some("skills.update") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "updated": true }
                            })
                        }
                        Some("skills.install") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "installed": true }
                            })
                        }
                        Some("node.list") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "nodes": [] }
                            })
                        }
                        Some("system-presence") => {
                            let config_guard = state.config.read().await;
                            let mode = config_guard.as_ref()
                                .map(|c| if c.gateway.require_pairing { "paired" } else { "open" })
                                .unwrap_or("open");
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": [{
                                    "host": "localhost",
                                    "mode": mode,
                                    "roles": ["node"],
                                    "scopes": ["operator.admin", "operator.approvals", "operator.pairing"],
                                    "version": env!("CARGO_PKG_VERSION")
                                }]
                            })
                        }
                        Some("node.list") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "nodes": [] }
                            })
                        }
                        Some("logs.tail") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "entries": [], "truncated": false }
                            })
                        }
                        Some("usage.cost") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { 
                                    "totalCost": 0,
                                    "totalTokens": 0,
                                    "daily": [],
                                    "byModel": [],
                                    "byProvider": []
                                }
                            })
                        }
                        Some("exec.approvals.get") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "policy": "deny", "ask": "on-miss", "allowlist": [] }
                            })
                        }
                        Some("exec.approvals.set") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "saved": true }
                            })
                        }
                        Some("chat.history") => {
                            let messages = state.chat_messages.read().await;
                            let chat_messages: Vec<serde_json::Value> = messages
                                .iter()
                                .map(|m| {
                                    serde_json::json!({
                                        "role": m.role,
                                        "content": [{"type": "text", "text": m.content}],
                                        "timestamp": m.timestamp
                                    })
                                })
                                .collect();
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "messages": chat_messages }
                            })
                        }
                        Some("chat.send") => {
                            let params = req.params.unwrap_or(serde_json::json!({}));
                            let message = params.get("message").and_then(|v| v.as_str()).unwrap_or("");
                            
                            let tenant_id = params.get("tenant_id").or(params.get("session")).and_then(|v| v.as_str()).unwrap_or("default");
                            tracing::info!(tenant_id = tenant_id, message = message, "chat.send received");
                            
                            let run_id = format!("run_{}", Uuid::new_v4().to_string().replace("-", "")[..8].to_string());
                            
                            match Config::load_or_init().await {
                                Ok(mut config) => {
                                    let tenant_workspace = config.workspace_dir.join(tenant_id);
                                    config.workspace_dir = tenant_workspace;
                                    
                                    if let Err(e) = tokio::fs::create_dir_all(&config.workspace_dir).await {
                                        tracing::warn!(workspace = %config.workspace_dir.display(), error = %e, "Failed to create tenant workspace directory");
                                    }
                                    
                                    match crate::agent::loop_::run(
                                        config,
                                        Some(message.to_string()),
                                        None,
                                        None,
                                        0.7,
                                        vec![],
                                    ).await {
                                        Ok(response) => {
                                            let now = Utc::now().timestamp_millis();
                                            {
                                                let mut messages = state.chat_messages.write().await;
                                                messages.push(ChatMessage {
                                                    role: "user".to_string(),
                                                    content: message.to_string(),
                                                    timestamp: now,
                                                });
                                                messages.push(ChatMessage {
                                                    role: "assistant".to_string(),
                                                    content: response.clone(),
                                                    timestamp: now + 1,
                                                });
                                            }
                                            
                                            let _ = send.send(ws::Message::Text(
                                                serde_json::json!({
                                                    "type": "event",
                                                    "event": "chat",
                                                    "payload": {
                                                        "runId": run_id,
                                                        "status": "ok",
                                                        "content": response
                                                    }
                                                }).to_string().into()
                                            )).await;
                                            
                                            serde_json::json!({
                                                "type": "res",
                                                "id": req.id,
                                                "ok": true,
                                                "payload": {
                                                    "runId": run_id,
                                                    "status": "ok"
                                                }
                                            })
                                        }
                                        Err(e) => {
                                            serde_json::json!({
                                                "type": "res",
                                                "id": req.id,
                                                "ok": false,
                                                "error": e.to_string()
                                            })
                                        }
                                    }
                                }
                                Err(e) => {
                                    serde_json::json!({
                                        "type": "res",
                                        "id": req.id,
                                        "ok": false,
                                        "error": format!("Config error: {}", e)
                                    })
                                }
                            }
                        }
                        Some("chat.abort") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "aborted": true }
                            })
                        }
                        Some("gateway.list_methods") | Some("status") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { 
                                    "methods": [
                                        "agents.list", "chat.history", "chat.send", "chat.abort",
                                        "sessions.list", "sessions.patch", "sessions.delete",
                                        "channels.status", "config.get", "config.set", "config.apply", "config.patch", "config.schema",
                                        "cron.status", "cron.list", "cron.add", "cron.update", "cron.run", "cron.remove",
                                        "skills.status", "skills.update", "skills.install",
                                        "node.list", "logs.tail", "usage.cost", "exec.approvals.get",
                                        "exec.approvals.set", "update.run", "gateway.list_methods"
                                    ]
                                }
                            })
                        }
                        Some("update.run") => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": { "running": true }
                            })
                        }
                        _ => {
                            serde_json::json!({
                                "type": "res",
                                "id": req.id,
                                "ok": true,
                                "payload": {}
                            })
                        }
                    };
                    
                    let _ = send.send(ws::Message::Text(response.to_string().into())).await;
                }
            }
        }
    })
}

pub async fn run_server(port: u16, host: &str, data_dir: &std::path::Path) -> Result<()> {
    let users_store = UserStore::new(data_dir);

    let config = match Config::load_or_init().await {
        Ok(cfg) => {
            tracing::info!("Config loaded from {:?}", cfg.config_path);
            Some(cfg)
        }
        Err(e) => {
            tracing::warn!("Failed to load config: {}, using default", e);
            None
        }
    };

    let state = AppState {
        users: Arc::new(RwLock::new(users_store)),
        tenants: Arc::new(RwLock::new(TenantStore::new(data_dir))),
        groups: Arc::new(RwLock::new(GroupStore::new(data_dir))),
        vault: Arc::new(RwLock::new(VaultStore::new(data_dir, &[0u8; 32]))),
        containers: Arc::new(RwLock::new(ContainerManager::new(data_dir))),
        config: Arc::new(RwLock::new(config)),
        http_client: Client::new(),
        chat_messages: Arc::new(RwLock::new(Vec::new())),
        start_time: std::time::Instant::now(),
    };

    let web_dist = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("multitenant-web")
        .join("dist");

    let web_dist_for_index = web_dist.clone();
    let web_dist_for_index2 = web_dist.clone();
    async fn serve_index(dist: std::path::PathBuf) -> Response<Body> {
        let content = std::fs::read_to_string(dist.join("index.html")).unwrap_or_default();
        Response::builder()
            .header("Content-Type", "text/html")
            .body(Body::from(content))
            .unwrap()
    }

    let static_service = tower_http::services::ServeDir::new(&web_dist).not_found_service(
        tower_http::services::ServeFile::new(web_dist.join("index.html")),
    );

    let app = Router::new()
        .route("/", get(move || serve_index(web_dist_for_index.clone())))
        .route(
            "/index.html",
            get(move || serve_index(web_dist_for_index2.clone())),
        )
        .route("/ws", get(ws_handler))
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/me", get(get_me))
        .route("/api/vault", get(list_vault).post(create_vault))
        .route("/api/vault/{id}", delete(delete_vault))
        .route("/api/groups", get(list_groups).post(create_group))
        .route("/api/containers", get(list_containers))
        .route(
            "/api/admin/tenants",
            get(admin_list_tenants).post(admin_create_tenant),
        )
        .route(
            "/api/admin/tenants/{tenant_id}",
            patch(admin_update_tenant).delete(admin_delete_tenant),
        )
        .route(
            "/api/admin/users",
            get(admin_list_users).post(admin_create_user),
        )
        .route("/api/admin/users/{user_id}/tenant", post(admin_assign_user))
        .route(
            "/api/admin/users/{user_id}/role",
            patch(admin_update_user_role),
        )
        .route("/api/admin/stats", get(admin_stats))
        .route("/api/chat", post(handle_chat_proxy))
        .fallback_service(static_service)
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let addr: SocketAddr = addr.parse()?;
    info!("Server running on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
