// tests/roundtrip_test.rs

use anyhow::{Context, Result};
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiArgument};
use bitcoin_rpc_codegen::{RegtestClient, RpcApi};
use serde_json::{json, Value};
use std::fs;

/// Return a Vec of placeholder JSON values for each argument,
/// substituting valid blockhash and txid where recognized.
fn default_params(args: &[ApiArgument], best_block: &str, dummy_txid: &str) -> Vec<Value> {
    args.iter()
        .map(|arg| {
            let name = arg.names[0].to_lowercase();
            if name.contains("blockhash") {
                json!(best_block)
            } else if name.contains("txid") {
                json!(dummy_txid)
            } else {
                match arg.type_.as_str() {
                    "string" => json!(""),
                    "number" => json!(0),
                    "boolean" => json!(false),
                    "array" => json!([]),
                    "object" => json!({}),
                    _ => Value::Null,
                }
            }
        })
        .collect()
}

#[test]
fn roundtrip_generated() -> Result<()> {
    // 1. Spin up a regtest node and client
    let mut rt = RegtestClient::new_from_path(
        "test",
        "/Users/bitnode/bitcoin-versions/v29/bitcoin-29.0/bin/bitcoind",
    )?;

    let client = &rt.client;

    // 2. Prepare a funded chain and valid txid
    let new_addr: String = client.call("getnewaddress", &[])?;
    let _: Vec<String> =
        client.call("generatetoaddress", &[json!(101), json!(new_addr.clone())])?;
    let best_block: String = client.call("getbestblockhash", &[])?;
    let dummy_txid: String = client.call("sendtoaddress", &[json!(new_addr), json!(0.0001)])?;

    // 3. Load schema and iterate all methods except "stop"
    let schema_path = "resources/schemas/api_v29.json";

    let schema_src = fs::read_to_string(schema_path)
        .with_context(|| format!("❌ Failed to read schema file at `{}`", schema_path))?;

    let methods = parse_api_json(&schema_src)
        .with_context(|| format!("❌ Failed to parse API schema at `{}`", schema_path))?;
    for m in methods.into_iter().filter(|m| m.name != "stop") {
        let params = if m.arguments.is_empty() {
            Vec::new()
        } else {
            let p = default_params(&m.arguments, &best_block, &dummy_txid);
            println!("🛠 calling '{}' with {} dummy args", m.name, p.len());
            p
        };

        println!("🔁 Calling RPC method '{}'", m.name);
        match client.call_json(&m.name, &params) {
            Ok(resp) => println!("   → got {} bytes", resp.to_string().len()),
            Err(err) => {
                println!("⚠️ skipping '{}' due to RPC error: {}", m.name, err);
                continue;
            }
        }
    }

    // 4. Explicitly test the stop RPC
    println!("🔁 Calling RPC method 'stop'");
    let stop_resp: Value = client.call_json("stop", &[])?;
    println!(
        "   → stop RPC returned {} bytes",
        stop_resp.to_string().len()
    );

    // 5. Drop the immutable borrow before mutable teardown
    let _ = client;

    // 6. Wait for the node to exit cleanly
    rt.teardown()?;

    // 7. Verify that further RPCs now error
    println!("🌟 Verifying shutdown: subsequent RPCs should fail");
    assert!(
        rt.client.call_json("getblockcount", &[]).is_err(),
        "Expected RPC error after shutdown"
    );

    Ok(())
}
