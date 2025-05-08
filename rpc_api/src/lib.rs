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
}

/// Supported Bitcoin‐Core RPC versions
pub const SUPPORTED_VERSIONS: &[&str] = &["v24", "v25", "v26" /* etc. */];

/// Parsed version enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    V24,
    V25,
    V26,
    // …
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
    // …any others you need
}

/// Convert a string tag into our `Version` enum
pub fn parse_version(s: &str) -> Result<Version, Error> {
    match s {
        "v24" => Ok(Version::V24),
        "v25" => Ok(Version::V25),
        "v26" => Ok(Version::V26),
        other => Err(Error::UnsupportedVersion(other.to_string())),
    }
}
