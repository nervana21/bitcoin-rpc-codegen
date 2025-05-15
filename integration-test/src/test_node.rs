// integration-test/src/test_node.rs

use anyhow::{Context, Result};
use client::RpcClient;
use node::{Config, NodeManager};
use std::time::{Duration, Instant};

/// Bundles a running node and a ready RPC client.
/// This struct is used for integration testing with a Bitcoin node.
pub struct TestNode {
    manager: Box<dyn NodeManager>,
    pub client: RpcClient,
}

impl TestNode {
    /// Start the node and wait until getnetworkinfo succeeds (or timeout).
    ///
    /// # Arguments
    /// * `manager` - The node manager that will control the Bitcoin node
    /// * `config` - The configuration containing RPC credentials
    ///
    /// # Returns
    /// A Result containing the TestNode if successful, or an error if the node fails to start
    /// or become ready within the timeout period.
    pub async fn spawn_and_ready(manager: Box<dyn NodeManager>, config: &Config) -> Result<Self> {
        // 1) spawn bitcoind
        manager
            .start()
            .await
            .context("Failed to start Bitcoin node")?;

        // Add initial delay to give node time to initialize
        tokio::time::sleep(Duration::from_secs(1)).await;

        // 2) build the client with authentication using the actual port from the manager
        let actual_port = manager.rpc_port();

        let client = RpcClient::new_with_auth(
            format!("http://127.0.0.1:{}", actual_port),
            &config.bitcoin.username,
            &config.bitcoin.password,
        );

        // 3) poll getnetworkinfo until it parses
        let deadline = Instant::now() + Duration::from_secs(30); // Increase timeout to 30 seconds
        let mut attempts = 0;
        while Instant::now() < deadline {
            match client.call_method("getnetworkinfo", &[]).await {
                Ok(_) => {
                    tracing::info!("Node ready after {} attempts", attempts);
                    return Ok(TestNode { manager, client });
                }
                Err(e) => {
                    attempts += 1;
                    // Add more detailed error logging
                    tracing::debug!(
                        error = ?e,
                        error_type = std::any::type_name::<dyn std::error::Error>(),
                        attempt = attempts,
                        "Node not ready yet: {}",
                        e
                    );
                    tokio::time::sleep(Duration::from_millis(500)).await; // Increase polling interval
                }
            }
        }

        anyhow::bail!("Timed out waiting for Bitcoin node RPC to become ready after 30 seconds");
    }

    /// Check if the node is ready by attempting to call getnetworkinfo.
    /// This is a non-blocking check that returns immediately.
    pub async fn is_ready(&self) -> bool {
        self.client.call_method("getnetworkinfo", &[]).await.is_ok()
    }

    /// Tear down the node and clean up resources.
    /// This should be called when the test node is no longer needed.
    pub async fn shutdown(mut self) -> Result<()> {
        self.manager
            .stop()
            .await
            .context("Failed to stop Bitcoin node")
    }
}
