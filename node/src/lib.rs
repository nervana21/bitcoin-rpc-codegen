// node/src/lib.rs

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Represents the state of a Bitcoin node
#[derive(Debug, Clone)]
pub struct NodeState {
    pub is_running: bool,
    pub version: String,
}

impl Default for NodeState {
    fn default() -> Self {
        Self {
            is_running: false,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

/// Trait defining the interface for a Bitcoin node manager
#[async_trait]
pub trait NodeManager: Send + Sync {
    async fn start(&self) -> Result<()>;
    async fn stop(&self) -> Result<()>;
    async fn get_state(&self) -> Result<NodeState>;
}

/// Implementation of the Bitcoin node manager
pub struct BitcoinNodeManager {
    state: Arc<RwLock<NodeState>>,
}

impl BitcoinNodeManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(NodeState::default())),
        }
    }
}

#[async_trait]
impl NodeManager for BitcoinNodeManager {
    async fn start(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if state.is_running {
            return Ok(());
        }

        info!("Starting Bitcoin node...");
        state.is_running = true;
        Ok(())
    }

    async fn stop(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if !state.is_running {
            return Ok(());
        }

        info!("Stopping Bitcoin node...");
        state.is_running = false;
        Ok(())
    }

    async fn get_state(&self) -> Result<NodeState> {
        Ok(self.state.read().await.clone())
    }
}
