use crate::CodeGenerator;
use types::ApiMethod;
use std::fmt::Write as _;

/// Code generator that creates the core transport layer for Bitcoin RPC communication
pub struct TransportCoreGenerator;

impl CodeGenerator for TransportCoreGenerator {
    fn generate(&self, _methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut code = String::new();

        emit_imports(&mut code);
        emit_error_enum(&mut code);
        emit_error_impls(&mut code);
        emit_transport_trait(&mut code);
        emit_transport_ext_trait(&mut code);
        emit_transport_ext_impl(&mut code);
        emit_default_transport_struct(&mut code);
        emit_default_transport_impl(&mut code);
        emit_transport_impl(&mut code);

        vec![("core.rs".to_string(), code)]
    }
}

fn emit_imports(code: &mut String) {
    writeln!(
        code,
        "use serde_json::Value;\n\
     use thiserror::Error;\n\
     use reqwest;\n\
     use serde;\n"
    )
    .unwrap();
}

fn emit_error_enum(code: &mut String) {
    writeln!(
        code,
        "#[derive(Debug, Error, serde::Serialize, serde::Deserialize)]\n\
         pub enum TransportError {{\n\
             #[error(\"HTTP error: {{0}}\")] Http(String),\n\
             #[error(\"JSON error: {{0}}\")] Json(String),\n\
             #[error(\"RPC error: {{0}}\")] Rpc(String),\n\
         }}\n"
    )
    .unwrap();
}

fn emit_error_impls(code: &mut String) {
    for (from, variant) in &[
        ("reqwest::Error", "Http"),
        ("serde_json::Error", "Json"),
        ("anyhow::Error", "Rpc"),
    ] {
        writeln!(
            code,
            "impl From<{from}> for TransportError {{\n\
                 fn from(err: {from}) -> Self {{\n\
                     TransportError::{variant}(err.to_string())\n\
                 }}\n\
             }}\n"
        )
        .unwrap();
    }
}

fn emit_transport_trait(code: &mut String) {
    writeln!(
        code,
        "pub trait TransportTrait: Send + Sync {{
    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>>;
    
    /// Send a **batch** of raw JSON-RPC objects in one HTTP call.
    ///
    /// The `bodies` slice is already serializable JSON-RPC-2.0 frames:
    ///   [ {{ \"jsonrpc\":\"2.0\", \"id\":0, \"method\":\"foo\", \"params\": [...] }}, … ]
    fn send_batch<'a>(
        &'a self,
        bodies: &'a [Value],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Value>, TransportError>> + Send + 'a>>;
    
    fn url(&self) -> &str;
}}"
    )
    .unwrap();
}

fn emit_transport_ext_trait(code: &mut String) {
    writeln!(
        code,
        "pub trait TransportExt {{\n\
             fn call<'a, T: serde::de::DeserializeOwned>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, TransportError>> + Send + 'a>>;\n\
         }}\n"
    )
    .unwrap();
}

fn emit_transport_ext_impl(code: &mut String) {
    writeln!(
        code,
        "impl<T: TransportTrait> TransportExt for T {{\n\
             fn call<'a, T2: serde::de::DeserializeOwned>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T2, TransportError>> + Send + 'a>> {{\n\
                 Box::pin(async move {{\n\
                     let result = self.send_request(method, params).await?;\n\
                     Ok(serde_json::from_value(result)?)\n\
                 }})\n\
             }}\n\
         }}\n"
    )
    .unwrap();
}

fn emit_default_transport_struct(code: &mut String) {
    writeln!(
        code,
        "#[derive(Clone, Debug)]\n\
         pub struct DefaultTransport {{\n\
             client: reqwest::Client,\n\
             url: String,\n\
             auth: Option<(String, String)>,\n\
             wallet_name: Option<String>,\n\
         }}\n"
    )
    .unwrap();
}

fn emit_default_transport_impl(code: &mut String) {
    writeln!(
        code,
        "impl DefaultTransport {{\n\
             pub fn new(url: impl Into<String>, auth: Option<(String, String)>) -> Self {{\n\
                 Self {{\n\
                     client: reqwest::Client::new(),\n\
                     url: url.into(),\n\
                     auth,\n\
                     wallet_name: None,\n\
                 }}\n\
             }}\n\
             \n\
             pub fn with_wallet(mut self, wallet_name: impl Into<String>) -> Self {{\n\
                 self.wallet_name = Some(wallet_name.into());\n\
                 self\n\
             }}\n\
         }}\n"
    )
    .unwrap();
}

