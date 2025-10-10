//! Code generation utilities for Bitcoin Core JSON-RPC.
//!
//! This crate turns `BtcMethod` descriptors into ready-to`cargo check` Rust modules.
//! It focuses solely on code generation: parsing API metadata, scaffolding module hierarchies,
//! generating transport-layer clients, strongly-typed response structs, and test-node helpers.
//!
//! Other responsibilities—such as runtime testing, node spawning, or API discovery logic—reside in companion crates.
#![warn(missing_docs)]

pub mod generators;
pub mod versioning;

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use bitcoin_rpc_types::BtcMethod;
use serde_json::Value;

use crate::generators::{doc_comment, response_type};
use crate::versioning::Version;

/// Load API methods from a JSON file using the new schema system
pub fn load_api_methods_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<BtcMethod>> {
    let raw = std::fs::read_to_string(&path)?;
    let v: Value = serde_json::from_str(&raw)?;

    let methods_value =
        v.get("methods").ok_or_else(|| anyhow::anyhow!("Missing 'methods' field in JSON"))?;

    let methods_map: std::collections::HashMap<String, BtcMethod> =
        serde_json::from_value(methods_value.clone())?;

    // Sort methods by name to ensure consistent ordering across runs
    let mut methods: Vec<BtcMethod> = methods_map.into_values().collect();
    methods.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(methods)
}

/// Sub-crate: **`namespace_scaffolder`**
///
/// Writes `mod.rs` scaffolding for generated modules.
/// Given schema versions (`v28`, `v29`, `latest`, etc.), it creates:
///
/// - `generated/client/{versions}`
/// - `generated/responses/{versions}`
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
/// client code for each `BtcMethod`.
pub trait CodeGenerator {
    /// Generate Rust source files for the provided API methods.
    fn generate(&self, methods: &[BtcMethod]) -> Vec<(String, String)>;

    /// Optional validation step after generation (default is no-op).
    fn validate(&self, _methods: &[BtcMethod]) -> Result<()> { Ok(()) }
}

#[allow(unused)]
fn format_with_rustfmt(path: &Path) {
    if let Ok(status) = Command::new("rustfmt").arg("--edition=2021").arg(path).status() {
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
/// `BtcMethod`, a self-contained Rust source file containing:
/// 1. An `async fn` that accepts a `&dyn TransportTrait` and JSON-serializable parameters.
/// 2. Logic to serialize those parameters into a `Vec<serde_json::Value>`.
/// 3. A call to `transport.send_request(method_name, &params).await`.
/// 4. Deserialization of the raw response into a typed `Response` struct (or raw `Value`).
pub struct TransportCodeGenerator {
    version: Version,
}

impl TransportCodeGenerator {
    /// Create a new TransportCodeGenerator with the specified Bitcoin Core version
    pub fn new(version: Version) -> Self { Self { version } }

    /// Generate conditional imports based on what is actually needed
    fn generate_imports(has_parameters: bool, has_structured_response: bool) -> String {
        let mut imports = vec![];
        imports.push("use serde_json::Value;".to_string());

        if has_parameters {
            imports.push("use serde_json::json;".to_string());
        }

        if has_structured_response {
            imports.push("use serde::{Deserialize, Serialize};".to_string());
        }

        imports.push("use crate::transport::{TransportTrait, TransportError};".to_string());

        imports.join("\n")
    }
}

impl CodeGenerator for TransportCodeGenerator {
    fn generate(&self, methods: &[BtcMethod]) -> Vec<(String, String)> {
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
                let docs_md = doc_comment::generate_example_docs(m, &self.version.as_doc_version())
                    .trim_end()
                    .to_string();
                let response_struct =
                    response_type::build_return_type(m).unwrap_or_default().unwrap_or_default();
                let ok_ty = if response_struct.is_empty() {
                    "Value".into()
                } else {
                    format!("{0}Response", capitalize(&m.name))
                };

                /* ---------- source file ---------- */
                let has_parameters = !m.arguments.is_empty();
                let has_structured_response = !response_struct.is_empty();
                let imports = Self::generate_imports(has_parameters, has_structured_response);

                // Add clippy allow for too many arguments if needed
                let clippy_allow = if m.arguments.len() > 7 {
                    "#[allow(clippy::too_many_arguments)]\n"
                } else {
                    ""
                };

                let src = format!(
                    r#"{docs}
#[allow(unused_imports)]
{imports}
{resp_struct}

/// Calls the `{rpc}` RPC method.
///
/// Generated transport wrapper for JSON-RPC.
{clippy_allow}pub async fn {fn_name}({fn_args}) -> Result<{ok_ty}, TransportError> {{
    let params = {params_vec};
    let raw = transport.send_request("{rpc}", &params).await?;
    {handler}
}}
"#,
                    docs = docs_md,
                    imports = imports,
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
