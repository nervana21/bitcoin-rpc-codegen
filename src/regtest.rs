// src/regtest.rs
// SPDX-License-Identifier: CC0-1.0
//
// Disposable **regtest bitcoind** helper.
//
// * always spawns a fresh regtest `bitcoind` using cookie-auth
// * waits for RPC, creates/loads a wallet
// * gracefully stops the node on Drop
// * now supports multi-wallet RPC prefixes cleanly

use crate::{Error, Result};
use bitcoincore_rpc::{Auth, Client as RawClient, RpcApi};
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};
use tempfile::TempDir;

/// 127.0.0.1 for every regtest instance.
const LOCALHOST: &str = "127.0.0.1";
/// Seconds to wait for RPC to come up or shut down.
const WAIT_SECS: u64 = 5;
/// Milliseconds between retries.
const RETRY_SLEEP_MS: u64 = 200;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Conf<'a> {
    pub wallet_name: &'a str,
    pub extra_args: Vec<&'a str>,
    pub view_stdout: bool,
    pub enable_txindex: bool,
    pub attempts: usize,
    pub bitcoind_path: Option<PathBuf>,
}

impl Default for Conf<'_> {
    fn default() -> Self {
        Conf {
            wallet_name: "default",
            extra_args: Vec::new(),
            view_stdout: false,
            enable_txindex: false,
            attempts: 3,
            bitcoind_path: None,
        }
    }
}

/// Lightweight client that tracks associated wallet (if any).
pub struct Client {
    raw: RawClient,
    wallet: Option<String>,
}

impl Client {
    pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
        Ok(Client {
            raw: RawClient::new(url, auth)?,
            wallet: None,
        })
    }

    pub fn load_or_create_wallet(&self, wallet_name: &str) -> Result<()> {
        // Try to load existing wallet
        if self
            .raw
            .call::<Value>("loadwallet", &[wallet_name.into()])
            .is_ok()
        {
            return Ok(());
        }
        // Otherwise, create a new one
        self.raw
            .call::<Value>("createwallet", &[wallet_name.into()])
            .map(|_| ())
            .map_err(Error::Rpc)
    }

    pub fn with_wallet(mut self, wallet_name: impl Into<String>) -> Self {
        self.wallet = Some(wallet_name.into());
        self
    }

    pub fn call_json(&self, method: &str, params: &[Value]) -> Result<Value> {
        let scoped_method = if self.requires_wallet_prefix(method) {
            if let Some(wallet) = &self.wallet {
                format!("wallet/{}/{}", wallet, method)
            } else {
                method.to_string()
            }
        } else {
            method.to_string()
        };
        self.raw
            .call::<Value>(&scoped_method, params)
            .map_err(Error::Rpc)
    }

    fn requires_wallet_prefix(&self, method: &str) -> bool {
        matches!(
            method,
            "abandontransaction"
                | "abortrescan"
                | "addmultisigaddress"
                | "backupwallet"
                | "bumpfee"
                | "createwallet"
                | "dumpprivkey"
                | "dumpwallet"
                | "encryptwallet"
                | "fundrawtransaction"
                | "getaddressesbylabel"
                | "getaddressinfo"
                | "getbalance"
                | "getnewaddress"
                | "getrawchangeaddress"
                | "getreceivedbyaddress"
                | "getreceivedbylabel"
                | "gettransaction"
                | "getwalletinfo"
                | "importaddress"
                | "importdescriptors"
                | "importmulti"
                | "importprivkey"
                | "importprunedfunds"
                | "keypoolrefill"
                | "listaddressgroupings"
                | "listlabels"
                | "listlockunspent"
                | "listreceivedbyaddress"
                | "listreceivedbylabel"
                | "listsinceblock"
                | "listtransactions"
                | "listunspent"
                | "loadwallet"
                | "lockunspent"
                | "removeprunedfunds"
                | "rescanblockchain"
                | "sendmany"
                | "sendtoaddress"
                | "sethdseed"
                | "setlabel"
                | "settxfee"
                | "signmessage"
                | "signrawtransactionwithwallet"
                | "unloadwallet"
                | "walletcreatefundedpsbt"
                | "walletlock"
                | "walletpassphrase"
                | "walletpassphrasechange"
                | "walletprocesspsbt"
        )
    }
}

/// Regtest node controller
pub struct RegtestClient {
    pub client: RawClient,
    child: Child,
    _datadir: TempDir,
}

impl RegtestClient {
    pub fn new_with_conf(conf: &Conf<'_>) -> Result<Self> {
        let (child, datadir, cookie, rpc_url) = spawn_node(conf)?;
        let client = RawClient::new(&rpc_url, Auth::CookieFile(cookie))?;

        // Load or create wallet manually
        let _ = client
            .call::<Value>("loadwallet", &[conf.wallet_name.into()])
            .or_else(|_| client.call::<Value>("createwallet", &[conf.wallet_name.into()]))?;

        Ok(Self {
            client,
            child,
            _datadir: datadir,
        })
    }

    pub fn new_auto(wallet_name: &str) -> Result<Self> {
        let conf = Conf {
            wallet_name,
            ..Default::default()
        };
        Self::new_with_conf(&conf)
    }

    pub fn teardown(&mut self) -> Result<()> {
        let _ = self.client.call::<Value>("stop", &[]);
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

fn spawn_node(conf: &Conf<'_>) -> Result<(Child, TempDir, PathBuf, String)> {
    let mut last_err = None;
    for attempt in 1..=conf.attempts {
        let datadir = TempDir::new()?;
        let port = get_available_port()?;
        let url = format!("http://{}:{}", LOCALHOST, port);
        let cookie = datadir.path().join("regtest").join(".cookie");

        let bitcoind_bin = conf
            .bitcoind_path
            .clone()
            .unwrap_or_else(|| PathBuf::from("bitcoind"));

        let mut cmd = Command::new(&bitcoind_bin);
        cmd.args([
            "-regtest",
            "-listen=0",
            &format!("-datadir={}", datadir.path().display()),
            &format!("-rpcport={}", port),
            &format!("-rpcbind=127.0.0.1:{}", port),
            "-rpcallowip=127.0.0.1",
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

        let child_result = cmd.spawn();
        let mut child = child_result?;
        match wait_for_rpc_ready(&url, &cookie, &mut child) {
            Ok(()) => return Ok((child, datadir, cookie, url)),
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                last_err = Some(e);
                if attempt < conf.attempts {
                    sleep(Duration::from_millis(RETRY_SLEEP_MS));
                    continue;
                }
            }
        }
    }
    Err(last_err.unwrap())
}

pub fn get_available_port() -> Result<u16> {
    let listener = std::net::TcpListener::bind((LOCALHOST, 0))?;
    Ok(listener.local_addr()?.port())
}

pub fn wait_for_rpc_ready(url: &str, cookie: &Path, child: &mut Child) -> Result<()> {
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            let msg = format!("bitcoind exited early with {}", status);
            return Err(Error::Rpc(bitcoincore_rpc::Error::ReturnedError(msg)));
        }
        if cookie.exists() {
            let auth = Auth::CookieFile(cookie.to_path_buf());
            if let Ok(c) = RawClient::new(url, auth) {
                if c.call::<Value>("getnetworkinfo", &[]).is_ok() {
                    return Ok(());
                }
            }
        }
        if start.elapsed() > Duration::from_secs(WAIT_SECS) {
            let msg = format!("bitcoind RPC never became ready at {}", url);
            return Err(Error::Rpc(bitcoincore_rpc::Error::ReturnedError(msg)));
        }
        sleep(Duration::from_millis(RETRY_SLEEP_MS));
    }
}
