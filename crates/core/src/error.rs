// crates/core/src/error.rs

use std::path::PathBuf;
use thiserror::Error;

/// Top-level error type for bitcoin-rpc-codegen
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("fetch error: {0}")]
    Fetch(#[from] FetchError),

    #[error("discovery error: {0}")]
    Discover(#[from] DiscoverError),

    #[error("schema error: {0}")]
    Schema(#[from] SchemaError),

    #[error("generation error: {0}")]
    Generate(#[from] GenerateError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("other error: {0}")]
    Other(String),
}

impl From<anyhow::Error> for CoreError {
    fn from(err: anyhow::Error) -> Self {
        CoreError::Other(err.to_string())
    }
}

/// Error during binary fetching (download/extract)
#[derive(Error, Debug)]
pub enum FetchError {
    #[error("could not download binary: {0}")]
    DownloadFailed(String),

    #[error("failed to extract archive: {0}")]
    ExtractFailed(String),

    #[error("bitcoind binary not found at: {0}")]
    NotFound(PathBuf),
}

/// Error during probing for available RPC methods
#[derive(Error, Debug)]
pub enum DiscoverError {
    #[error("failed to run bitcoin-cli: {0}")]
    CliFailed(String),
}

/// Error during schema parsing or generation
#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("invalid schema format: {0}")]
    InvalidFormat(String),
}

/// Error during Rust code generation
#[derive(Error, Debug)]
pub enum GenerateError {
    #[error("failed to generate Rust code: {0}")]
    CodegenFailed(String),
}
