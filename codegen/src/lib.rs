//! Code generation utilities for Bitcoin Core JSON-RPC.
//!
//! This crate turns `ApiMethod` descriptors into ready-to`cargo check` Rust modules.
//! It focuses solely on code generation: parsing API metadata, scaffolding module hierarchies,
//! generating transport-layer clients, strongly-typed response structs, and test-node helpers.
//!
//! Other responsibilities—such as runtime testing, node spawning, or API discovery logic—reside in companion crates.
#![warn(missing_docs)]

pub mod generators;

use crate::generators::doc_comment;
use crate::generators::response_type;
use anyhow::Result;
use rpc_api::ApiMethod;
use std::{fs, path::Path, process::Command};

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

#[allow(unused)]
fn format_with_rustfmt(path: &Path) {
    if let Ok(status) = Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(path)
        .status()
    {
        if !status.success() {
            eprintln!("[warn] rustfmt failed on {path:?}");
        }
    } else {
        eprintln!("[warn] rustfmt not found or failed to run for {path:?}");
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
        fs::write(&path, src.as_bytes())?;
        format_with_rustfmt(&path);
    }
    Ok(())
}

/// Emits async JSON-RPC transport wrappers for Bitcoin Core RPC methods.
///
/// `TransportCodeGenerator` implements the `CodeGenerator` trait to produce, for each
/// `ApiMethod`, a self-contained Rust source file containing:
/// 1. An `async fn` that accepts a `&dyn TransportTrait` and JSON-serializable parameters.
/// 2. Logic to serialize those parameters into a `Vec<serde_json::Value>`.
/// 3. A call to `transport.send_request(method_name, &params).await`.
/// 4. Deserialization of the raw response into a typed `Response` struct (or raw `Value`).
pub struct TransportCodeGenerator;

impl CodeGenerator for TransportCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        use utils::capitalize;

        methods
            .iter()
            .map(|m| {
                /* ---------- fn signature ---------- */
                let fn_args = std::iter::once("transport: &dyn TransportTrait".into())
                    .chain(m.arguments.iter().map(|a| {
                        let name = if a.names[0] == "type" {
                            format!("r#{}", a.names[0])
                        } else {
                            a.names[0].clone()
                        };
                        format!("{name}: serde_json::Value")
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
                            format!("json!({name})")
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("vec![{elems}]")
                };

                /* ---------- docs + types ---------- */
                let docs_md = doc_comment::generate_example_docs(m, "latest")
                    .trim_end()
                    .to_string();
                let response_struct = response_type::build_return_type(m)
                    .unwrap_or_default()
                    .unwrap_or_default();
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
use transport::{{TransportTrait, TransportError}};
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
                    fn_name = &m.name,
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

// TODO(multiprocess): Introduce an `RpcComponent` abstraction to formally distinguish between
// independently-addressable RPC components like `node`, `wallet`, `index`, and `gui`.
//
// This will support:
// - routing method calls to different endpoints (e.g., node.sock vs wallet.sock)
// - preventing runtime errors by associating methods with their component
// - future `CombinedClient` that multiplexes requests across components
//
// This abstraction will become essential as Bitcoin Core moves toward
// separate processes with their own RPC servers.
//
// Start by creating a `components.rs` module defining `RpcComponent` and a registry of methods.
