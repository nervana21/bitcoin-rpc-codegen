// examples/auto_client.rs

use bitcoin_rpc_codegen::{RegtestClient, Result, RpcApi};

/// Replace these with your actual URL/creds when running locally.
const RPC_URL: &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "rpcuser";
const RPC_PASS: &str = "rpcpassword";
const WALLET: &str = "test";

fn main() -> Result<()> {
    // one call does everything: spawn, wait, detect version, create/load wallet
    let rt = RegtestClient::new_auto(RPC_URL, RPC_USER, RPC_PASS, WALLET)?;
    let client = &rt.client;

    // now just use `client` as usual
    let height = client.get_block_count()?;
    println!("height = {}", height);

    let info = client.get_blockchain_info()?;
    println!("info = {:?}", info);

    let list_wallets: Vec<String> = client.list_wallets()?;
    println!("wallets: {:?}", list_wallets);

    Ok(())
}
