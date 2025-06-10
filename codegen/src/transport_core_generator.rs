use crate::CodeGenerator;
use rpc_api::ApiMethod;
use std::fmt::Write as _;

/// Code generator that creates the core transport layer for Bitcoin RPC communication
pub struct TransportCoreGenerator;

impl CodeGenerator for TransportCoreGenerator {
    fn generate(&self, _methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut code = String::new();

        emit_imports(&mut code);
        emit_wallet_methods(&mut code);
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

fn emit_wallet_methods(code: &mut String) {
    let methods: String = crate::wallet_methods::WALLET_METHODS
        .iter()
        .map(|method| format!("        \"{}\",", method))
        .collect::<Vec<_>>()
        .join("\n");

    writeln!(
        code,
        "/// List of Bitcoin Core wallet RPC methods\n\
         pub mod wallet_methods {{\n\
             pub const WALLET_METHODS: &[&str] = &[\n\
         {}\n\
             ];\n\
         }}\n",
        methods
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
            "impl From<{}> for TransportError {{\n\
                 fn from(err: {}) -> Self {{\n\
                     TransportError::{}(err.to_string())\n\
                 }}\n\
             }}\n",
            from, from, variant
        )
        .unwrap();
    }
}

fn emit_transport_trait(code: &mut String) {
    writeln!(
        code,
        "pub trait Transport: Send + Sync {{\n\
             fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>>;\n\
         }}\n"
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
        "impl<T: Transport> TransportExt for T {{\n\
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
        "impl Transport for DefaultTransport {{\n\
             fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>> {{\n\
                 let client = self.client.clone();\n\
                 let url = self.url.clone();\n\
                 let auth = self.auth.clone();\n\
                 let wallet_name = self.wallet_name.clone();\n\
                 Box::pin(async move {{\n\
                     let request = serde_json::json!({{\n\
                         \"jsonrpc\": \"2.0\", \"id\": \"1\", \"method\": method, \"params\": params\n\
                     }});\n\
                     eprintln!(\"[debug] Sending request to {{}}: {{}}\", url, request);\n\
                     let url = if let Some(wallet) = &wallet_name {{\n\
                         if wallet_methods::WALLET_METHODS.contains(&method) {{\n\
                             format!(\"{{}}/wallet/{{}}\", url.trim_end_matches('/'), wallet)\n\
                         }} else {{ url }}\n\
                     }} else {{ url }};\n\
                     let mut req = client.post(&url).json(&request);\n\
                     if let Some((username, password)) = &auth {{\n\
                         req = req.basic_auth(username, Some(password));\n\
                     }}\n\
                     let response = match req.send().await {{\n\
                         Ok(resp) => {{ eprintln!(\"[debug] Response status: {{}}\", resp.status()); resp }},\n\
                         Err(e) => return Err(TransportError::Http(e.to_string())),\n\
                     }};\n\
                     let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;\n\
                     eprintln!(\"[debug] Response body: {{}}\", text);\n\
                     let json: Value = serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;\n\
                     if let Some(error) = json.get(\"error\") {{\n\
                         return Err(TransportError::Rpc(error.to_string()));\n\
                     }}\n\
                     Ok(json.get(\"result\").cloned().ok_or_else(|| TransportError::Rpc(\"No result field\".to_string()))?)\n\
                 }})\n\
             }}\n\
         }}\n"
    )
    .unwrap();
}
