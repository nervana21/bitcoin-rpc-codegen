#![warn(missing_docs)]

//! **`Transport`**
//! A thin, configurable JSON‑RPC over HTTP transport layer for communicating with Bitcoin Core nodes.
//!
//! Features:
//! - HTTP client setup with optional basic authentication via `new_with_auth`
//! - Low‑level `send_request` returning raw `serde_json::Value` for maximum flexibility
//! - High‑level `call` with automatic serialization/deserialization to Rust types
//! - Unified error handling through the `TransportError` enum, covering HTTP, RPC, and JSON errors

use base64::Engine;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

/// Encapsulates an HTTP client and endpoint URL for sending JSON‑RPC requests.
#[derive(Clone)]
pub struct Transport {
    client: Client,
    url: String,
}

impl std::fmt::Debug for Transport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transport")
            .field("url", &self.url)
            .field("client", &"<reqwest::Client>")
            .finish()
    }
}

/// Errors that can occur while sending or receiving JSON‑RPC requests.
#[derive(Debug, Error)]
pub enum TransportError {
    /// HTTP transport error, includes status code and underlying transport error.
    #[error("HTTP error (status {0}): {1}")]
    Http(u16, #[source] reqwest::Error),

    /// The JSON‑RPC response contained an error object.
    #[error("RPC error: {0}")]
    Rpc(Value),

    /// Failed to parse JSON response.
    #[error("Invalid JSON: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The JSON‑RPC response did not include a `result` field.
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
    ///
    /// # Parameters
    /// - `url`: The HTTP endpoint of the Bitcoin Core JSON‑RPC server.
    pub fn new<U: Into<String>>(url: U) -> Self {
        Transport {
            client: Client::new(),
            url: url.into(),
        }
    }

    /// Create a new transport with HTTP basic authentication.
    ///
    /// # Parameters
    /// - `url`: The HTTP endpoint of the Bitcoin Core JSON‑RPC server.
    /// - `rpcuser`: RPC username.
    /// - `rpcpass`: RPC password.
    ///
    /// # Panics
    /// Panics if default headers cannot be constructed.
    pub fn new_with_auth<U: Into<String>>(url: U, rpcuser: &str, rpcpass: &str) -> Self {
        let mut headers = HeaderMap::new();
        let auth =
            base64::engine::general_purpose::STANDARD.encode(format!("{}:{}", rpcuser, rpcpass));
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

    /// Send a JSON‑RPC request with given `method` and `params`, returning the raw `result` field.
    ///
    /// # Type Parameters
    /// - `P`: The type of the parameters, must implement `Serialize`.
    ///
    /// # Parameters
    /// - `method`: The RPC method name.
    /// - `params`: The parameters to pass to the RPC call.
    ///
    /// # Errors
    /// Returns `TransportError` if the HTTP request fails, the server returns an error object,
    /// or the response cannot be parsed or is missing the `result`.
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

    /// Send a JSON‑RPC request with given `method` and `params`, deserializing the `result` into `R`.
    ///
    /// # Type Parameters
    /// - `T`: The type of the parameters, must implement `Serialize`.
    /// - `R`: The expected return type, must implement `DeserializeOwned`.
    ///
    /// # Parameters
    /// - `method`: The RPC method name.
    /// - `params`: The parameters to pass to the RPC call.
    ///
    /// # Errors
    /// Returns `TransportError` if the HTTP request fails or deserialization fails.
    pub async fn call<T: Serialize, R: DeserializeOwned>(
        &self,
        method: &str,
        params: &[T],
    ) -> Result<R, TransportError> {
        let response = self.send_request(method, params).await?;
        Ok(serde_json::from_value(response)?)
    }
}
