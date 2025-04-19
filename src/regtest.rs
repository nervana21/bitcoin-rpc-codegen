// src/regtest.rs
// SPDX‑License‑Identifier: CC0‑1.0
//
// Disposable **regtest bitcoind** helper.
//
// * spawns or attaches to a node          (cookie‑auth only)
// * waits for RPC, loads/creates wallet
// * shuts the node down on Drop

use crate::{Client, Result, RpcError};
use bitcoincore_rpc::{Auth, RpcApi};
use serde_json::Value;
use std::{
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};
use tempfile::TempDir;

/// 127.0.0.1 for every regtest instance.
const LOCALHOST: &str = "127.0.0.1";

/// Seconds to wait for RPC to come up / shut down.
const WAIT_SECS: u64 = 15;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Conf<'a> {
    /// Wallet that must exist (will be auto‑created).
    pub wallet_name: &'a str,
    /// Extra `bitcoind` flags.
    pub extra_args: Vec<&'a str>,
    /// Forward Core's stdout/stderr to our terminal.
    pub view_stdout: bool,
    /// Convenient `-txindex` toggle.
    pub enable_txindex: bool,
}

impl Default for Conf<'_> {
    fn default() -> Self {
        Self {
            wallet_name: "default",
            extra_args: Vec::new(),
            view_stdout: false,
            enable_txindex: false,
        }
    }
}

pub struct RegtestClient {
    pub client: Client,
    child: Option<Child>,
    _datadir: TempDir, // kept alive
}

impl RegtestClient {
    /// Configurable constructor.
    pub fn new_with_conf(conf: &Conf<'_>) -> Result<Self> {
        let rpc_port = get_available_port()?;
        let (child, datadir, cookie, rpc_url) = spawn_or_attach(rpc_port, conf)?;

        // Build cookie‑auth RPC client
        let client = Client::new_with_auth(&rpc_url, Auth::CookieFile(cookie))?;
        client.load_or_create_wallet(conf.wallet_name)?;

        Ok(Self {
            client,
            child,
            _datadir: datadir,
        })
    }

    /// One‑liner: `RegtestClient::new_auto("wallet")`
    pub fn new_auto(wallet_name: &str) -> Result<Self> {
        Self::new_with_conf(&Conf {
            wallet_name,
            ..Conf::default()
        })
    }

    /// Graceful stop, then kill after a grace period.
    pub fn teardown(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            let _ = self.client.call_json("stop", &[]);
            let start = Instant::now();
            while start.elapsed() < Duration::from_secs(WAIT_SECS) {
                if child.try_wait()?.is_some() {
                    break;
                }
                sleep(Duration::from_millis(200));
            }
            if child.try_wait()?.is_none() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
        Ok(())
    }
}

impl Drop for RegtestClient {
    fn drop(&mut self) {
        let _ = self.teardown();
    }
}

/// Decide whether to spawn a new node or attach to an existing one.
fn spawn_or_attach(
    port: u16,
    conf: &Conf<'_>,
) -> Result<(Option<Child>, TempDir, PathBuf, String)> {
    if rpc_listening(port) {
        // Attach
        let dummy = TempDir::new()?;
        let cookie = default_cookie_path();
        let url = format!("http://{LOCALHOST}:{port}");
        Ok((None, dummy, cookie, url))
    } else {
        // Spawn
        let datadir = TempDir::new()?;
        let child = bitcoind_command(datadir.path(), port, conf)?.spawn()?;
        let url = format!("http://{LOCALHOST}:{port}");
        let cookie = cookie_path(datadir.path());

        wait_for_rpc_ready(&url, &cookie)?;
        Ok((Some(child), datadir, cookie, url))
    }
}

/// Build the fully‑parameterised bitcoind `Command`.
fn bitcoind_command(dir: &Path, port: u16, conf: &Conf<'_>) -> Result<Command> {
    let mut cmd = Command::new("bitcoind");
    cmd.args([
        "-regtest",
        &format!("-datadir={}", dir.display()),
        &format!("-rpcport={port}"),
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
    Ok(cmd)
}

/// Wait until RPC answers `getnetworkinfo` *and* cookie exists.
fn wait_for_rpc_ready(url: &str, cookie: &Path) -> Result<()> {
    let start = Instant::now();
    loop {
        if cookie.exists() {
            let auth = Auth::CookieFile(cookie.to_path_buf());
            if let Ok(c) = bitcoincore_rpc::Client::new(url, auth) {
                if c.call::<Value>("getnetworkinfo", &[]).is_ok() {
                    break;
                }
            }
        }
        if start.elapsed() > Duration::from_secs(WAIT_SECS) {
            return Err(RpcError::ReturnedError(format!(
                "bitcoind RPC never became ready at {url}"
            )));
        }
        sleep(Duration::from_millis(200));
    }
    Ok(())
}

/// True if *anything* is already listening on `LOCALHOST:port`.
fn rpc_listening(port: u16) -> bool {
    TcpStream::connect((LOCALHOST, port)).is_ok()
}

/// `<datadir>/regtest/.cookie`
fn cookie_path(datadir: &Path) -> PathBuf {
    datadir.join("regtest/.cookie")
}

/// Default cookie path when attaching.
fn default_cookie_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .unwrap_or_else(|| ".".into());
    PathBuf::from(home).join(".bitcoin/regtest/.cookie")
}

/// Get an unused port by binding to `0`.
fn get_available_port() -> Result<u16> {
    let l = TcpListener::bind((LOCALHOST, 0))?;
    Ok(l.local_addr()?.port())
}
