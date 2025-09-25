//! Node module for Bitcoin RPC testing
//!
//! This module provides utilities for managing Bitcoin nodes in test environments.

use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Result;
use async_trait::async_trait;
use tempfile::TempDir;
use tokio::io::AsyncBufReadExt;
use tokio::process::{Child, Command};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info};
use std::process::Stdio;

use crate::test_config::TestConfig;

/// Represents the state of a Bitcoin node
#[derive(Debug, Default, Clone)]
pub struct NodeState {
    pub is_running: bool,
}

/// Configuration for port selection behavior
#[derive(Debug, Clone)]
pub enum PortSelection {
    /// Use the specified port number
    Fixed(u16),
    /// Let the OS assign an available port
    Dynamic,
    /// Use port 0 (not recommended, may cause bitcoind to fail)
    Zero,
}

/// Trait defining the interface for a Bitcoin node manager
#[async_trait]
pub trait NodeManager: Send + Sync + std::any::Any + std::fmt::Debug {
    async fn start(&self) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
    async fn get_state(&self) -> Result<NodeState>;
    /// Return the RPC port this manager was configured with
    fn rpc_port(&self) -> u16;
}

/// Implementation of the Bitcoin node manager
#[derive(Debug)]
pub struct BitcoinNodeManager {
    state: Arc<RwLock<NodeState>>,
    child: Arc<Mutex<Option<Child>>>,
    pub rpc_port: u16,
    config: TestConfig,
    _datadir: Option<TempDir>,
}

impl BitcoinNodeManager {
    pub fn new() -> Result<Self> { Self::new_with_config(&TestConfig::default()) }

    pub fn new_with_config(config: &TestConfig) -> Result<Self> {
        let datadir = TempDir::new()?;

        // Handle automatic port selection:
        // When rpc_port is 0, we need to find an available port dynamically.
        // This is important because:
        // 1. It allows multiple test instances to run in parallel without port conflicts
        // 2. It prevents the "Invalid port specified in -rpcport: '0'" error from bitcoind
        // 3. It makes the code more robust by not requiring manual port selection
        let rpc_port = if config.rpc_port == 0 {
            // Bind to port 0 to let the OS assign an available port
            let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
            let port = listener.local_addr()?.port();
            drop(listener);
            port
        } else {
            config.rpc_port
        };

        Ok(Self {
            state: Arc::new(RwLock::new(NodeState::default())),
            child: Arc::new(Mutex::new(None)),
            rpc_port,
            config: config.clone(),
            _datadir: Some(datadir),
        })
    }

    pub fn rpc_port(&self) -> u16 { self.rpc_port }
}

#[async_trait]
impl NodeManager for BitcoinNodeManager {
    async fn start(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if state.is_running {
            return Ok(());
        }

        let datadir = self._datadir.as_ref().unwrap().path();
        let mut cmd = Command::new("bitcoind");

        let chain = format!("-chain={}", self.config.as_chain_str());
        let data_dir = format!("-datadir={}", datadir.display());
        let rpc_port = format!("-rpcport={}", self.rpc_port);
        let rpc_bind = format!("-rpcbind=127.0.0.1:{}", self.rpc_port);
        let rpc_user = format!("-rpcuser={}", self.config.rpc_username);
        let rpc_password = format!("-rpcpassword={}", self.config.rpc_password);

        let mut args = vec![
            &chain,
            "-listen=0",
            &data_dir,
            &rpc_port,
            &rpc_bind,
            "-rpcallowip=127.0.0.1",
            "-fallbackfee=0.0002",
            "-server=1",
            "-prune=1",
            &rpc_user,
            &rpc_password,
        ];

        for arg in &self.config.extra_args {
            args.push(arg);
        }

        cmd.args(&args);

        // Capture both stdout and stderr for better error reporting
        cmd.stderr(Stdio::piped());
        cmd.stdout(Stdio::piped());

        let mut child = cmd.spawn()?;

        // Read stderr in a separate task
        let stderr = child.stderr.take().unwrap();
        let stderr_reader = tokio::io::BufReader::new(stderr);
        tokio::spawn(async move {
            let mut lines = stderr_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                error!("bitcoind stderr: {}", line);
            }
        });

        // Read stdout in a separate task
        let stdout = child.stdout.take().unwrap();
        let stdout_reader = tokio::io::BufReader::new(stdout);
        tokio::spawn(async move {
            let mut lines = stdout_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                info!("bitcoind stdout: {}", line);
            }
        });

        // Store the child process
        let mut child_guard = self.child.lock().await;
        *child_guard = Some(child);

        info!("Waiting for Bitcoin node to initialize...");
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Wait for node to be ready
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut attempts = 0;
        while Instant::now() < deadline {
            if let Some(child) = child_guard.as_mut() {
                if let Ok(Some(status)) = child.try_wait() {
                    let error = format!("Bitcoin node exited early with status: {}", status);
                    error!("{}", error);
                    anyhow::bail!(error);
                }
            }

            // Try to connect to RPC
            let client = reqwest::Client::new();
            match client
                .post(format!("http://127.0.0.1:{}/", self.rpc_port))
                .basic_auth(&self.config.rpc_username, Some(&self.config.rpc_password))
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "method": "getnetworkinfo",
                    "params": [],
                    "id": 1
                }))
                .send()
                .await
            {
                Ok(response) =>
                    if response.status().is_success() {
                        state.is_running = true;
                        info!("Bitcoin node started successfully on port {}", self.rpc_port);
                        return Ok(());
                    } else {
                        debug!(
                            "RPC request failed with status {} (attempt {})",
                            response.status(),
                            attempts
                        );
                    },
                Err(e) => {
                    debug!("Failed to connect to RPC (attempt {}): {}", attempts, e);
                }
            }

            attempts += 1;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }

        let error = format!(
            "Timed out waiting for Bitcoin node to start on port {} after {} attempts",
            self.rpc_port, attempts
        );
        error!("{}", error);
        anyhow::bail!(error);
    }

    async fn stop(&mut self) -> Result<()> {
        let mut state = self.state.write().await;
        if !state.is_running {
            return Ok(());
        }

        let child = self.child.lock().await.take();
        if let Some(mut child) = child {
            std::mem::drop(child.kill());
            std::mem::drop(child.wait());
        }

        state.is_running = false;
        Ok(())
    }

    async fn get_state(&self) -> Result<NodeState> { Ok(self.state.read().await.clone()) }

    fn rpc_port(&self) -> u16 { self.rpc_port }
}

impl Drop for BitcoinNodeManager {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.try_lock().ok().and_then(|mut guard| guard.take()) {
            std::mem::drop(child.kill());
            std::mem::drop(child.wait());
        }
    }
}

impl Default for BitcoinNodeManager {
    fn default() -> Self {
        Self::new_with_config(&TestConfig::default())
            .expect("Failed to create default BitcoinNodeManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extra_args() {
        let config = crate::test_config::TestConfig {
            extra_args: vec!["-debug=1".to_string()],
            ..crate::test_config::TestConfig::default()
        };

        let node_manager = BitcoinNodeManager::new_with_config(&config)
            .expect("Failed to create node manager with extra args");

        assert_eq!(node_manager.config.extra_args[0], "-debug=1");
    }
}
