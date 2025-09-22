//! This crate contains the code generators for the Bitcoin RPC API.
//!
//! The generators are responsible for generating the code for the Bitcoin RPC API.

/// Sub-crate generates: **`doc_comment`**
///
/// Produces Rust-doc comments and Markdown "Example:" blocks.
/// Transforms each `BtcMethod` into triple-slash doc comments injected into generated files.
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

/// Sub-crate generates: **`client_trait`**
///
/// Generates the client trait that defines the interface for Bitcoin RPC clients.
/// This trait is implemented for any type that implements Transport.
pub mod client_trait;
pub use client_trait::ClientTraitGenerator;

// Sub-crate generates: **`batch_builder`**
///
/// Generates a fluent `BatchBuilder` with one method-per-RPC that queues
/// calls and an `.execute().await` entrypoint returning a strongly-typed tuple.
pub mod batch_builder;
pub use batch_builder::BatchBuilderGenerator;

pub mod test_node;
