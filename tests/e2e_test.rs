// tests/e2e_test.rs

use anyhow::{Context, Result};
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiMethod};
use bitcoin_rpc_codegen::Client;
use bitcoin_rpc_codegen::RegtestClient;
use serde_json::{json, Value};

/// Smoke‑test every RPC method (except "stop") by calling it with trivial dummy params
fn run_all_methods(client: &Client) -> Result<()> {
    // ensure wallet "test" exists
    let list: Value = client.call_json("listwalletdir", &[])?;
    let exists = list["wallets"]
        .as_array()
        .context("parsing listwalletdir response")?
        .iter()
        .any(|w| w.get("name").and_then(Value::as_str) == Some("test"));

    if !exists {
        println!("Creating wallet 'test'…");
        client.call_json("createwallet", &[json!("test")])?;
    } else {
        println!("Wallet 'test' already exists; skipping creation");
    }

    // attempt to load the wallet
    println!("Loading wallet 'test'…");
    let _ = client.call_json("loadwallet", &[json!("test")]);
    println!("Wallet 'test' loaded or already loaded");

    // load the deterministic API spec
    let api: Vec<ApiMethod> = {
        let s = include_str!("../resources/api.json");
        parse_api_json(s)?
    };
    println!("Loaded {} methods", api.len());

    // call each method with dummy args, skipping "stop"
    for m in &api {
        if m.name == "stop" {
            // we'll call this exactly once after the loop
            continue;
        }

        let params: Vec<Value> = m
            .arguments
            .iter()
            .map(|arg| match arg.type_.as_str() {
                "string" | "hex" => json!(""),
                "number" => json!(0),
                "boolean" => json!(false),
                "array" => json!([]),
                "object" | "object-named-parameters" => json!({}),
                _ => json!(null),
            })
            .collect();

        print!("Calling `{}` ({:2} params)… ", m.name, params.len());
        match client.call_json(&m.name, &params) {
            Ok(res) => println!("OK → {:?}", res),
            Err(e) => println!("ERR → {}", e),
        }
    }

    Ok(())
}

#[test]
fn e2e_all_methods() -> Result<()> {
    println!("=== e2e_all_methods START ===");

    // spawn regtest bitcoind with our helper (wallet "test")
    let rt = RegtestClient::new_auto("test")?;
    let client = &rt.client;

    // run the full‑surface smoke test (skips "stop")
    run_all_methods(client)?;

    println!("=== e2e_all_methods END ===");
    Ok(())
}
