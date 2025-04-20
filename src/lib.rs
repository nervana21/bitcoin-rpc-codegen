// src/lib.rs

// Export our patched serde_json (local_serde_json.rs)
#[path = "local_serde_json.rs"]
pub mod serde_json;
pub use serde_json::*;

// Export parser for integration tests
pub mod parser;
pub use parser::{parse_api_json, ApiMethod};

use bitcoincore_rpc::Auth;
pub use bitcoincore_rpc::RpcApi;
use std::{ops::Deref, sync::Arc};
use thiserror::Error;

/// Unified error type for this crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("RPC error: {0}")]
    Rpc(#[from] bitcoincore_rpc::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result specialized to our crate's `Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// Batteries‑included RPC client with runtime version dispatch (v17–v28).
#[derive(Debug, Clone)]
pub struct Client {
    inner: Arc<bitcoincore_rpc::Client>,
}

mod regtest;
pub use crate::regtest::{Conf, RegtestClient};

impl Client {
    /// Connect with explicit auth, then probe for supported version.
    pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
        let rpc = bitcoincore_rpc::Client::new(url, auth)?;
        version_probe(&rpc)?;
        Ok(Self {
            inner: Arc::new(rpc),
        })
    }

    /// Connect with user/pass, auto‑detect version.
    pub fn new_auto(url: &str, user: &str, pass: &str) -> Result<Self> {
        let auth = Auth::UserPass(user.to_string(), pass.to_string());
        Self::new_with_auth(url, auth)
    }

    /// Generic raw JSON‑RPC call.
    pub fn call_json(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        let v = self.inner.call(method, params)?;
        Ok(v)
    }

    /// Load or create a wallet (regtest convenience).
    pub fn load_or_create_wallet(&self, wallet_name: &str) -> Result<()> {
        // If loadwallet succeeds, we're done.
        if self
            .call_json("loadwallet", &[serde_json::json!(wallet_name)])
            .is_ok()
        {
            return Ok(());
        }
        // Otherwise try createwallet, but ignore "already exists"/"already loaded".
        match self.call_json("createwallet", &[serde_json::json!(wallet_name)]) {
            Ok(_) => Ok(()),
            Err(Error::Rpc(e)) if wallet_exists_err(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

/// Allow access to all underlying RPC methods.
impl Deref for Client {
    type Target = bitcoincore_rpc::Client;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl RpcApi for Client {
    fn call<T: for<'de> serde::Deserialize<'de>>(
        &self,
        cmd: &str,
        params: &[serde_json::Value],
    ) -> bitcoincore_rpc::Result<T> {
        self.inner.call(cmd, params)
    }
}

fn version_probe(rpc: &bitcoincore_rpc::Client) -> Result<()> {
    let info = rpc.call::<serde_json::Value>("getnetworkinfo", &[])?;
    let ver = info
        .get("version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| {
            Error::Rpc(bitcoincore_rpc::Error::ReturnedError(
                "missing version".into(),
            ))
        })?;
    let major = (ver / 10_000) as u32;
    if !(17..=28).contains(&major) {
        return Err(Error::Rpc(bitcoincore_rpc::Error::ReturnedError(format!(
            "unsupported Core v{major}"
        ))));
    }
    Ok(())
}

fn wallet_exists_err(e: &bitcoincore_rpc::Error) -> bool {
    let s = e.to_string();
    s.contains("already exists") || s.contains("already loaded")
}
