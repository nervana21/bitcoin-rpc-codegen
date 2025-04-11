use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Failed to parse RPC method: {0}")]
    ParseError(String),
}

/// Represents a Bitcoin Core RPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcMethod {
    pub name: String,
    pub description: String,
    pub arguments: Vec<RpcArgument>,
    pub results: Vec<RpcResult>,
}

/// Represents an argument to an RPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcArgument {
    pub description: String,
    pub names: Vec<String>,
    pub optional: bool,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub inner: Vec<RpcResult>,
}

/// Represents a result from an RPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResult {
    #[serde(rename = "type")]
    pub type_: String,
    pub optional: bool,
    pub description: String,
    pub inner: Vec<RpcResult>,
    #[serde(default)]
    pub key_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiRoot {
    commands: HashMap<String, Vec<RpcMethod>>,
}

/// Parses the Bitcoin Core RPC API specification
pub fn parse_api_spec(json: &str) -> Result<Vec<RpcMethod>, ParserError> {
    let root: ApiRoot =
        serde_json::from_str(json).map_err(|e| ParserError::ParseError(e.to_string()))?;

    let mut methods = Vec::new();

    for (_, method_variants) in root.commands {
        // For now, just take the first variant of each method
        if let Some(method) = method_variants.into_iter().next() {
            methods.push(method);
        }
    }

    Ok(methods)
}

// Helper function to convert JSON types to Rust types
pub fn json_type_to_rust_type(json_type: &str) -> &str {
    match json_type {
        "string" => "String",
        "number" => "f64",
        "integer" => "i64",
        "boolean" => "bool",
        "array" => "Vec<T>",
        "object" => "HashMap<String, Value>",
        _ => "String", // Default to String for unknown types
    }
}

// Helper function to sanitize method names for Rust
pub fn sanitize_method_name(name: &str) -> String { name.replace("-", "_").to_lowercase() }
