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
    writeln!(code, "use serde_json::Value;").unwrap();
    writeln!(code, "use thiserror::Error;").unwrap();
    writeln!(code, "use reqwest;").unwrap();
    writeln!(code, "use serde;\n").unwrap();
}

fn emit_wallet_methods(code: &mut String) {
    writeln!(code, "/// List of Bitcoin Core wallet RPC methods").unwrap();
    writeln!(code, "pub mod wallet_methods {{").unwrap();
    writeln!(code, "    pub const WALLET_METHODS: &[&str] = &[").unwrap();
    for method in crate::wallet_methods::WALLET_METHODS {
        writeln!(code, "        \"{}\",", method).unwrap();
    }
    writeln!(code, "    ];").unwrap();
    writeln!(code, "}}\n").unwrap();
}

fn emit_error_enum(code: &mut String) {
    writeln!(
        code,
        "#[derive(Debug, Error, serde::Serialize, serde::Deserialize)]"
    )
    .unwrap();
    writeln!(code, "pub enum TransportError {{").unwrap();
    writeln!(code, "    #[error(\"HTTP error: {{0}}\")] Http(String),").unwrap();
    writeln!(code, "    #[error(\"JSON error: {{0}}\")] Json(String),").unwrap();
    writeln!(code, "    #[error(\"RPC error: {{0}}\")] Rpc(String),").unwrap();
    writeln!(code, "}}\n").unwrap();
}

fn emit_error_impls(code: &mut String) {
    for (from, variant) in &[
        ("reqwest::Error", "Http"),
        ("serde_json::Error", "Json"),
        ("anyhow::Error", "Rpc"),
    ] {
        writeln!(code, "impl From<{}> for TransportError {{", from).unwrap();
        writeln!(code, "    fn from(err: {}) -> Self {{", from).unwrap();
        writeln!(code, "        TransportError::{}(err.to_string())", variant).unwrap();
        writeln!(code, "    }}").unwrap();
        writeln!(code, "}}\n").unwrap();
    }
}

fn emit_transport_trait(code: &mut String) {
    writeln!(code, "pub trait Transport: Send + Sync {{").unwrap();
    writeln!(code, "    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>>;").unwrap();
    writeln!(code, "}}\n").unwrap();
}

fn emit_transport_ext_trait(code: &mut String) {
    writeln!(code, "pub trait TransportExt {{").unwrap();
    writeln!(code, "    fn call<'a, T: serde::de::DeserializeOwned>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, TransportError>> + Send + 'a>>;").unwrap();
    writeln!(code, "}}\n").unwrap();
}

fn emit_transport_ext_impl(code: &mut String) {
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
}

fn emit_default_transport_struct(code: &mut String) {
    writeln!(code, "#[derive(Clone, Debug)]").unwrap();
    writeln!(code, "pub struct DefaultTransport {{").unwrap();
    writeln!(code, "    client: reqwest::Client,").unwrap();
    writeln!(code, "    url: String,").unwrap();
    writeln!(code, "    auth: Option<(String, String)>,").unwrap();
    writeln!(code, "    wallet_name: Option<String>,").unwrap();
    writeln!(code, "}}\n").unwrap();
}

fn emit_default_transport_impl(code: &mut String) {
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
    writeln!(code, "            wallet_name: None,").unwrap();
    writeln!(code, "        }}").unwrap();
    writeln!(code, "    }}\n").unwrap();
    writeln!(
        code,
        "    pub fn with_wallet(mut self, wallet_name: impl Into<String>) -> Self {{"
    )
    .unwrap();
    writeln!(code, "        self.wallet_name = Some(wallet_name.into());").unwrap();
    writeln!(code, "        self").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}\n").unwrap();
}

fn emit_transport_impl(code: &mut String) {
    writeln!(code, "impl Transport for DefaultTransport {{").unwrap();
    writeln!(code, "    fn send_request<'a>(&'a self, method: &'a str, params: &'a [Value]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Value, TransportError>> + Send + 'a>> {{").unwrap();
    writeln!(code, "        let client = self.client.clone();").unwrap();
    writeln!(code, "        let url = self.url.clone();").unwrap();
    writeln!(code, "        let auth = self.auth.clone();").unwrap();
    writeln!(code, "        let wallet_name = self.wallet_name.clone();").unwrap();
    writeln!(code, "        Box::pin(async move {{").unwrap();
    writeln!(code, "            let request = serde_json::json!({{").unwrap();
    writeln!(code, "                \"jsonrpc\": \"2.0\", \"id\": \"1\", \"method\": method, \"params\": params").unwrap();
    writeln!(code, "            }});").unwrap();
    writeln!(
        code,
        "            eprintln!(\"[debug] Sending request to {{}}: {{}}\", url, request);"
    )
    .unwrap();
    writeln!(
        code,
        "            let url = if let Some(wallet) = &wallet_name {{"
    )
    .unwrap();
    writeln!(
        code,
        "                if wallet_methods::WALLET_METHODS.contains(&method) {{"
    )
    .unwrap();
    writeln!(
        code,
        "                    format!(\"{{}}/wallet/{{}}\", url.trim_end_matches('/'), wallet)"
    )
    .unwrap();
    writeln!(code, "                }} else {{ url }}").unwrap();
    writeln!(code, "            }} else {{ url }};").unwrap();
    writeln!(
        code,
        "            let mut req = client.post(&url).json(&request);"
    )
    .unwrap();
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
    writeln!(code, "            let response = match req.send().await {{").unwrap();
    writeln!(code, "                Ok(resp) => {{ eprintln!(\"[debug] Response status: {{}}\", resp.status()); resp }},").unwrap();
    writeln!(
        code,
        "                Err(e) => return Err(TransportError::Http(e.to_string())),"
    )
    .unwrap();
    writeln!(code, "            }};").unwrap();
    writeln!(code, "            let text = response.text().await.map_err(|e| TransportError::Http(e.to_string()))?;").unwrap();
    writeln!(
        code,
        "            eprintln!(\"[debug] Response body: {{}}\", text);"
    )
    .unwrap();
    writeln!(code, "            let json: Value = serde_json::from_str(&text).map_err(|e| TransportError::Json(e.to_string()))?;").unwrap();
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
    writeln!(code, "            Ok(json.get(\"result\").cloned().ok_or_else(|| TransportError::Rpc(\"No result field\".to_string()))?)").unwrap();
    writeln!(code, "        }})").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}").unwrap();
}
