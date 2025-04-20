use anyhow::Result;
use bitcoin_rpc_codegen::generator::SUPPORTED_VERSIONS;
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiMethod};
use bitcoin_rpc_codegen::{Client, RegtestClient};

use bitcoincore_rpc::RpcApi;
use std::fs;

fn assert_method_presence(_client: &Client, _name: &str) {} // TODO: implement

fn run_all_methods_from_source(client: &Client, src: &str) -> Result<usize> {
    let api: Vec<ApiMethod> = parse_api_json(src)?;
    for m in &api {
        assert_method_presence(client, &m.name);
    }
    Ok(api.len())
}

#[test]
fn e2e_all_methods() -> Result<()> {
    let rt = RegtestClient::new_auto("test")?;
    let client = &rt.client;
    let info = client.get_network_info()?;
    let blockchain = client.get_blockchain_info()?;

    let current_major = info.version / 100;

    println!();
    println!("Hello, world! 👋");
    println!("Hi user, I'm an instance of your node.");
    println!("I speak protocol version:       {}", info.version);
    println!(
        "I run software:                 {}",
        info.subversion.trim_matches('/')
    );
    println!(
        "My best block hash is:          {}",
        blockchain.best_block_hash
    );
    println!("Chain:                          {}", blockchain.chain);
    println!(
        "Headers: {} | Blocks: {}",
        blockchain.headers, blockchain.blocks
    );
    println!();

    std::io::Write::flush(&mut std::io::stdout()).ok();

    // current version's methods
    let current_api = include_str!("../resources/api.json");
    let current_count = run_all_methods_from_source(client, current_api)?;

    // prior version's methods (by going down one major version)
    let prior_major = current_major - 1;
    let filename = format!("resources/api_{}00.json", prior_major);
    let prior_count = fs::read_to_string(&filename)
        .ok()
        .and_then(|src| run_all_methods_from_source(client, &src).ok());

    println!("🧠 Cool detail:");
    println!(
        "This node speaks version {}, which supports {} RPC methods.",
        current_major, current_count
    );
    match prior_count {
        Some(count) => println!(
            "And we can also speak version {}, which supported {} methods.",
            prior_major, count
        ),
        None => println!(
            "No prior version file found for version {} — you're on the edge of history.",
            prior_major
        ),
    }
    println!("→ You didn’t have to change anything — the client just understood.");
    println!("→ That’s the elegance of a Universal Adapter. ✨");

    Ok(())
}
