// src/regtest.rs
// SPDX‑License‑Identifier: CC0‑1.0
//
// Disposable **regtest bitcoind** helper that:
//
// * Creates a fresh `TempDir` datadir
// * Chooses a random free RPC port
// * Spawns `bitcoind -regtest` without any -rpcuser/-rpcpassword flags
// * Uses Core's cookie file for authentication
// * Waits until RPC is ready, then returns a fully‑typed `Client`
// * Sends `stop`, waits, and finally `kill`s the child on `Drop`

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

/// Loopback host used for every regtest instance.
const LOCALHOST: &str = "127.0.0.1";

/// How long to wait for RPC to come up / shut down.
const WAIT_SECS: u64 = 15;

/// Helper returned to callers.  Tears everything down automatically.
pub struct RegtestClient {
    pub client: Client,
    child: Option<Child>,
    _datadir: TempDir, // kept alive for the whole lifetime
}

impl RegtestClient {
    /// Spawn an isolated node and return a ready `Client`.
    ///
    /// If *another* node is already listening on the chosen port, this helper
    /// falls back to *connecting* to it (still via cookie auth) instead of
    /// spawning a second instance – useful when running multiple tests in
    /// parallel that share a single regtest fixture.
    pub fn new_auto(wallet_name: &str) -> Result<Self> {
        // 1) Pick an unused port & form the RPC URL
        let rpc_port = get_available_port()?;
        let rpc_url = format!("http://{LOCALHOST}:{rpc_port}");

        // 2) Decide whether we need to spawn `bitcoind`
        let (child, datadir) = if !rpc_listening(rpc_port) {
            let datadir = TempDir::new()?;
            let child = spawn_bitcoind(&datadir, rpc_port)?;
            wait_for_rpc_ready(&rpc_url, &cookie_path(datadir.path()))?;
            (Some(child), datadir)
        } else {
            // Someone else is running a node; create throw‑away TempDir to keep
            // the struct layout the same.
            (None, TempDir::new()?)
        };

        // 3) Build Auth from the cookie in *our* datadir (or default one)
        let cookie = if child.is_some() {
            cookie_path(datadir.path())
        } else {
            default_cookie_path()
        };
        let auth = Auth::CookieFile(cookie);

        // 4) Connect our typed wrapper & ensure the requested wallet exists
        let client = Client::new_with_auth(&rpc_url, auth)?;
        client.load_or_create_wallet(wallet_name)?;

        Ok(RegtestClient {
            client,
            child,
            _datadir: datadir,
        })
    }

    /// Attempt polite shutdown, then force‑kill after a grace period.
    pub fn teardown(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            // Fire‑and‑forget RPC stop (ignore any error)
            let _ = self.client.call_json("stop", &[]);

            let start = Instant::now();
            while start.elapsed() < Duration::from_secs(WAIT_SECS) {
                if child.try_wait()?.is_some() {
                    break; // exited
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

// ---------------------------------------------------------------------------
// Internals
// ---------------------------------------------------------------------------

/// Start `bitcoind -regtest` bound to `LOCALHOST:port`, storing data in `dir`.
fn spawn_bitcoind(dir: &TempDir, port: u16) -> Result<Child> {
    let child = Command::new("bitcoind")
        .args([
            "-regtest",
            &format!("-datadir={}", dir.path().display()),
            &format!("-rpcport={port}"),
            "-fallbackfee=0.0002",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    Ok(child)
}

/// Wait until RPC responds to `getnetworkinfo` *and* the cookie file exists.
fn wait_for_rpc_ready(url: &str, cookie: &Path) -> Result<()> {
    let start = Instant::now();
    loop {
        if cookie.exists() {
            let auth = Auth::CookieFile(cookie.to_path_buf());
            if let Ok(c) = bitcoincore_rpc::Client::new(url, auth) {
                if c.call::<Value>("getnetworkinfo", &[]).is_ok() {
                    break; // ready!
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

/// True if something is already listening on `LOCALHOST:port`.
fn rpc_listening(port: u16) -> bool {
    TcpStream::connect((LOCALHOST, port)).is_ok()
}

/// Build `<datadir>/regtest/.cookie`.
fn cookie_path(datadir: &Path) -> PathBuf {
    datadir.join("regtest").join(".cookie")
}

/// Fallback path when using a node we did not spawn ourselves.
fn default_cookie_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE")) // Windows
        .unwrap_or_else(|| ".".into());
    PathBuf::from(home)
        .join(".bitcoin")
        .join("regtest")
        .join(".cookie")
}

/// Pick an unused TCP port by binding to 0 and reading back the assigned port.
fn get_available_port() -> Result<u16> {
    let listener = TcpListener::bind((LOCALHOST, 0))?;
    Ok(listener.local_addr()?.port())
}
