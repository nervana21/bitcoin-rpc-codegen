// crates/core/src/lib.rs
//! bitcoin-rpc-codegen core library
//!
//! Provides pure logic (no side-effects) for downloading, discovering,
//! parsing, and generating Rust clients/types for Bitcoin Core RPCs.

// Modules
pub mod discover;
pub mod error;
pub mod fetch;
pub mod generator;
pub mod schema;

// Re-exported public API
pub use discover::discover_methods;
pub use error::{CoreError, DiscoverError, FetchError, GenerateError, SchemaError};
pub use fetch::fetch_bitcoind;
pub use generator::generate_version_code;
pub use schema::{extract_api_docs, parse_api_json, ApiArgument, ApiMethod, ApiResult};
