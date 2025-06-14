//! This crate contains the code generators for the Bitcoin RPC API.
//!
//! The generators are responsible for generating the code for the Bitcoin RPC API.

/// Sub-crate generates: **`doc_comment`**
///
/// Produces Rust-doc comments and Markdown "Example:" blocks.
/// Transforms each `ApiMethod` into triple-slash doc comments injected into generated files.
pub mod doc_comment;

/// Sub-crate generates: **`response_type`**
///
/// Defines strongly-typed response structs for RPC methods:
///
/// - Parses each method's "Result:" section (or `api.json`).
/// - Builds a `<method>_response.rs` file with appropriate `serde` attributes
///   (`Option<T>`, `skip_serializing_if`).
/// - Exported as `TypesCodeGenerator`, used by the transport generator.
pub mod response_type;
pub use response_type::ResponseTypeCodeGenerator;

/// Sub-crate generates: **`rpc_client`**
///
/// Generates the transport-layer client code: async RPC method wrappers
/// that handle parameter serialization and response deserialization.
pub mod rpc_client;

/// Sub-crate generates: **`rpc_method_macro`**
///
/// Generates `macro_rules!` definitions for version-scoped client wrappers.
/// Downstream crates can write:
///
/// ```rust,ignore
/// impl_client_latest__getblockchaininfo!();
/// ```
///
/// to obtain a fully-typed `fn getblockchaininfo(&self) -> ...` method on their `Client`.
pub mod rpc_method_macro;

/// Sub-crate generates: **`client_trait`**
///
/// Generates the client trait that defines the interface for Bitcoin RPC clients.
/// This trait is implemented for any type that implements Transport.
pub mod client_trait;
pub use client_trait::ClientTraitGenerator;
