// tests/e2e_test.rs

use anyhow::Result;
use bitcoin_rpc_codegen::BitcoinRpcClient;
use bitcoin_rpc_codegen::serde_json::{Value, from_value};

/// Connection settings for your regtest node.
const RPC_URL: &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "rpcuser";
const RPC_PASS: &str = "rpcpassword";

#[test]
fn e2e_test_getblockcount() -> Result<()> {
    // Instantiate the ready‑to‑use client.
    let client = BitcoinRpcClient::new(RPC_URL, RPC_USER, RPC_PASS)?;
    println!("Client instantiated successfully.");

    // Call the ergonomic method.
    let raw: Value = client.getblockcount()?;
    println!("Raw JSON response: {}", raw);

    // Convert to i64 and assert.
    let count: i64 = from_value(raw)?;
    println!("Current block count: {}", count);
    assert!(count >= 0, "Block count should be non‑negative");

    Ok(())
}

#[test]
fn e2e_test_getblockhash() -> Result<()> {
    let client = BitcoinRpcClient::new(RPC_URL, RPC_USER, RPC_PASS)?;
    println!("Client instantiated successfully.");

    // Look up the genesis‐height hash.
    let raw: Value = client.getblockhash(0)?;
    println!("Raw JSON response: {}", raw);

    // Convert to String and assert it's nonempty.
    let hash: String = from_value(raw)?;
    println!("Block hash at height 0: {}", hash);
    assert!(
        !hash.is_empty(),
        "Block hash should be a non‑empty hex string"
    );

    Ok(())
}
