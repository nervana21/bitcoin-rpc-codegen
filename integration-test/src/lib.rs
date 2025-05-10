// integration-test/src/lib.rs

use anyhow::Result;
use client::RpcClient;
use node::test_config::TestConfig;
use node::BitcoinNodeManager;
use std::time::Duration;

mod test_node;
pub use test_node::TestNode;

/// Helper function to create a test configuration with an available port
pub fn create_test_config() -> Result<TestConfig> {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0))?;
    let port = listener.local_addr()?.port();
    let config = TestConfig {
        rpc_port: port,
        rpc_username: "rpcuser".to_string(),
        rpc_password: "rpcpassword".to_string(),
    };
    Ok(config)
}

/// Helper function to create a test node manager
pub async fn create_test_node_manager(config: &TestConfig) -> Result<BitcoinNodeManager> {
    let node_manager = BitcoinNodeManager::new_with_config(config)?;
    Ok(node_manager)
}

/// Helper function to create a test RPC client
pub async fn create_test_client(config: &TestConfig) -> Result<RpcClient> {
    let client = RpcClient::new_with_auth(
        format!("http://127.0.0.1:{}", config.rpc_port),
        &config.rpc_username,
        &config.rpc_password,
    );
    Ok(client)
}

/// Helper function to wait for a condition with timeout
pub async fn wait_for_condition<F, Fut>(condition: F, timeout: Duration) -> Result<bool>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<bool>>,
{
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if condition().await? {
            return Ok(true);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    Ok(false)
}

/// Initialize test environment with logging
pub fn init_test_env() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug,integration_test=debug,node=debug,client=debug")
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .try_init();
}
