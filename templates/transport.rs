//! Minimal transport layer for Bitcoin RPC communication

use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("RPC error: {0}")]
    Rpc(String),
}

/// Core transport trait for sending JSON-RPC requests
pub trait Transport: Send + Sync {
    /// Send a JSON-RPC request and return the result
    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>>;
}

/// Extension trait for Transport that provides convenience methods
pub trait TransportExt {
    /// Call a JSON-RPC method with parameters
    fn call<'a, T: serde::de::DeserializeOwned>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, TransportError>> + Send + 'a>>;
}

impl<T: Transport> TransportExt for T {
    fn call<'a, T2: serde::de::DeserializeOwned>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T2, TransportError>> + Send + 'a>> {
        Box::pin(async move {
            let result = self.send_request(method, params).await?;
            Ok(serde_json::from_value(result)?)
        })
    }
}

#[derive(Clone)]
pub struct DefaultTransport {
    pub url: String,
    pub user: String,
    pub pass: String,
}

impl Transport for DefaultTransport {
    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>> {
        let url = self.url.clone();
        let user = self.user.clone();
        let pass = self.pass.clone();
        
        Box::pin(async move {
            let client = reqwest::Client::new();
            let body = serde_json::json!({
                "jsonrpc": "2.0",
                "id": "1",
                "method": method,
                "params": params,
            });

            let res = client
                .post(&url)
                .basic_auth(&user, Some(&pass))
                .json(&body)
                .send()
                .await?;

            let json: Value = res.json().await?;
            if let Some(error) = json.get("error") {
                return Err(TransportError::Rpc(error.to_string()));
            }

            Ok(json["result"].clone())
        })
    }
}
