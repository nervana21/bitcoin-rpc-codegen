//! Minimal node manager implementation for Bitcoin RPC

use crate::config::Config;
use crate::transport::{DefaultTransport, TransportError};
use serde_json::json;

#[derive(Debug, Clone)]
pub struct BitcoinNodeManager {
    pub transport: DefaultTransport,
}

impl BitcoinNodeManager {
    pub fn new(config: &Config) -> Self {
        let transport = DefaultTransport::new(
            &config.rpc_url,
            Some((config.rpc_user.clone(), config.rpc_password.clone())),
        );
        Self { transport }
    }

    pub async fn shutdown(&self) -> Result<(), TransportError> {
        let _ = self.transport.call("stop", &[]).await?;
        Ok(())
    }

    pub async fn getblockchaininfo(&self) -> Result<serde_json::Value, TransportError> {
        self.transport.call("getblockchaininfo", &[]).await
    }

    pub async fn getblockhash(&self, height: u64) -> Result<serde_json::Value, TransportError> {
        self.transport.call("getblockhash", &[json!(height)]).await
    }
}
