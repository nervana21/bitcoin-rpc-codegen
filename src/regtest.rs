// src/regtest.rs

use crate::{Client, Result};
use std::{
    fs,
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

const DEFAULT_RPC_HOST: &str = "127.0.0.1";
const DEFAULT_RPC_PORT: u16 = 18443;

/// A helper that spawns a regtest `bitcoind`, auto-loads or creates a wallet,
/// and tears down `bitcoind` when dropped.
pub struct RegtestClient {
    /// The underlying RPC client, already connected and wallet-loaded.
    pub client: Client,
    child: Option<Child>,
}

impl RegtestClient {
    /// Spawns `bitcoind --regtest` (if needed), waits for RPC, then
    /// connects, auto-detects version, and loads/creates `wallet_name`.
    pub fn new_auto(url: &str, user: &str, pass: &str, wallet_name: &str) -> Result<Self> {
        // spawn bitcoind if no RPC is listening
        let child = if !rpc_listening() {
            let datadir = PathBuf::from("target/bitcoind-test");
            let _ = fs::remove_dir_all(&datadir);
            fs::create_dir_all(&datadir)?;

            Some(
                Command::new("bitcoind")
                    .args([
                        "-regtest",
                        &format!("-datadir={}", datadir.display()),
                        &format!("-rpcuser={}", user),
                        &format!("-rpcpassword={}", pass),
                        &format!("-rpcport={}", DEFAULT_RPC_PORT),
                        "-fallbackfee=0.0002",
                    ])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?,
            )
        } else {
            None
        };

        // wait up to 15s for RPC to come up
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(15) {
            if let Ok(c) = Client::new_auto(url, user, pass) {
                if c.call_json("getnetworkinfo", &[]).is_ok() {
                    break;
                }
            }
            sleep(Duration::from_millis(200));
        }

        // root client to load/create the wallet
        let root = Client::new_auto(url, user, pass)?;
        root.load_or_create_wallet(wallet_name)?;

        // now switch to wallet-scoped RPC endpoint
        let wallet_url = format!("{}{}{}", url.trim_end_matches('/'), "/wallet/", wallet_name);
        let client = Client::new_auto(&wallet_url, user, pass)?;

        Ok(RegtestClient { client, child })
    }

    /// Stops the spawned `bitcoind` (if any) via RPC and reaps the process.
    pub fn teardown(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            let _ = self.client.call_json("stop", &[]);
            let stop_start = Instant::now();
            while stop_start.elapsed() < Duration::from_secs(10) {
                if !rpc_listening() {
                    break;
                }
                sleep(Duration::from_millis(200));
            }
            let _ = child.wait()?;
        }
        Ok(())
    }
}

impl Drop for RegtestClient {
    fn drop(&mut self) {
        let _ = self.teardown();
    }
}

/// Returns true if something is listening on the default regtest RPC port.
fn rpc_listening() -> bool {
    TcpStream::connect((DEFAULT_RPC_HOST, DEFAULT_RPC_PORT)).is_ok()
}
