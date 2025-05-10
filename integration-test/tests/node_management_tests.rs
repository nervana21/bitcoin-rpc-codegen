use anyhow::Result;
use integration_test::{
    create_test_client, create_test_config, create_test_node_manager, init_test_env, TestNode,
};

use node::NodeManager;
use std::time::Duration;

#[tokio::test]
async fn test_node_startup_shutdown() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    // Create and start node using TestNode
    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;

    // Verify node is running by making an RPC call
    let info = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await?;
    assert!(info.get("chain").is_some());

    // First try to stop the node gracefully using RPC
    test_node.client.call_method("stop", &[]).await?;

    // Wait for node to fully shut down
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Create a new client to avoid connection reuse
    let client = create_test_client(&config).await?;

    // Verify node is stopped by attempting an RPC call
    let result = client.call_method("getblockchaininfo", &[]).await;
    assert!(result.is_err(), "Node is still responding after shutdown");
    Ok(())
}

#[tokio::test]
async fn test_node_configuration() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    // Create and start node with custom config
    let mut node_manager = create_test_node_manager(&config).await?;
    node_manager.start().await?;

    // Wait for node to be ready
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Verify node is running on custom port
    let client = create_test_client(&config).await?;
    let info = client.call_method("getblockchaininfo", &[]).await?;
    assert!(info.get("chain").is_some());

    // Cleanup
    node_manager.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_node_restart() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    // Create and start node
    let mut node_manager = create_test_node_manager(&config).await?;
    node_manager.start().await?;

    // Wait for node to be ready
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Stop node
    node_manager.stop().await?;

    // Wait a bit
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Start node again
    node_manager.start().await?;

    // Wait for node to be ready
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Verify node is running again
    let client = create_test_client(&config).await?;
    let info = client.call_method("getblockchaininfo", &[]).await?;
    assert!(info.get("chain").is_some());

    // Cleanup
    node_manager.stop().await?;
    Ok(())
}
