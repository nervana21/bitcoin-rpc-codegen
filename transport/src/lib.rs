// transport/src/lib.rs

use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};
use thiserror::Error;

/// Thin JSON‑RPC transport layer.
#[derive(Clone)]
pub struct Transport {
    client: Client,
    url: String,
}

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("RPC error: {0}")]
    Rpc(Value),
}

impl Transport {
    /// Create a new transport pointing at `url` (e.g. "http://127.0.0.1:18443").
    pub fn new<U: Into<String>>(url: U) -> Self {
        Transport {
            client: Client::new(),
            url: url.into(),
        }
    }

    /// Send a JSON‑RPC request with given `method` and `params`, returning the `result` field.
    pub async fn send_request<P: Serialize>(
        &self,
        method: &str,
        params: &[P],
    ) -> Result<Value, TransportError> {
        let req_body = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1,
        });

        let resp: Value = self
            .client
            .post(&self.url)
            .json(&req_body)
            .send()
            .await?
            .json()
            .await?;

        if let Some(err) = resp.get("error") {
            Err(TransportError::Rpc(err.clone()))
        } else {
            Ok(resp["result"].clone())
        }
    }
}
