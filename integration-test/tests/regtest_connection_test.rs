use anyhow::{Context, Result};
use integration_test::{create_test_config, create_test_node_manager, init_test_env, TestNode};
use std::time::Duration;
use tokio;
#[tokio::test]
async fn test_regtest_connection() -> Result<()> {
    init_test_env();
    let config = create_test_config()?;

    // Start a test node using TestNode
    let test_node = TestNode::spawn_and_ready(
        Box::new(create_test_node_manager(&config).await?),
        &config.rpc_username,
        &config.rpc_password,
    )
    .await?;

    // Add initial delay to give node time to initialize
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Try to get blockchain info using the generic call_method
    let response = test_node
        .client
        .call_method("getblockchaininfo", &[])
        .await
        .context("Failed to call getblockchaininfo")?;

    // Parse the response
    let chain = response["chain"]
        .as_str()
        .context("Failed to get chain from response")?;

    // Verify we got a response
    assert!(chain == "regtest", "Expected regtest chain, got {}", chain);

    Ok(())
}
