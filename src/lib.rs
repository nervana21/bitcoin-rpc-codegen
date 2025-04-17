// src/lib.rs

// Export our patched serde_json (local_serde_json.rs)
#[path = "local_serde_json.rs"]
pub mod serde_json;
pub use serde_json::*;

// Re-export bitcoincore_rpc types for idiomatic use
pub use bitcoincore_rpc::{Auth, Error as RpcError, RpcApi};

/// Alias Result<T, RpcError> for simplicity
pub type Result<T> = std::result::Result<T, RpcError>;

/// Batteries-included RPC client with runtime version dispatch (v17–v28)
pub struct Client {
    inner: bitcoincore_rpc::Client,
}

impl Client {
    /// Connects, auto-detects Core version via getnetworkinfo, and errors if unsupported.
    pub fn new_auto(url: &str, user: &str, pass: &str) -> Result<Self> {
        let auth = Auth::UserPass(user.to_string(), pass.to_string());
        let rpc = bitcoincore_rpc::Client::new(url, auth)?;
        // Probe version
        let info: serde_json::Value = rpc.call("getnetworkinfo", &[])?;
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

    /// Generic raw JSON-RPC call
    pub fn call_json(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> Result<serde_json::Value> {
        self.inner.call(method, params)
    }

    /// Ergonomic: get the current block height
    pub fn getblockcount(&self) -> Result<u64> {
        self.inner.call("getblockcount", &[])
    }

    /// Ergonomic: get block hash at a given height
    pub fn getblockhash(&self, height: i64) -> Result<String> {
        self.inner
            .call("getblockhash", &[serde_json::Value::Number(height.into())])
    }

    /// Ergonomic: fetch full getblockchaininfo JSON
    pub fn getblockchaininfo(&self) -> Result<serde_json::Value> {
        self.inner.call("getblockchaininfo", &[])
    }
}
