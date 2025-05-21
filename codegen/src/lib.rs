//! Code generation utilities for Bitcoin Core RPC.
//!
//! The crate's job is *only* to turn `ApiMethod` descriptors
//! into ready‑to‑`cargo check` Rust modules.  Runtime testing,
//! node spawning, and schema extraction belong in other crates.

#![warn(missing_docs)]

use anyhow::Result;
use lazy_static::lazy_static;
use rpc_api::ApiMethod;
use std::{fs, path::Path};

// ---------------------------------------------------------------------------
//  Sub‑crates that do the heavy lifting
// ---------------------------------------------------------------------------

/// **`rpc_method_macro_generator`** – Emits the `macro_rules!` blocks that expand into
/// ergonomic, version‑scoped client wrappers.  
/// A downstream crate can simply `impl_client_latest__getblockchaininfo!()` and
/// obtain a fully‑typed `fn getblockchaininfo(&self) -> …` on its `Client`.
pub mod rpc_method_macro_generator;

/// **`rpc_method_discovery`** – Runtime discovery helpers.  
/// Talks to a local `bitcoin-cli` binary to list available RPC methods,
/// download their help‑text, and turn that into a minimal `ApiMethod` set.
/// Used by the *discovery* pipeline mode so we can generate against whatever
/// node version happens to be on `PATH`.
pub mod rpc_method_discovery;

/// **`docs`** – Rust‑doc & Markdown generation utilities.  
/// Converts `ApiMethod` metadata into nice triple‑slash comments and "Example:"
/// blocks that are injected at the top of every generated source file.
pub mod doc_comment_generator;

/// **`namespace_scaffolder`** – Writes the `mod.rs` scaffolding.  
/// Given a set of schema versions (`v28`, `v29`, `latest`…) it produces:
///  generated/
///   ├─ client/ v28/ v29/ …
///   └─ types/  v28_types/ …
///   plus top‑level mod.rs that re‑export everything
/// so that downstream crates can just `use generated::client::*;`.
pub mod namespace_scaffolder;

/// **`rpc_client_generator`** – Generates transport layer code.
/// Creates async RPC method wrappers that handle parameter serialization
/// and response deserialization.
pub mod rpc_client_generator;

/// **`test_node_generator`** – Generates convenience methods for TestNode.
/// Creates simple wrapper methods that delegate to the underlying RPC client,
/// making the API more ergonomic for users.
pub mod test_node_generator;

/// **`types`** – Shapes the JSON you get back from Core into real Rust.  
///   * Parses each method's _Result:_ section (or the pre‑built `api.json`)  
///   * Builds a strongly‑typed `…Response` struct with the right `serde`
///     attributes (`Option<T>` + `skip_serializing_if`)  
///   * Exposes **`TypesCodeGenerator`** which writes one
///     `<method>_response.rs` file per RPC – these are imported by the
///     transport wrapper so users work with first‑class Rust types instead of
///     raw `Value`.
pub mod response_type_generator;

pub use response_type_generator::TypesCodeGenerator;

/// Sub-crate: **`type_registry`**
///
/// Central registry for mapping Bitcoin RPC types to Rust types.
/// Provides `TypeRegistry` and `TypeMapping` for canonical type conversions.
pub mod type_registry;
pub use type_registry::{TypeMapping, TypeRegistry};

/// ---------------------------------------------------------------------------
/// 1. Common helper traits / functions
/// ---------------------------------------------------------------------------
/// Anything that outputs `(module_name, source_code)` pairs.
pub trait CodeGenerator {
    /// Create Rust source files for the supplied API methods.
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)>;

    /// Optional post‑generation check (defaults to a no‑op).
    fn validate(&self, _methods: &[ApiMethod]) -> Result<()> {
        Ok(())
    }
}

/// Write each `(name, src)` pair to `<out_dir>/<name>.rs`.
pub fn write_generated<P: AsRef<Path>>(
    out_dir: P,
    files: &[(String, String)],
) -> std::io::Result<()> {
    fs::create_dir_all(&out_dir)?;
    for (name, src) in files {
        let path = out_dir.as_ref().join(format!("{name}.rs"));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        // Force write the file
        fs::write(path, src.as_bytes())?;
    }
    Ok(())
}

/// ---------------------------------------------------------------------------
/// TransportCodeGenerator: generates async JSON‑RPC wrapper functions
/// ---------------------------------------------------------------------------

pub struct TransportCodeGenerator;

impl CodeGenerator for TransportCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        use response_type_generator::{capitalize, generate_return_type, sanitize_method_name};

        methods
            .iter()
            .map(|m| {
                /* ---------- fn signature ---------- */
                let fn_args = std::iter::once("transport: &Transport".into())
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
                let docs_md = doc_comment_generator::generate_example_docs(m, "latest")
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

use serde_json::{{Value, json}};
use transport::{{Transport, TransportError}};
{resp_struct}

/// Calls the `{rpc}` RPC method.
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
