// tests/roundtrip.rs

use anyhow::{Context, Result};
use bitcoin_rpc_codegen::Client;
use bitcoin_rpc_codegen::parser::{ApiMethod, parse_api_json};
use bitcoincore_rpc::{Auth as RpcAuth, Client as RawClient, RpcApi};
use serde_json::{Value, json};

use std::{
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command},
    thread::sleep,
    time::{Duration, Instant},
};

const RPC_HOST: &str = "127.0.0.1";
const RPC_PORT: u16 = 18443;
const RPC_URL: &str = "http://127.0.0.1:18443";
const RPC_USER: &str = "rpcuser";
const RPC_PASS: &str = "rpcpassword";

/// Returns true if something is listening on the RPC port
fn rpc_listening() -> bool {
    TcpStream::connect((RPC_HOST, RPC_PORT)).is_ok()
}

/// Spawns a fresh regtest `bitcoind` under `target/bitcoind-test`
fn spawn_bitcoind() -> Result<Child> {
    let datadir = PathBuf::from("target/bitcoind-test");
    let _ = std::fs::remove_dir_all(&datadir);
    std::fs::create_dir_all(&datadir).context("creating datadir")?;

    let child = Command::new("bitcoind")
        .arg("-regtest")
        .arg(format!("-datadir={}", datadir.display()))
        .arg(format!("-rpcuser={}", RPC_USER))
        .arg(format!("-rpcpassword={}", RPC_PASS))
        .arg(format!("-rpcport={}", RPC_PORT))
        .arg("-fallbackfee=0.0002")
        .spawn()
        .context("spawning bitcoind")?;
    Ok(child)
}

/// Wait until `getnetworkinfo` finally succeeds (or panic after 15s)
fn wait_for_rpc_ready() -> Result<()> {
    let raw = RawClient::new(RPC_URL, RpcAuth::UserPass(RPC_USER.into(), RPC_PASS.into()))
        .context("connect raw RPC")?;

    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(15) {
        if raw.call::<Value>("getnetworkinfo", &[]).is_ok() {
            return Ok(());
        }
        sleep(Duration::from_millis(200));
    }
    panic!("bitcoind RPC never came up after 15s");
}

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
fn roundtrip_all_methods() -> Result<()> {
    println!("=== roundtrip_all_methods START ===");

    // 1) Start/regtest bitcoind if needed
    let maybe_child = if !rpc_listening() {
        println!("RPC not up, spawning regtest bitcoind…");
        let child = spawn_bitcoind()?;
        wait_for_rpc_ready()?;
        println!("bitcoind ready!");
        Some(child)
    } else {
        None
    };

    // 2) Connect your generated client
    let client = Client::new_auto(RPC_URL, RPC_USER, RPC_PASS)?;
    // 3) Run the full‑surface smoke test (skips "stop")
    run_all_methods(&client)?;

    // 4) If we spawned it, shut it down cleanly
    if let Some(mut child) = maybe_child {
        println!("Stopping bitcoind via RPC…");
        // fire‑and‑forget stop, ignore any error
        let _ = client.call_json("stop", &[]);

        // wait up to 10s for the RPC port to go away
        let stop_start = Instant::now();
        while stop_start.elapsed() < Duration::from_secs(10) {
            if !rpc_listening() {
                break;
            }
            sleep(Duration::from_millis(200));
        }

        // finally reap the process
        let status = child.wait().context("waiting for bitcoind exit")?;
        println!("bitcoind exited with {}", status);
    }

    println!("=== roundtrip_all_methods END ===");
    Ok(())
}
