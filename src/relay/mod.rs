use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayMessage {
    pub id: String,
    pub from_container: String,
    pub to_container: String,
    pub payload: Vec<u8>,
    pub timestamp: String,
}

pub struct RelayServer {
    container_keys: Arc<RwLock<HashMap<String, String>>>,
    message_tx: broadcast::Sender<RelayMessage>,
}

impl RelayServer {
    pub fn new() -> Self {
        let (message_tx, _) = broadcast::channel(1000);
        Self {
            container_keys: Arc::new(RwLock::new(HashMap::new())),
            message_tx,
        }
    }

    pub async fn register_container(&self, container_id: String, public_key: String) {
        let mut keys = self.container_keys.write().await;
        keys.insert(container_id, public_key);
    }

    pub async fn get_container_key(&self, container_id: &str) -> Option<String> {
        let keys = self.container_keys.read().await;
        keys.get(container_id).cloned()
    }

    #[inline]
    pub fn subscribe(&self) -> broadcast::Receiver<RelayMessage> {
        self.message_tx.subscribe()
    }

    pub fn broadcast(&self, msg: RelayMessage) {
        let _ = self.message_tx.send(msg);
    }
}

impl Default for RelayServer {
    fn default() -> Self {
        Self::new()
    }
}
