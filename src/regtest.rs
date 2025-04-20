// src/regtest.rs
// SPDX-License-Identifier: CC0-1.0
//
// Disposable **regtest bitcoind** helper.
//
// * always spawns a fresh regtest `bitcoind` using cookie-auth
// * waits for RPC, creates/loads a wallet
// * gracefully stops the node on Drop

use crate::{Client, Error, Result};
use bitcoincore_rpc::{Auth, RpcApi};
use serde_json::Value;
use std::{
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};
use tempfile::TempDir;

/// 127.0.0.1 for every regtest instance.
const LOCALHOST: &str = "127.0.0.1";
/// Seconds to wait for RPC to come up or shut down.
const WAIT_SECS: u64 = 15;
/// Milliseconds between retries.
const RETRY_SLEEP_MS: u64 = 200;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Conf<'a> {
    /// Wallet that must exist (will be auto‑created).
    pub wallet_name: &'a str,
    /// Extra `bitcoind` flags (e.g. `-deprecatedrpc=addresses`).
    pub extra_args: Vec<&'a str>,
    /// Forward Core's stdout/stderr to the terminal.
    pub view_stdout: bool,
    /// Enable `-txindex` on the node.
    pub enable_txindex: bool,
    /// How many times to retry spawning `bitcoind` if it exits immediately.
    pub attempts: usize,
}

impl Default for Conf<'_> {
    fn default() -> Self {
        Conf {
            wallet_name: "default",
            extra_args: Vec::new(),
            view_stdout: false,
            enable_txindex: false,
            attempts: 3,
        }
    }
}

/// Helper that spawns a regtest bitcoind, auto-loads/creates a wallet,
/// and tears down on Drop.
pub struct RegtestClient {
    pub client: Client,
    child: Child,
    _datadir: TempDir,
}

impl RegtestClient {
    /// Spawns a fresh node with the given config.
    pub fn new_with_conf(conf: &Conf<'_>) -> Result<Self> {
        let (child, datadir, cookie, rpc_url) = spawn_node(conf)?;
        let client = Client::new_with_auth(&rpc_url, Auth::CookieFile(cookie))?;
        client.load_or_create_wallet(conf.wallet_name)?;
        Ok(Self {
            client,
            child,
            _datadir: datadir,
        })
    }

    /// Convenience for `new_with_conf(&Conf::default())`.
    pub fn new_auto(wallet_name: &str) -> Result<Self> {
        // Initialize wallet_name directly, inheriting all other defaults
        let conf = Conf {
            wallet_name,
            ..Default::default()
        };
        Self::new_with_conf(&conf)
    }

    /// Gracefully stop then kill after timeout.
    pub fn teardown(&mut self) -> Result<()> {
        let _ = self.client.call_json("stop", &[]);
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(WAIT_SECS) {
            if let Ok(Some(_)) = self.child.try_wait() {
                return Ok(());
            }
            sleep(Duration::from_millis(RETRY_SLEEP_MS));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        Ok(())
    }
}

impl Drop for RegtestClient {
    fn drop(&mut self) {
        let _ = self.teardown();
    }
}

/// Spawn a fresh bitcoind regtest node.
fn spawn_node(conf: &Conf<'_>) -> Result<(Child, TempDir, PathBuf, String)> {
    let port = get_available_port()?;
    let datadir = TempDir::new()?;
    let mut cmd = Command::new("bitcoind");
    cmd.args([
        "-regtest",
        &format!("-datadir={}", datadir.path().display()),
        &format!("-rpcport={}", port),
        "-fallbackfee=0.0002",
    ]);
    if conf.enable_txindex {
        cmd.arg("-txindex");
    }
    cmd.args(&conf.extra_args);
    if conf.view_stdout {
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    } else {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }
    let mut child = cmd.spawn()?;
    let url = format!("http://{}:{}", LOCALHOST, port);
    let cookie = datadir.path().join("regtest").join(".cookie");
    wait_for_rpc_ready(&url, &cookie, &mut child)?;
    Ok((child, datadir, cookie, url))
}

/// Wait until cookie exists and RPC responds.
fn wait_for_rpc_ready(url: &str, cookie: &Path, child: &mut Child) -> Result<()> {
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Err(Error::Rpc(bitcoincore_rpc::Error::ReturnedError(format!(
                "bitcoind exited early with {status}"
            ))));
        }
        if cookie.exists() {
            let auth = Auth::CookieFile(cookie.to_path_buf());
            if let Ok(c) = bitcoincore_rpc::Client::new(url, auth) {
                if c.call::<Value>("getnetworkinfo", &[]).is_ok() {
                    return Ok(());
                }
            }
        }
        if start.elapsed() > Duration::from_secs(WAIT_SECS) {
            return Err(Error::Rpc(bitcoincore_rpc::Error::ReturnedError(format!(
                "bitcoind RPC never became ready at {url}"
            ))));
        }
        sleep(Duration::from_millis(RETRY_SLEEP_MS));
    }
}

/// Bind to port 0 to let OS allocate a free port.
fn get_available_port() -> Result<u16> {
    let listener = TcpListener::bind((LOCALHOST, 0))?;
    Ok(listener.local_addr()?.port())
}
