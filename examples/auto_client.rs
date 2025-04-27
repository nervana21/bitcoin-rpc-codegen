// examples/auto_client.rs

use anyhow::Result;
use bitcoin_rpc_codegen::{Conf, RegtestClient, RpcApi};

const WALLET: &str = "test";

fn main() -> Result<()> {
    println!("=== Demo 1: RegtestClient::new_auto ===");
    demo_auto(WALLET)?;

    println!("\n=== Demo 2: RegtestClient::new_with_conf ===");
    demo_with_conf()?;

    Ok(())
}

fn demo_auto(wallet: &str) -> anyhow::Result<()> {
    let rt = RegtestClient::new_auto(wallet)?;
    let client = &rt.client;

    let height = client.get_block_count()?;
    println!("  height  = {}", height);

    let info = client.get_blockchain_info()?;
    println!("  info    = {:?}", info);

    let list_wallets: Vec<String> = client.list_wallets()?;
    println!("  wallets = {:?}", list_wallets);

    Ok(())
}

/// Full‑featured constructor using a custom [`Conf`].
fn demo_with_conf() -> anyhow::Result<()> {
    let mut conf = Conf::default();
    conf.wallet_name = WALLET;
    conf.enable_txindex = true;
    conf.view_stdout = false; // show bitcoind stdout/stderr
    conf.extra_args = vec!["-deprecatedrpc=addresses"];

    let rt = RegtestClient::new_with_conf(&conf)?;
    let client = &rt.client;

    // Same RPC calls as before
    let height = client.get_block_count()?;
    println!("  height  = {}", height);

    let info = client.get_blockchain_info()?;
    println!("  info    = {:?}", info);

    let list_wallets: Vec<String> = client.list_wallets()?;
    println!("  wallets = {:?}", list_wallets);

    Ok(())
}
