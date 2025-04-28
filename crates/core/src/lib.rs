// crates/core/src/lib.rs
//! bitcoin-rpc-codegen core library
//!
//! Provides pure logic (no side-effects) for downloading, discovering,
//! parsing, and generating Rust clients/types for Bitcoin Core RPCs.

// Modules
pub mod discover;
pub mod download;
pub mod generator;
pub mod schema;

// Re-exported public API
pub use discover::discover_methods;
pub use download::fetch_bitcoind;
pub use generator::generate_version_code;
pub use schema::{parse_api_json, ApiArgument, ApiMethod, ApiResult};
