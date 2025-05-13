// transport/src/lib.rs

use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
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
    #[error("HTTP error (status {0}): {1}")]
    Http(u16, #[source] reqwest::Error),

    #[error("RPC error: {0}")]
    Rpc(Value),

    #[error("Invalid JSON: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Missing result field in response")]
    MissingResult,
}

impl From<reqwest::Error> for TransportError {
    fn from(e: reqwest::Error) -> Self {
        if let Some(status) = e.status() {
            TransportError::Http(status.as_u16(), e)
        } else {
            // Use 0 as a sentinel value for network errors (timeouts, connection refused, etc.)
            // where no HTTP status code is available
            TransportError::Http(0, e)
        }
    }
}

impl Transport {
    /// Create a new transport pointing at `url` (e.g. "http://127.0.0.1:18443").
    pub fn new<U: Into<String>>(url: U) -> Self {
        Transport {
            client: Client::new(),
            url: url.into(),
        }
    }

    /// Create a new transport pointing at `url` with basic authentication.
    pub fn new_with_auth<U: Into<String>>(url: U, username: &str, password: &str) -> Self {
        let mut headers = HeaderMap::new();
        let auth =
            base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", username, password));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", auth)).unwrap(),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();

        Transport {
            client,
            url: url.into(),
        }
    }

    /// Send a JSON‑RPC request with given `method` and `params`, returning the `result` field.
    /// This is a low-level method that works with raw JSON values.
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
        } else if !resp.is_object() || resp.get("result").is_none() {
            Err(TransportError::MissingResult)
        } else {
            Ok(resp["result"].clone())
        }
    }

    /// Send a JSON‑RPC request with given `method` and `params`, returning a deserialized response.
    /// This is a high-level method that handles type conversion and serialization.
    pub async fn call<T: Serialize, R: DeserializeOwned>(
        &self,
        method: &str,
        params: &[T],
    ) -> Result<R, TransportError> {
        let response = self.send_request(method, params).await?;
        Ok(serde_json::from_value(response)?)
    }
}