fn emit_transport_impl(code: &mut String) {
    writeln!(
        code,
        "impl TransportTrait for DefaultTransport {{
    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>> {{
        let client = self.client.clone();
        let url = self.url.clone();
        let auth = self.auth.clone();
        let wallet_name = self.wallet_name.clone();
        Box::pin(async move {{
            let request = serde_json::json!({{
                \"jsonrpc\": \"2.0\", \"id\": \"1\", \"method\": method, \"params\": params
            }});
            eprintln!(\"[debug] Sending request to {{}}\", url);

            // If a wallet is configured, prefer wallet endpoint; fallback to base URL on -32601 (method not found)
            if let Some(wallet) = &wallet_name {{
                let wallet_url = format!(\"{{}}/wallet/{{}}\", url.trim_end_matches('/'), wallet);

                // Try wallet endpoint first
                let mut req = client.post(&wallet_url).json(&request);
                if let Some((username, password)) = &auth {{
                    req = req.basic_auth(username, Some(password));
                }}
                let response = match req.send().await {{
                    Ok(resp) => {{ eprintln!(\"[debug] Response status: {{}}\", resp.status()); resp }}
                    Err(e) => return Err(TransportError::Http(e.to_string())),
                }};

                let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;
                eprintln!(\"[debug] Response body: {{}}\", text);
                let json: Value = serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;

                if let Some(error) = json.get(\"error\") {{
                    // Fallback only for -32601 (Method not found)
                    if error.get(\"code\").and_then(|c| c.as_i64()) == Some(-32601) {{
                        let mut req = client.post(&url).json(&request);
                        if let Some((username, password)) = &auth {{
                            req = req.basic_auth(username, Some(password));
                        }}
                        let response = match req.send().await {{
                            Ok(resp) => {{ eprintln!(\"[debug] Base response status: {{}}\", resp.status()); resp }}
                            Err(e) => return Err(TransportError::Http(e.to_string())),
                        }};
                        let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;
                        eprintln!(\"[debug] Base response body: {{}}\", text);
                        let json: Value = serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;
                        if let Some(error) = json.get(\"error\") {{
                            return Err(TransportError::Rpc(error.to_string()));
                        }}
                        return Ok(json.get(\"result\").cloned().ok_or_else(|| TransportError::Rpc(\"No result field\".to_string()))?);
                    }} else {{
                        return Err(TransportError::Rpc(error.to_string()));
                    }}
                }}

                return Ok(json.get(\"result\").cloned().ok_or_else(|| TransportError::Rpc(\"No result field\".to_string()))?);
            }}

            // No wallet configured → base URL
            let mut req = client.post(&url).json(&request);
            if let Some((username, password)) = &auth {{
                req = req.basic_auth(username, Some(password));
            }}
            let response = match req.send().await {{
                Ok(resp) => {{ eprintln!(\"[debug] Response status: {{}}\", resp.status()); resp }},
                Err(e) => return Err(TransportError::Http(e.to_string())),
            }};
            let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;
            eprintln!(\"[debug] Response body: {{}}\", text);
            let json: Value = serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;
            if let Some(error) = json.get(\"error\") {{
                return Err(TransportError::Rpc(error.to_string()));
            }}
            Ok(json.get(\"result\").cloned().ok_or_else(|| TransportError::Rpc(\"No result field\".to_string()))?)
        }})
    }}
    
    fn send_batch<'a>(
        &'a self,
        bodies: &'a [Value],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Value>, TransportError>> + Send + 'a>> {{
        let client = self.client.clone();
        let url = self.url.clone();
        let auth = self.auth.clone();
        Box::pin(async move {{
            eprintln!(\"[debug] Sending batch request to {{}}: {{:?}}\", url, bodies);
            let mut req = client.post(&url).json(bodies);
            if let Some((username, password)) = &auth {{
                req = req.basic_auth(username, Some(password));
            }}
            let response = match req.send().await {{
                Ok(resp) => {{ eprintln!(\"[debug] Batch response status: {{}}\", resp.status()); resp }},
                Err(e) => return Err(TransportError::Http(e.to_string())),
            }};
            let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;
            eprintln!(\"[debug] Batch response body: {{}}\", text);
            let v: Vec<Value> = serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;
            Ok(v)
        }})
    }}
    
    fn url(&self) -> &str {{
        &self.url
    }}
}}"
    )
    .unwrap();
}
