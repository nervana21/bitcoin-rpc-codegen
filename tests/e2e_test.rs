// tests/e2e_test.rs

use bitcoin_rpc_codegen::{Client, Result};

const RPC_URL: &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "rpcuser";
const RPC_PASS: &str = "rpcpassword";

#[test]
fn e2e_json_client_count() -> Result<()> {
    let client = Client::new_auto(RPC_URL, RPC_USER, RPC_PASS)?;
    let count = client.getblockcount()?;
    println!("count: {}", count);
    assert!(count > 0, "Block count should be positive");
    Ok(())
}

#[test]
fn e2e_json_client_hash() -> Result<()> {
    let client = Client::new_auto(RPC_URL, RPC_USER, RPC_PASS)?;
    let hash = client.getblockhash(0)?;
    println!("hash: {}", hash);
    assert!(!hash.is_empty(), "Block hash should not be empty");
    Ok(())
}

#[test]
fn e2e_json_client_info() -> Result<()> {
    let client = Client::new_auto(RPC_URL, RPC_USER, RPC_PASS)?;
    let info = client.getblockchaininfo()?;
    println!("info: {:?}", info);
    // e.g. assert_eq!(info.get("chain").and_then(|c| c.as_str()), Some("regtest"));
    Ok(())
}
