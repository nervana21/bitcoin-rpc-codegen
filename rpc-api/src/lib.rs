//! rpc_api/src/lib.rs
//! Defines the canonical types, error enum, and supported-version logic.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An RPC method's full schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMethod {
    pub name: String,
    pub description: String,
    pub arguments: Vec<ApiArgument>,
    pub results: Vec<ApiResult>,
}

/// One argument to an RPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiArgument {
    pub names: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub optional: bool,
    pub description: String,
}

/// One result field from an RPC method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult {
    pub key_name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub inner: Vec<ApiResult>,
    pub optional: bool,
}

/// Supported Bitcoin‐Core RPC versions
pub const SUPPORTED_VERSIONS: &[&str] = &["v27", "v28", "v29" /* etc. */];

/// Parsed version enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    V27,
    V28,
    V29,
    // TODO: add support for prior versions
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("unsupported version: {0}")]
    UnsupportedVersion(String),

    #[error("failed to parse version: {0}")]
    VersionParseError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    // TODO: add more cases as needed
}

/// Convert a string tag into our `Version` enum
pub fn parse_version(s: &str) -> Result<Version, Error> {
    match s {
        "v27" => Ok(Version::V27),
        "v28" => Ok(Version::V28),
        "v29" => Ok(Version::V29),
        other => Err(Error::UnsupportedVersion(other.to_string())),
    }
}

pub fn parse_api_json(json: &str) -> Result<Vec<ApiMethod>, serde_json::Error> {
    let api: serde_json::Value = serde_json::from_str(json)?;
    let commands = api["commands"].as_object().unwrap();

    let mut parsed_methods = Vec::new();

    for (name, command_array) in commands {
        let command = &command_array.as_array().unwrap()[0];
        let command_obj = command.as_object().unwrap();

        let arguments = command_obj["arguments"]
            .as_array()
            .map(|params| {
                params
                    .iter()
                    .map(|param| ApiArgument {
                        names: param["names"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|n| n.as_str().unwrap().to_string())
                            .collect(),
                        type_: param["type"].as_str().unwrap().to_string(),
                        optional: param["optional"].as_bool().unwrap_or(false),
                        description: param["description"].as_str().unwrap_or("").to_string(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let results = command_obj["results"]
            .as_array()
            .map(|results| results.iter().map(parse_result).collect())
            .unwrap_or_default();

        parsed_methods.push(ApiMethod {
            name: name.clone(),
            description: command_obj["description"]
                .as_str()
                .unwrap_or("")
                .to_string(),
            arguments,
            results,
        });
    }

    Ok(parsed_methods)
}

fn parse_result(value: &serde_json::Value) -> ApiResult {
    let obj = value.as_object().unwrap();

    ApiResult {
        type_: obj["type"].as_str().unwrap_or("").to_string(),
        description: obj["description"].as_str().unwrap_or("").to_string(),
        key_name: obj["key_name"].as_str().unwrap_or("").to_string(),
        inner: obj["inner"]
            .as_array()
            .map(|props| props.iter().map(parse_result).collect())
            .unwrap_or_default(),
        optional: obj["optional"].as_bool().unwrap_or(false),
    }
}
