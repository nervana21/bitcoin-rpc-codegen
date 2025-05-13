use anyhow::{Context, Result};
use integration_test::{
    create_test_client, create_test_config, create_test_node_manager, init_test_env,
    wait_for_condition, TestNode,
};
use node::NodeManager;
use std::time::Duration;
use tokio;

// Basic RPC Tests
#[tokio::test]
async fn test_get_blockchain_info() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;

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

    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;

    let info = test_node.client.call_method("getnetworkinfo", &[]).await?;
    assert!(info.get("version").and_then(|v| v.as_i64()).unwrap_or(0) > 0);
    assert!(info.get("subversion").is_some());

    Ok(())
}

// Regtest Connection Test
#[tokio::test]
async fn test_regtest_connection() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    let response = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await
        .context("Failed to call getblockchaininfo")?;

    let chain = response["chain"]
        .as_str()
        .context("Failed to get chain from response")?;

    assert!(chain == "regtest", "Expected regtest chain, got {}", chain);

    Ok(())
}

// Node Management Tests
#[tokio::test]
async fn test_node_startup_shutdown() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    // start & verify up
    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;
    let info = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await?;
    assert!(info.get("chain").is_some());

    // send stop RPC and assert immediate response
    let stop_msg = test_node.client.call_method("stop", &[]).await?;
    assert_eq!(stop_msg.as_str().unwrap(), "Bitcoin Core stopping");

    // now poll for shutdown instead of sleeping 5 s
    let client = create_test_client(&config).await?;
    let down = wait_for_condition(
        || async {
            // if RPC errors out, we're down
            Ok(client.call_method("getblockchaininfo", &[]).await.is_err())
        },
        Duration::from_secs(1),
    )
    .await?;

    assert!(down, "Node still responding after shutdown");
    Ok(())
}

#[tokio::test]
async fn test_node_configuration() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    let mut node_manager = create_test_node_manager(&config).await?;
    node_manager.start().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = create_test_client(&config).await?;
    let info = client.call_method("getblockchaininfo", &[]).await?;
    assert!(info.get("chain").is_some());

    node_manager.stop().await?;
    Ok(())
}

#[tokio::test]
async fn test_node_restart() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    let mut node_manager = create_test_node_manager(&config).await?;
    node_manager.start().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    node_manager.stop().await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    node_manager.start().await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    let client = create_test_client(&config).await?;
    let info = client.call_method("getblockchaininfo", &[]).await?;
    assert!(info.get("chain").is_some());

    node_manager.stop().await?;
    Ok(())
}
