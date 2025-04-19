// src/lib.rs

// Export our patched serde_json (local_serde_json.rs)
#[path = "local_serde_json.rs"]
pub mod serde_json;
pub use serde_json::*;

// Export parser for integration tests
pub mod parser;
pub use parser::{parse_api_json, ApiMethod};

// Re‑export bitcoincore_rpc types for ergonomic use
pub use bitcoincore_rpc::{Auth, Error as RpcError, RpcApi};

/// Alias used throughout this crate
pub type Result<T> = std::result::Result<T, RpcError>;

/// Thin wrapper around `bitcoincore_rpc::Client` that **auto‑detects Core
/// version** (v17 → v28).
pub struct Client {
    inner: bitcoincore_rpc::Client,
}

mod regtest;
pub use crate::regtest::{Conf, RegtestClient};

impl Client {
    pub fn new_with_auth(url: &str, auth: Auth) -> Result<Self> {
        let rpc = bitcoincore_rpc::Client::new(url, auth)?;
        version_probe(&rpc)?;
        Ok(Self { inner: rpc })
    }

    pub fn new_auto(url: &str, user: &str, pass: &str) -> Result<Self> {
        let auth = Auth::UserPass(user.to_string(), pass.to_string());
        Self::new_with_auth(url, auth)
    }

    pub fn call_json(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        self.inner.call(method, params)
    }

    /// Load `wallet_name` or create it if missing (regtest convenience).
    pub fn load_or_create_wallet(&self, wallet_name: &str) -> Result<()> {
        if self.call_json("loadwallet", &[json!(wallet_name)]).is_ok() {
            return Ok(());
        }
        match self.call_json("createwallet", &[json!(wallet_name)]) {
            Ok(_) => Ok(()),
            Err(e) if wallet_exists_err(&e) => Ok(()),
            Err(e) => Err(e),
        }
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
        .ok_or_else(|| RpcError::ReturnedError("missing version".into()))?;
    let major = (ver / 10_000) as u32;
    if !(17..=28).contains(&major) {
        return Err(RpcError::ReturnedError(format!(
            "unsupported Core v{major}"
        )));
    }
    Ok(())
}

fn wallet_exists_err(e: &RpcError) -> bool {
    let s = e.to_string();
    s.contains("already exists") || s.contains("already loaded")
}
