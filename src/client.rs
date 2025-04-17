use crate::serde_json;
use anyhow::Result;
use bitcoincore_rpc::{Auth, Client as RpcClient, RpcApi};

// Bring in the generated client‑method implementations and type definitions.
// (OUT_DIR is set at build time by build.rs.)
include!(concat!(env!("OUT_DIR"), "/client/src/v28/blockchain.rs"));
include!(concat!(env!("OUT_DIR"), "/types/src/v28/blockchain.rs"));

/// A "batteries‑included" Bitcoin Core RPC client.
/// You can drop this straight into your code and call any RPC by name.
pub struct BitcoinRpcClient {
    rpc: RpcClient,
}

impl BitcoinRpcClient {
    /// Connects to a node at `rpc_url` with basic auth.
    pub fn new(rpc_url: &str, user: &str, password: &str) -> Result<Self> {
        let auth = Auth::UserPass(user.to_string(), password.to_string());
        let rpc = RpcClient::new(rpc_url, auth)?;
        Ok(BitcoinRpcClient { rpc })
    }

    /// Returns the chain's block height as JSON.
    /// For a strongly‑typed return, you can also call
    /// `impl_client_v28__getblockcount!();` (generated) instead.
    pub fn getblockcount(&self) -> Result<serde_json::Value> {
        self.rpc.call("getblockcount", &[]).map_err(|e| e.into())
    }

    /// Looks up the block hash at a particular height.
    pub fn getblockhash(&self, height: i64) -> Result<serde_json::Value> {
        self.rpc
            .call("getblockhash", &[serde_json::Value::Number(height.into())])
            .map_err(|e| e.into())
    }

    // If you'd rather have fully‑typed responses, you can invoke any of the
    // generated macros right here. For example:
    //
    // impl_client_v28__getblockchaininfo!();
    // impl_client_v28__getnetworkinfo!();
    // impl_client_v28__getrawmempool!();
    //
    // …and so on for every RPC in v28.
}
