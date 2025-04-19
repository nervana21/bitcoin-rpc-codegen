// src/lib.rs

// Export our patched serde_json (local_serde_json.rs)
#[path = "local_serde_json.rs"]
pub mod serde_json;
pub use serde_json::*;

// Export parser for integration tests
pub mod parser;
pub use parser::{parse_api_json, ApiMethod};

// Re-export bitcoincore_rpc types for idiomatic use
pub use bitcoincore_rpc::{Auth, Error as RpcError, RpcApi};

/// Alias Result<T, RpcError> for simplicity
pub type Result<T> = std::result::Result<T, RpcError>;

/// Batteries-included RPC client with runtime version dispatch (v17–v28)
pub struct Client {
    inner: bitcoincore_rpc::Client,
}

/// A helper for regtest: spawns `bitcoind`, loads a wallet, tears down on Drop.
pub use crate::regtest::RegtestClient;

mod regtest;

impl Client {
    /// Connects, auto-detects Core version via getnetworkinfo, and errors if unsupported.
    pub fn new_auto(url: &str, user: &str, pass: &str) -> Result<Self> {
        let auth = Auth::UserPass(user.to_string(), pass.to_string());
        let rpc = bitcoincore_rpc::Client::new(url, auth)?;
        // Probe version
        let info = rpc.call::<serde_json::Value>("getnetworkinfo", &[])?;
        let ver_num = info
            .get("version")
            .and_then(|v| v.as_u64())
            .ok_or_else(|| RpcError::ReturnedError("missing version".into()))?;
        let major = (ver_num / 10_000) as u32;
        if !(17..=28).contains(&major) {
            return Err(RpcError::ReturnedError(format!(
                "unsupported Core v{}",
                major
            )));
        }
        Ok(Self { inner: rpc })
    }

    /// Generic raw JSON-RPC call (returns serde_json::Value)
    pub fn call_json(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        self.inner.call(method, params)
    }

    /// Try to load the named wallet, or create it if it doesn’t exist (regtest only).
    pub fn load_or_create_wallet(&self, wallet_name: &str) -> Result<()> {
        // skip if already loaded
        let loaded = self.call_json("listwallets", &[])?;
        if let Some(arr) = loaded.as_array() {
            if arr.iter().any(|v| v.as_str() == Some(wallet_name)) {
                return Ok(());
            }
        }

        // not yet loaded: try load, else create → load
        if self.call_json("loadwallet", &[json!(wallet_name)]).is_err() {
            let _ = self.call_json("createwallet", &[json!(wallet_name)]);
            self.call_json("loadwallet", &[json!(wallet_name)])?;
        }
        Ok(())
    }
}

impl RpcApi for Client {
    fn call<T: for<'de> serde::Deserialize<'de>>(
        &self,
        cmd: &str,
        params: &[serde_json::Value],
    ) -> bitcoincore_rpc::Result<T> {
        // Delegate straight to the inner bitcoincore_rpc::Client
        self.inner.call(cmd, params)
    }
}
