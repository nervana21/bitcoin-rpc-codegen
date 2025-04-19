// examples/auto_client.rs

use bitcoin_rpc_codegen::{RegtestClient, Result, RpcApi};

const WALLET: &str = "test";

fn main() -> Result<()> {
    // one call does everything: spawn, wait, detect version, create/load wallet
    let rt = RegtestClient::new_auto(WALLET)?;
    let client = &rt.client;

    let height = client.get_block_count()?;
    println!("height = {}", height);

    let info = client.get_blockchain_info()?;
    println!("info = {:?}", info);

    let list_wallets: Vec<String> = client.list_wallets()?;
    println!("wallets: {:?}", list_wallets);

    Ok(())
}
