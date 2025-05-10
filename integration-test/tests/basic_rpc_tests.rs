// integration-test/tests/basic_rpc_tests.rs

use anyhow::Result;
use integration_test::{create_test_config, create_test_node_manager, init_test_env, TestNode};

#[tokio::test]
async fn test_get_blockchain_info() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    // Now a single call spins up bitcoind, waits for RPC, and gives you a client:
    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;

    // straight to calling RPC
    let info = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await?;

    assert!(info.get("chain").is_some());
    assert!(info["blocks"].as_i64().unwrap_or(-1) >= 0);

    test_node.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_get_network_info() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    // Start a test node using TestNode
    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;

    // Test getnetworkinfo RPC call
    let info = test_node.client.call_method("getnetworkinfo", &[]).await?;
    assert!(info.get("version").and_then(|v| v.as_i64()).unwrap_or(0) > 0);
    assert!(info.get("subversion").is_some());

    // Cleanup is handled by TestNode's Drop implementation
    Ok(())
}
