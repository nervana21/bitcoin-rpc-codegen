// src/regtest.rs
// SPDX‑License‑Identifier: CC0‑1.0
//
// Disposable **regtest bitcoind** helper with early‑exit retry support.
//
// * spawns (or attaches to) a node using cookie‑auth only
// * waits for RPC, creates/loads a wallet
// * retries on early exit, picking a fresh port each time
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

/// Seconds to wait for RPC to come up or shut down.
const WAIT_SECS: u64 = 15;

/// Milliseconds we pause between retry attempts.
const RETRY_SLEEP_MS: u64 = 200;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Conf<'a> {
    /// Wallet that must exist (will be auto‑created).
    pub wallet_name: &'a str,
    /// Extra `bitcoind` flags.
    pub extra_args: Vec<&'a str>,
    /// Forward Core's stdout/stderr to the terminal.
    pub view_stdout: bool,
    /// Convenient `-txindex` toggle.
    pub enable_txindex: bool,
    /// How many times to retry if the process exits immediately.
    pub attempts: u8,
}

impl Default for Conf<'_> {
    fn default() -> Self {
        Self {
            wallet_name: "default",
            extra_args: Vec::new(),
            view_stdout: false,
            enable_txindex: false,
            attempts: 3,
        }
    }
}

/// A regtest `bitcoind` plus an already‑connected RPC client.
///
/// If we merely _attach_ to an existing node, `child` is `None` and
/// nothing is killed on drop.
pub struct RegtestClient {
    pub client: Client,
    child: Option<Child>,
    _datadir: TempDir, // kept alive so the cookie path stays valid
}

impl RegtestClient {
    /// Configurable constructor.
    pub fn new_with_conf(conf: &Conf<'_>) -> Result<Self> {
        let mut remaining = conf.attempts.max(1);

        loop {
            let rpc_port = get_available_port()?;
            match spawn_or_attach(rpc_port, conf) {
                Ok((child, datadir, cookie, rpc_url)) => {
                    // Connect via cookie auth
                    let client = Client::new_with_auth(&rpc_url, Auth::CookieFile(cookie))?;
                    client.load_or_create_wallet(conf.wallet_name)?;

                    return Ok(Self {
                        client,
                        child,
                        _datadir: datadir,
                    });
                }

                Err(e) if remaining > 1 => {
                    remaining -= 1;
                    eprintln!("bitcoind failed to start ({} retries left): {e}", remaining);
                    sleep(Duration::from_millis(RETRY_SLEEP_MS));
                    continue;
                }

                Err(e) => return Err(e),
            }
        }
    }

    pub fn new_auto(wallet_name: &str) -> Result<Self> {
        Self::new_with_conf(&Conf {
            wallet_name,
            ..Conf::default()
        })
    }

    /// Graceful stop, then hard‑kill after a grace period.
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

            // Final hammer if still running
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

/// Spawn a fresh node **or** attach to an already‑listening one.
fn spawn_or_attach(
    port: u16,
    conf: &Conf<'_>,
) -> Result<(Option<Child>, TempDir, PathBuf, String)> {
    if rpc_listening(port) {
        // -- attach ---------------------------------------------------------
        let dummy = TempDir::new()?; // placeholder; never used
        let cookie = default_cookie_path();
        let url = format!("http://{LOCALHOST}:{port}");
        wait_for_rpc_ready(&url, &cookie, None)?;
        Ok((None, dummy, cookie, url))
    } else {
        // -- spawn ----------------------------------------------------------
        let datadir = TempDir::new()?;
        let mut child = bitcoind_command(datadir.path(), port, conf)?.spawn()?;
        let url = format!("http://{LOCALHOST}:{port}");
        let cookie = cookie_path(datadir.path());

        match wait_for_rpc_ready(&url, &cookie, Some(&mut child)) {
            Ok(()) => Ok((Some(child), datadir, cookie, url)),
            Err(e) => {
                // make sure we reap the broken child before bubbling up
                let _ = child.kill();
                let _ = child.wait();
                Err(e)
            }
        }
    }
}

/// Build the full `bitcoind` command.
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

/// Wait until both the cookie exists **and** `getnetworkinfo` succeeds.
///
/// If `child` is supplied we also abort immediately on early exit.
fn wait_for_rpc_ready(url: &str, cookie: &Path, mut child: Option<&mut Child>) -> Result<()> {
    let start = Instant::now();
    loop {
        // Early‑exit detection
        if let Some(child) = child.as_mut() {
            if let Some(status) = child.try_wait()? {
                return Err(RpcError::ReturnedError(format!(
                    "bitcoind exited early with {status}"
                )));
            }
        }

        // Happy‑path check
        if cookie.exists() {
            let auth = Auth::CookieFile(cookie.to_path_buf());
            if let Ok(c) = bitcoincore_rpc::Client::new(url, auth) {
                if c.call::<Value>("getnetworkinfo", &[]).is_ok() {
                    break;
                }
            }
        }

        // Timeout
        if start.elapsed() > Duration::from_secs(WAIT_SECS) {
            return Err(RpcError::ReturnedError(format!(
                "bitcoind RPC never became ready at {url}"
            )));
        }
        sleep(Duration::from_millis(200));
    }
    Ok(())
}

/// Return `true` if anything is listening on `LOCALHOST:port`.
fn rpc_listening(port: u16) -> bool {
    TcpStream::connect((LOCALHOST, port)).is_ok()
}

/// `<datadir>/regtest/.cookie`
fn cookie_path(datadir: &Path) -> PathBuf {
    datadir.join("regtest/.cookie")
}

/// Fallback cookie path when we attach to an existing node.
fn default_cookie_path() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .unwrap_or_else(|| ".".into());
    PathBuf::from(home).join(".bitcoin/regtest/.cookie")
}

/// Bind to port 0 to let the OS hand us a free port.
fn get_available_port() -> Result<u16> {
    let l = TcpListener::bind((LOCALHOST, 0))?;
    Ok(l.local_addr()?.port())
}
