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
pub mod version;

// Re-exported public API
pub use discover::{discover_methods, parse_help_output};
pub use error::{CoreError, DiscoverError, FetchError, GenerateError, SchemaError};
pub use fetch::fetch_bitcoind;
pub use generator::{
    capitalize, format_doc_comment, format_struct_field, generate_client_macro,
    generate_client_mod_rs, generate_mod_rs, generate_return_type, generate_top_level_client_mod,
    generate_top_level_types_mod, generate_type_conversion, generate_types_mod_rs,
    generate_version_code, get_field_type, map_type_to_rust, sanitize_field_name,
    sanitize_method_name, SUPPORTED_MAJORS,
};
pub use schema::{extract_api_docs, parse_api_json, ApiArgument, ApiMethod, ApiResult};
pub use version::{ParseVersionError, Version};
