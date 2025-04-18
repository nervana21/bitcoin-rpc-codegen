// examples/auto_client.rs

use bitcoin_rpc_codegen::{Client, Result};
use std::{
    net::TcpStream,
    process::{Child, Command},
    thread::sleep,
    time::{Duration, Instant},
};

/// Replace these with your actual URL/creds when running locally.
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
    let datadir = std::path::PathBuf::from("target/bitcoind-test");
    let _ = std::fs::remove_dir_all(&datadir);
    std::fs::create_dir_all(&datadir)?;

    let child = Command::new("bitcoind")
        .arg("-regtest")
        .arg(format!("-datadir={}", datadir.display()))
        .arg(format!("-rpcuser={}", RPC_USER))
        .arg(format!("-rpcpassword={}", RPC_PASS))
        .arg(format!("-rpcport={}", RPC_PORT))
        .arg("-fallbackfee=0.0002")
        .spawn()?;
    Ok(child)
}

/// Wait until `getnetworkinfo` finally succeeds (or timeout after 15s)
fn wait_for_rpc_ready() -> Result<()> {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(15) {
        if let Ok(client) = Client::new_auto(RPC_URL, RPC_USER, RPC_PASS) {
            if client.call_json("getnetworkinfo", &[]).is_ok() {
                return Ok(());
            }
        }
        sleep(Duration::from_millis(200));
    }
    Err(bitcoincore_rpc::Error::ReturnedError(
        "bitcoind RPC never came up after 15s".into(),
    ))
}

fn main() -> Result<()> {
    // 1) Start bitcoind if not already running
    let maybe_child = if !rpc_listening() {
        println!("RPC not up, spawning regtest bitcoind...");
        let child = spawn_bitcoind()?;
        wait_for_rpc_ready()?;
        println!("bitcoind ready!");
        Some(child)
    } else {
        None
    };

    // 2) connect (auto-detects version)
    let client = Client::new_auto(RPC_URL, RPC_USER, RPC_PASS)?;

    // 3) get block count
    let count = client.getblockcount()?;
    println!("block count = {}", count);

    // 4) get the genesis-block hash
    let hash = client.getblockhash(0)?;
    println!("genesis hash = {}", hash);

    // 5) get full blockchain info
    let info = client.getblockchaininfo()?;
    println!("chain info: {:?}", info);

    // 6) If we spawned bitcoind, shut it down cleanly
    if let Some(mut child) = maybe_child {
        println!("Stopping bitcoind via RPC...");
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
        let status = child.wait()?;
        println!("bitcoind exited with {}", status);
    }

    Ok(())
}
