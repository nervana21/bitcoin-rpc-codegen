use anyhow::{Context, Result};
use integration_test::{init_test_env, wait_for_condition, TestNode};
use std::time::Duration;
use tokio;

// Basic RPC Tests
#[tokio::test]
async fn test_get_blockchain_info() -> Result<()> {
    init_test_env();

    let test_node = TestNode::new().await?;

    let info = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await?;

    assert_eq!(info.get("chain").unwrap().as_str().unwrap(), "regtest");
    assert!(info.get("blocks").unwrap().as_i64().unwrap() >= 0);

    Ok(())
}

#[tokio::test]
async fn test_get_network_info() -> Result<()> {
    init_test_env();

    let test_node = TestNode::new().await?;

    let info = test_node.client.call_method("getnetworkinfo", &[]).await?;
    assert!(info.get("version").and_then(|v| v.as_i64()).unwrap_or(0) > 0);
    assert!(info.get("subversion").is_some());

    Ok(())
}

// Regtest Connection Test
#[tokio::test]
async fn test_regtest_connection() -> Result<()> {
    init_test_env();

    let test_node = TestNode::new().await?;

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
    let test_node = TestNode::new().await?;

    let info = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await?;
    assert!(info.get("chain").is_some());

    // send stop RPC and assert immediate response
    let stop_msg = test_node.client.call_method("stop", &[]).await?;
    assert_eq!(stop_msg.as_str().unwrap(), "Bitcoin Core stopping");

    // now poll for shutdown instead of sleeping 5 s
    let down = wait_for_condition(
        || async {
            // if RPC errors out, we're down
            Ok(test_node
                .client
                .call_method("getblockchaininfo", &[])
                .await
                .is_err())
        },
        Duration::from_secs(1),
    )
    .await?;

    assert!(down, "Node still responding after shutdown");
    Ok(())
}

#[tokio::test]
async fn test_node_restart() -> Result<()> {
    init_test_env();

    let test_node = TestNode::new().await?;

    // Verify initial node is running
    let info = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await?;
    assert!(info.get("chain").is_some());

    // Stop the node
    let stop_msg = test_node.client.call_method("stop", &[]).await?;
    assert_eq!(stop_msg.as_str().unwrap(), "Bitcoin Core stopping");

    // Wait for node to be down
    let down = wait_for_condition(
        || async {
            Ok(test_node
                .client
                .call_method("getblockchaininfo", &[])
                .await
                .is_err())
        },
        Duration::from_secs(1),
    )
    .await?;
    assert!(down, "Node still responding after shutdown");

    // Create a new test node to simulate restart
    let new_test_node = TestNode::new().await?;

    // Verify the new node is running
    let info = new_test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await?;
    assert!(info.get("chain").is_some());

    Ok(())
}
