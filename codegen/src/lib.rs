//! Code generation utilities for Bitcoin Core JSON-RPC.
//!
//! This crate turns `ApiMethod` descriptors into ready-to`cargo check` Rust modules.
//! It focuses solely on code generation: parsing API metadata, scaffolding module hierarchies,
//! generating transport-layer clients, strongly-typed response structs, and test-node helpers.
//!
//! Other responsibilities—such as runtime testing, node spawning, or API discovery logic—reside in companion crates.
#![warn(missing_docs)]

use anyhow::Result;
use generators::doc_comment;
use lazy_static::lazy_static;
use rpc_api::ApiMethod;
use std::{fs, path::Path};

pub mod generators;

/// Sub-crate: **`rpc_method_discovery`**
///
/// Discovers available RPC methods at runtime using `bitcoin-cli`.
/// Queries the node for `help` output and converts it into an `ApiMethod` list.
/// Useful for generating code against whichever node version is on your `PATH`.
pub mod rpc_method_discovery;

/// Sub-crate: **`namespace_scaffolder`**
///
/// Writes `mod.rs` scaffolding for generated modules.
/// Given schema versions (`v28`, `v29`, `latest`, etc.), it creates:
///
/// - `generated/client/{versions}`
/// - `generated/types/{versions}`
///
/// plus a top-level `mod.rs` that re-exports everything, so downstream crates can simply:
///
/// ```rust,ignore
/// use generated::client::*;
/// ```
pub mod namespace_scaffolder;

/// Sub-crate: **`test_node_generator`**
///
/// Generates ergonomic TestNode methods that delegate to the underlying RPC client,
/// simplifying common integration-test workflows.
pub mod test_node_generator;

/// Sub-crate: **`type_registry`**
///
/// Central registry for mapping Bitcoin RPC types to Rust types.
/// Provides `TypeRegistry` and `TypeMapping` for canonical type conversions.
pub mod type_registry;
pub use type_registry::{TypeMapping, TypeRegistry};

/// Sub-crate: **`transport_core_generator`**
///
/// Generates the core transport types: Transport trait, TransportError enum,
/// and DefaultTransport implementation.
pub mod transport_core_generator;
pub use transport_core_generator::TransportCoreGenerator;

/// Sub-crate: **`utils`**
///
/// Utility functions for code generation.
pub mod utils;

/// Defines the core interface for generating Rust source files from a collection of
/// Bitcoin Core RPC API methods. Implementors produce a set of `(filename, source)`
/// pairs and may optionally perform post-generation validation.
///
/// This trait is used by the `TransportCodeGenerator` to produce the transport-layer
/// client code for each `ApiMethod`.
pub trait CodeGenerator {
    /// Generate Rust source files for the provided API methods.
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)>;

    /// Optional validation step after generation (default is no-op).
    fn validate(&self, _methods: &[ApiMethod]) -> Result<()> {
        Ok(())
    }
}

/// Persist a list of generated source files to disk under the given output directory,
/// creating any necessary subdirectories and appending `.rs` if missing.
pub fn write_generated<P: AsRef<Path>>(
    out_dir: P,
    files: &[(String, String)],
) -> std::io::Result<()> {
    fs::create_dir_all(&out_dir)?;
    for (name, src) in files {
        let path = if name.ends_with(".rs") {
            out_dir.as_ref().join(name)
        } else {
            out_dir.as_ref().join(format!("{name}.rs"))
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, src.as_bytes())?;
    }
    Ok(())
}

/// Emits async JSON-RPC transport wrappers for Bitcoin Core RPC methods.
///
/// `TransportCodeGenerator` implements the `CodeGenerator` trait to produce, for each
/// `ApiMethod`, a self-contained Rust source file containing:
/// 1. An `async fn` that accepts a `&dyn Transport` and JSON-serializable parameters.
/// 2. Logic to serialize those parameters into a `Vec<serde_json::Value>`.
/// 3. A call to `transport.send_request(method_name, &params).await`.
/// 4. Deserialization of the raw response into a typed `Response` struct (or raw `Value`).
pub struct TransportCodeGenerator;

impl CodeGenerator for TransportCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        use generators::response_type::{capitalize, generate_return_type, sanitize_method_name};

        methods
            .iter()
            .map(|m| {
                /* ---------- fn signature ---------- */
                let fn_args = std::iter::once("transport: &dyn Transport".into())
                    .chain(m.arguments.iter().map(|a| {
                        let name = if a.names[0] == "type" {
                            format!("r#{}", a.names[0])
                        } else {
                            a.names[0].clone()
                        };
                        format!("{}: serde_json::Value", name)
                    }))
                    .collect::<Vec<_>>()
                    .join(", ");

                /* ---------- params vec ---------- */
                let params_vec = if m.arguments.is_empty() {
                    "Vec::<Value>::new()".into()
                } else {
                    let elems = m
                        .arguments
                        .iter()
                        .map(|a| {
                            let name = if a.names[0] == "type" {
                                format!("r#{}", a.names[0])
                            } else {
                                a.names[0].clone()
                            };
                            format!("json!({})", name)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("vec![{elems}]")
                };

                /* ---------- docs + types ---------- */
                let docs_md = doc_comment::generate_example_docs(m, "latest")
                    .trim_end()
                    .to_string();
                let response_struct = generate_return_type(m).unwrap_or_default();
                let ok_ty = if response_struct.is_empty() {
                    "Value".into()
                } else {
                    format!("{0}Response", capitalize(&m.name))
                };

                /* ---------- source file ---------- */
                let src = format!(
                    r#"{docs}

use serde::{{Deserialize, Serialize}};
use serde_json::{{Value, json}};
use transport::{{Transport, TransportError}};
{resp_struct}

/// Calls the `{rpc}` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
pub async fn {fn_name}({fn_args}) -> Result<{ok_ty}, TransportError> {{
    let params = {params_vec};
    let raw = transport.send_request("{rpc}", &params).await?;
    {handler}
}}
"#,
                    docs = docs_md,
                    resp_struct = response_struct,
                    rpc = m.name,
                    fn_name = sanitize_method_name(&m.name),
                    fn_args = fn_args,
                    ok_ty = ok_ty,
                    params_vec = params_vec,
                    handler = if response_struct.is_empty() {
                        "Ok(raw)".into()
                    } else {
                        format!("Ok(serde_json::from_value::<{ok_ty}>(raw)?)")
                    }
                );

                (m.name.clone(), src)
            })
            .collect()
    }
}

lazy_static! {
    /// Canonical Type Registry: the single source of truth for Bitcoin‑RPC ⇢ Rust mappings.
    ///
    /// - Normalized RPC primitives → Rust primitives
    /// - Named structs/enums discovered during parsing
    /// - Version-specific overrides (e.g., `Numeric` vs `Amount`)
    ///
    /// All code-generation phases consult this registry to ensure consistent type conversions.
    pub static ref TYPE_REGISTRY: TypeRegistry = TypeRegistry::new();
}
