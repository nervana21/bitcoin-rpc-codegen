// src/regtest.rs
//
// Step 1: use a fresh TempDir for every node and bind RPC on an
//         OS‑allocated free port instead of the fixed 18443.

use crate::{Client, Result};
use serde::de::Error as SerdeError;
use std::{
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};
use tempfile::TempDir;

/// Pick an unused local TCP port by binding to 0 and asking the OS.
fn get_available_port() -> u16 {
    TcpListener::bind(("127.0.0.1", 0))
        .expect("bind(0)")
        .local_addr()
        .unwrap()
        .port()
}

/// Minimal URL parser for "http://host:port[/...]".
fn host_port_from_url(url: &str) -> Option<(&str, u16)> {
    let url = url.strip_prefix("http://")?.split('/').next()?;
    let mut parts = url.split(':');
    let host = parts.next()?;
    let port = parts.next()?.parse().ok()?;
    Some((host, port))
}

/// A helper that spawns a regtest `bitcoind`, auto‑loads/creates a wallet,
/// and tears everything down on `Drop`.
pub struct RegtestClient {
    /// The ready‑to‑use RPC client.
    pub client: Client,
    /// Child handle if *we* spawned bitcoind.
    child: Option<Child>,
    /// Keeps the temp datadir alive for the lifetime of the node.
    _datadir: Option<TempDir>,
}

impl RegtestClient {
    /// If nothing is already listening at `url`, spin up a private
    /// regtest `bitcoind` in a temp directory on a free RPC port.
    pub fn new_auto(url: &str, user: &str, pass: &str, wallet: &str) -> Result<Self> {
        let (need_spawn, host, port) = match host_port_from_url(url) {
            Some((h, p)) if TcpStream::connect((h, p)).is_ok() => (false, h.to_string(), p),
            _ => (true, "127.0.0.1".into(), get_available_port()),
        };

        // Spawn bitcoind if required
        let (child, datadir) = if need_spawn {
            let dir = TempDir::new()?;
            let child = Command::new("bitcoind")
                .args([
                    "-regtest",
                    &format!("-datadir={}", dir.path().display()),
                    &format!("-rpcuser={}", user),
                    &format!("-rpcpassword={}", pass),
                    &format!("-rpcport={}", port),
                    "-fallbackfee=0.0002",
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?;
            (Some(child), Some(dir))
        } else {
            (None, None)
        };

        let rpc_url = format!("http://{}:{}", host, port);

        // Wait for RPC to come up (max 15 s)
        let start = Instant::now();
        loop {
            if let Ok(c) = Client::new_auto(&rpc_url, user, pass) {
                if c.call_json("getnetworkinfo", &[]).is_ok() {
                    // load/create wallet, then return
                    c.load_or_create_wallet(wallet)?;
                    return Ok(RegtestClient {
                        client: c,
                        child,
                        _datadir: datadir,
                    });
                }
            }
            if start.elapsed() > Duration::from_secs(15) {
                return Err(bitcoincore_rpc::Error::Json(serde_json::Error::custom(
                    format!("bitcoind RPC never became ready on {}", rpc_url),
                )));
            }
            sleep(Duration::from_millis(200));
        }
    }

    /// Stop the node (if we started it) via RPC and reap the process.
    pub fn teardown(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            let _ = self.client.call_json("stop", &[]);

            // give bitcoind up to 10 s to exit
            let t0 = Instant::now();
            loop {
                if let Some(status) = child.try_wait()? {
                    // exited
                    let _ = status;
                    break;
                }
                if t0.elapsed() > Duration::from_secs(10) {
                    let _ = child.kill();
                    break;
                }
                sleep(Duration::from_millis(200));
            }
        }
        Ok(())
    }
}

impl Drop for RegtestClient {
    fn drop(&mut self) {
        let _ = self.teardown();
        // `TempDir` cleans itself up automatically.
    }
}
