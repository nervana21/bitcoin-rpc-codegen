use crate::CodeGenerator;
use rpc_api::ApiMethod;
use std::fmt::Write as _;

/// Code generator that creates the core transport layer for Bitcoin RPC communication
pub struct TransportCoreGenerator;

impl CodeGenerator for TransportCoreGenerator {
    fn generate(&self, _methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut code = String::new();

        // Add the transport module code
        writeln!(code, "use serde_json::Value;").unwrap();
        writeln!(code, "use thiserror::Error;").unwrap();
        writeln!(code, "use reqwest;").unwrap();
        writeln!(code, "use serde;").unwrap();
        writeln!(code, "\n").unwrap();

        // TransportError enum
        writeln!(
            code,
            "#[derive(Debug, Error, serde::Serialize, serde::Deserialize)]"
        )
        .unwrap();
        writeln!(code, "pub enum TransportError {{").unwrap();
        writeln!(code, "    #[error(\"HTTP error: {{0}}\")]").unwrap();
        writeln!(code, "    Http(String),").unwrap();
        writeln!(code, "    #[error(\"JSON error: {{0}}\")]").unwrap();
        writeln!(code, "    Json(String),").unwrap();
        writeln!(code, "    #[error(\"RPC error: {{0}}\")]").unwrap();
        writeln!(code, "    Rpc(String),").unwrap();
        writeln!(code, "}}\n").unwrap();

        // Implement From for error types
        writeln!(code, "impl From<reqwest::Error> for TransportError {{").unwrap();
        writeln!(code, "    fn from(err: reqwest::Error) -> Self {{").unwrap();
        writeln!(code, "        TransportError::Http(err.to_string())").unwrap();
        writeln!(code, "    }}").unwrap();
        writeln!(code, "}}\n").unwrap();

        writeln!(code, "impl From<serde_json::Error> for TransportError {{").unwrap();
        writeln!(code, "    fn from(err: serde_json::Error) -> Self {{").unwrap();
        writeln!(code, "        TransportError::Json(err.to_string())").unwrap();
        writeln!(code, "    }}").unwrap();
        writeln!(code, "}}\n").unwrap();

        writeln!(code, "impl From<anyhow::Error> for TransportError {{").unwrap();
        writeln!(code, "    fn from(err: anyhow::Error) -> Self {{").unwrap();
        writeln!(code, "        TransportError::Rpc(err.to_string())").unwrap();
        writeln!(code, "    }}").unwrap();
        writeln!(code, "}}\n").unwrap();

        // Core transport trait for sending JSON-RPC requests
        writeln!(
            code,
            "/// Core transport trait for sending JSON-RPC requests"
        )
        .unwrap();
        writeln!(code, "pub trait Transport: Send + Sync {{").unwrap();
        writeln!(
            code,
            "    /// Send a JSON-RPC request and return the result"
        )
        .unwrap();
        writeln!(code, "    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>>;").unwrap();
        writeln!(code, "}}\n").unwrap();

        // Extension trait for Transport that provides convenience methods
        writeln!(
            code,
            "/// Extension trait for Transport that provides convenience methods"
        )
        .unwrap();
        writeln!(code, "pub trait TransportExt {{").unwrap();
        writeln!(code, "    /// Call a JSON-RPC method with parameters").unwrap();
        writeln!(code, "    fn call<'a, T: serde::de::DeserializeOwned>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, TransportError>> + Send + 'a>>;").unwrap();
        writeln!(code, "}}\n").unwrap();

        // Implement the extension trait for all types that implement Transport
        writeln!(code, "impl<T: Transport> TransportExt for T {{").unwrap();
        writeln!(code, "    fn call<'a, T2: serde::de::DeserializeOwned>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T2, TransportError>> + Send + 'a>> {{").unwrap();
        writeln!(code, "        Box::pin(async move {{").unwrap();
        writeln!(
            code,
            "            let result = self.send_request(method, params).await?;"
        )
        .unwrap();
        writeln!(code, "            Ok(serde_json::from_value(result)?)").unwrap();
        writeln!(code, "        }})").unwrap();
        writeln!(code, "    }}").unwrap();
        writeln!(code, "}}\n").unwrap();

        // DefaultTransport implementation
        writeln!(code, "#[derive(Clone)]").unwrap();
        writeln!(code, "pub struct DefaultTransport {{").unwrap();
        writeln!(code, "    client: reqwest::Client,").unwrap();
        writeln!(code, "    url: String,").unwrap();
        writeln!(code, "    auth: Option<(String, String)>,").unwrap();
        writeln!(code, "}}\n").unwrap();

        writeln!(code, "impl DefaultTransport {{").unwrap();
        writeln!(
            code,
            "    pub fn new(url: impl Into<String>, auth: Option<(String, String)>) -> Self {{"
        )
        .unwrap();
        writeln!(code, "        Self {{").unwrap();
        writeln!(code, "            client: reqwest::Client::new(),").unwrap();
        writeln!(code, "            url: url.into(),").unwrap();
        writeln!(code, "            auth,").unwrap();
        writeln!(code, "        }}").unwrap();
        writeln!(code, "    }}").unwrap();
        writeln!(code, "}}\n").unwrap();

        writeln!(code, "impl Transport for DefaultTransport {{").unwrap();
        writeln!(code, "    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>> {{").unwrap();
        writeln!(code, "        let client = self.client.clone();").unwrap();
        writeln!(code, "        let url = self.url.clone();").unwrap();
        writeln!(code, "        let auth = self.auth.clone();").unwrap();
        writeln!(code, "        Box::pin(async move {{").unwrap();
        writeln!(code, "            let request = serde_json::json!({{").unwrap();
        writeln!(code, "                \"jsonrpc\": \"2.0\",").unwrap();
        writeln!(code, "                \"id\": \"1\",").unwrap();
        writeln!(code, "                \"method\": method,").unwrap();
        writeln!(code, "                \"params\": params,").unwrap();
        writeln!(code, "            }});").unwrap();
        writeln!(code, "\n").unwrap();
        writeln!(code, "            let mut req = client.post(&url)").unwrap();
        writeln!(code, "                .json(&request);").unwrap();
        writeln!(code, "\n").unwrap();
        writeln!(
            code,
            "            if let Some((username, password)) = &auth {{"
        )
        .unwrap();
        writeln!(
            code,
            "                req = req.basic_auth(username, Some(password));"
        )
        .unwrap();
        writeln!(code, "            }}").unwrap();
        writeln!(code, "\n").unwrap();
        writeln!(code, "            let response = req.send().await?;").unwrap();
        writeln!(
            code,
            "            let json: Value = response.json().await?;"
        )
        .unwrap();
        writeln!(code, "\n").unwrap();
        writeln!(
            code,
            "            if let Some(error) = json.get(\"error\") {{"
        )
        .unwrap();
        writeln!(
            code,
            "                return Err(TransportError::Rpc(error.to_string()));"
        )
        .unwrap();
        writeln!(code, "            }}").unwrap();
        writeln!(code, "\n").unwrap();
        writeln!(code, "            let result = json.get(\"result\")").unwrap();
        writeln!(
            code,
            "                .ok_or_else(|| TransportError::Rpc(\"No result field\".to_string()))?;"
        )
        .unwrap();
        writeln!(code, "\n").unwrap();
        writeln!(code, "            Ok(result.clone())").unwrap();
        writeln!(code, "        }})").unwrap();
        writeln!(code, "    }}").unwrap();
        writeln!(code, "}}").unwrap();

        vec![("core.rs".to_string(), code)]
    }
}
