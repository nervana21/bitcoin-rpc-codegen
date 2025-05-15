//! High‑level, ergonomic wrapper around [`transport::Transport`].
//!
//! `RpcClient` provides a strongly-typed interface for making Bitcoin RPC calls.
//! It supports both low-level raw JSON calls and high-level typed calls.
//!
//! ```rust,ignore
//! let client = RpcClient::new("http://127.0.0.1:8332");
//! let height: GetblockcountResponse = client
//!     .call("getblockcount", &[])
//!     .await?;
//! println!("block height = {}", height.count);
//! ```

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use thiserror::Error;
pub use transport::{Transport, TransportError};

/// High-level client for Bitcoin RPC calls
#[derive(Clone)]
pub struct RpcClient {
    transport: Transport,
}

/// Errors that can occur during RPC calls
#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("RPC error: {0}")]
    RpcError(String),
}

impl RpcClient {
    /// Create a new client with the given URL
    pub fn new<U: Into<String>>(url: U) -> Self {
        Self {
            transport: Transport::new(url),
        }
    }

    /// Create a new client with HTTP Basic authentication
    pub fn new_with_auth<U: Into<String>>(url: U, username: &str, password: &str) -> Self {
        Self {
            transport: Transport::new_with_auth(url, username, password),
        }
    }

    /// Make a low-level RPC call, returning the raw result as JSON
    pub async fn call_raw(&self, method: &str, params: &[Value]) -> Result<Value, ClientError> {
        self.transport
            .send_request(method, params)
            .await
            .map_err(Into::into)
    }

    /// Make a typed RPC call, automatically handling serialization/deserialization
    pub async fn call<T: Serialize, R: DeserializeOwned>(
        &self,
        method: &str,
        params: &[T],
    ) -> Result<R, ClientError> {
        self.transport
            .call(method, params)
            .await
            .map_err(Into::into)
    }
}

// For backward compatibility
impl RpcClient {
    /// Alias for `call_raw` to maintain backward compatibility
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value, ClientError> {
        self.call_raw(method, params).await
    }
}
