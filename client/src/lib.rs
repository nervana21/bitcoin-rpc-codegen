// client/src/lib.rs

use serde_json::Value;
use thiserror::Error;
use transport::{Transport, TransportError};

/// High‑level Bitcoin RPC client.
#[derive(Clone)]
pub struct RpcClient {
    transport: Transport,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Transport(#[from] TransportError),
}

/// Construct with basic HTTP auth or no auth (bitcoin‑cli cookie file later).
impl RpcClient {
    /// Create a new client pointing at `url`, e.g. "http://127.0.0.1:18443".
    pub fn new<U: Into<String>>(url: U) -> Self {
        RpcClient {
            transport: Transport::new(url),
        }
    }

    /// Create a new client pointing at `url` with basic authentication.
    pub fn new_with_auth<U: Into<String>>(url: U, username: &str, password: &str) -> Self {
        RpcClient {
            transport: Transport::new_with_auth(url, username, password),
        }
    }

    /// Generic call: method name plus JSON‐serialized params.
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value, ClientError> {
        Ok(self.transport.send_request(method, params).await?)
    }
}
