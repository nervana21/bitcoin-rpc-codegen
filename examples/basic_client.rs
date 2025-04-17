// examples/basic_client.rs

use bitcoin_rpc_codegen::{Client, Result};

/// Replace these with your actual URL/creds when running locally.
const RPC_URL: &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "rpcuser";
const RPC_PASS: &str = "rpcpassword";

fn main() -> Result<()> {
    // 1) connect (auto‐detects version)
    let client = Client::new_auto(RPC_URL, RPC_USER, RPC_PASS)?;

    // 2) get block count
    let count = client.getblockcount()?;
    println!("block count = {}", count);

    // 3) get the genesis‐block hash
    let hash = client.getblockhash(0)?;
    println!("genesis hash = {}", hash);

    // 4) get full blockchain info
    let info = client.getblockchaininfo()?;
    println!("chain info: {:?}", info);

    Ok(())
}
