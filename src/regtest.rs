// src/regtest.rs – step 1 (dynamic port + tempdir with robust teardown)
// SPDX‑License‑Identifier: CC0‑1.0

//! Spawn a **private regtest `bitcoind`** on an **ephemeral datadir** and a
//! **random free RPC port**, auto‑load (or create) a wallet, expose a typed
//! [`Client`], and guarantee clean shutdown on `Drop`.
//!

use crate::{Client, Result, RpcError};
use std::{
    net::{SocketAddr, TcpStream},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

use tempfile::TempDir;

/// A helper that spawns a regtest node and cleans everything up automatically.
pub struct RegtestClient {
    /// Ready‑to‑use JSON‑RPC client (already wallet‑loaded).
    pub client: Client,
    child: Option<Child>,
    _datadir: TempDir, // keeps the directory alive for the lifetime
    rpc_port: u16,     // needed to poll for shutdown
}

impl RegtestClient {
    /// Start (or attach to) a regtest node and ensure `wallet_name` exists.
    ///
    /// *If an RPC server is already listening on the requested URL we **do not**
    /// spawn a new daemon.*  This lets callers reuse a shared dev node.
    pub fn new_auto(url: &str, user: &str, pass: &str, wallet_name: &str) -> Result<Self> {
        let default_port = 18443u16;
        let wants_spawn = !Self::rpc_listening(default_port);

        // Always create a fresh temp dir even if we don't end up spawning; this
        // simplifies lifetimes and keeps the type non‑`Option`.
        let datadir = TempDir::new()?;
        let mut child = None;
        let rpc_port;
        let rpc_url;

        if wants_spawn {
            rpc_port = get_available_port()?;
            rpc_url = format!("http://127.0.0.1:{rpc_port}");

            child = Some(
                Command::new("bitcoind")
                    .args([
                        "-regtest",
                        &format!("-datadir={}", datadir.path().display()),
                        &format!("-rpcuser={user}"),
                        &format!("-rpcpassword={pass}"),
                        &format!("-rpcport={rpc_port}"),
                        "-fallbackfee=0.0002",
                    ])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()?,
            );
        } else {
            // Re‑use the caller‑provided URL & port.
            rpc_url = url.to_string();
            rpc_port = url
                .rsplit(':')
                .next()
                .and_then(|p| p.parse().ok())
                .unwrap_or(default_port);
        }

        // Wait up to 15 s for the RPC server to come up.
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(15) {
            if let Ok(c) = Client::new_auto(&rpc_url, user, pass) {
                if c.call_json("getnetworkinfo", &[]).is_ok() {
                    // Ensure wallet
                    c.load_or_create_wallet(wallet_name)?;
                    return Ok(RegtestClient {
                        client: c,
                        child,
                        _datadir: datadir,
                        rpc_port,
                    });
                }
            }
            sleep(Duration::from_millis(250));
        }
        Err(RpcError::ReturnedError(
            "regtest RPC did not become ready within 15 seconds".into(),
        ))
    }

    /// Cleanly stop the node we spawned (if any).
    pub fn teardown(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            // Request shutdown via RPC.
            let _ = self.client.call_json("stop", &[]);

            // Poll the port for up to 10 s; exit sooner if closed.
            let t0 = Instant::now();
            while t0.elapsed() < Duration::from_secs(10) {
                if !Self::rpc_listening(self.rpc_port) {
                    break;
                }
                sleep(Duration::from_millis(200));
            }

            // If still alive, force‑kill.
            if child.try_wait()?.is_none() {
                let _ = child.kill();
            }
            let _ = child.wait();
        }
        Ok(())
    }

    /*‑‑‑ helpers ‑‑‑*/
    fn rpc_listening(port: u16) -> bool {
        TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], port))).is_ok()
    }
}

impl Drop for RegtestClient {
    fn drop(&mut self) {
        let _ = self.teardown();
    }
}

/// Bind port 0 to let the OS pick a free port, then return it.
fn get_available_port() -> std::io::Result<u16> {
    std::net::TcpListener::bind(("127.0.0.1", 0)).map(|l| l.local_addr().unwrap().port())
}
