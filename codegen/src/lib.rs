//! Code generation utilities for Bitcoin Core RPC.
//!
//! The crate’s job is *only* to turn `ApiMethod` descriptors
//! into ready‑to‑`cargo check` Rust modules.  Runtime testing,
//! node spawning, and schema extraction belong in other crates.

#![warn(missing_docs)]

use anyhow::Result;
use rpc_api::ApiMethod;
use std::{fs, io::Write, path::Path};

// ---------------------------------------------------------------------------
//  Sub‑crates that do the heavy lifting
// ---------------------------------------------------------------------------

/// **`client_macros`** – Emits the `macro_rules!` blocks that expand into
/// ergonomic, version‑scoped client wrappers.  
/// A downstream crate can simply `impl_client_latest__getblockchaininfo!()` and
/// obtain a fully‑typed `fn getblockchaininfo(&self) -> …` on its `Client`.
pub mod client_macros;

/// **`discover`** – Runtime discovery helpers.  
/// Talks to a local `bitcoin-cli` binary to list available RPC methods,
/// download their help‑text, and turn that into a minimal `ApiMethod` set.
/// Used by the *discovery* pipeline mode so we can generate against whatever
/// node version happens to be on `PATH`.
pub mod discover;

/// **`docs`** – Rust‑doc & Markdown generation utilities.  
/// Converts `ApiMethod` metadata into nice triple‑slash comments and “Example:”
/// blocks that are injected at the top of every generated source file.
pub mod docs;

/// **`module_generator`** – Writes the `mod.rs` scaffolding.  
/// Given a set of schema versions (`v28`, `v29`, `latest`…) it produces:
///  generated/
///   ├─ client/ v28/ v29/ …
///   └─ types/  v28_types/ …
///   plus top‑level mod.rs that re‑export everything
/// so that downstream crates can just `use generated::client::*;`.
pub mod module_generator;

/// **`types`** – Shapes the JSON you get back from Core into real Rust.  
///   * Parses each method’s _Result:_ section (or the pre‑built `api.json`)  
///   * Builds a strongly‑typed `…Response` struct with the right `serde`
///     attributes (`Option<T>` + `skip_serializing_if`)  
///   * Exposes **`TypesCodeGenerator`** which writes one
///     `<method>_response.rs` file per RPC – these are imported by the
///     transport wrapper so users work with first‑class Rust types instead of
///     raw `Value`.
pub mod types;

// Re‑export so downstream crates can just `use codegen::ClientCodeGenerator`
// pub use client_macros::ClientGenerator as ClientCodeGenerator;
pub use types::TypesCodeGenerator;

/// ---------------------------------------------------------------------------
/// 1. Common helper traits / functions
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
        let mut f = fs::File::create(path)?;
        f.write_all(src.as_bytes())?;
    }
    Ok(())
}

/// ---------------------------------------------------------------------------
/// 2. A trivial stub generator (handy for tests / scaffolding)
/// ---------------------------------------------------------------------------
pub struct BasicCodeGenerator;

impl CodeGenerator for BasicCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        methods
            .iter()
            .map(|m| {
                let src = format!(
                    r#"// Auto‑generated stub for RPC method `{n}`

pub fn {n}() {{
    unimplemented!();
}}
"#,
                    n = m.name
                );
                (m.name.clone(), src)
            })
            .collect()
    }
}

/// ---------------------------------------------------------------------------
/// 3. The real generator: async JSON‑RPC wrappers
/// ---------------------------------------------------------------------------
pub struct TransportCodeGenerator;

impl CodeGenerator for TransportCodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        use types::{capitalize, generate_return_type, sanitize_method_name};

        methods
            .iter()
            .map(|m| {
                /* ---------- fn signature ---------- */
                let fn_args = std::iter::once("transport: &Transport".into())
                    .chain(
                        m.arguments
                            .iter()
                            .map(|a| format!("{}: serde_json::Value", a.names[0])),
                    )
                    .collect::<Vec<_>>()
                    .join(", ");

                /* ---------- params vec ---------- */
                let params_vec = if m.arguments.is_empty() {
                    "Vec::<Value>::new()".into()
                } else {
                    let elems = m
                        .arguments
                        .iter()
                        .map(|a| format!("json!({})", a.names[0]))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("vec![{elems}]")
                };

                /* ---------- docs + types ---------- */
                let docs_md = docs::generate_example_docs(m, "latest");
                let response_struct = generate_return_type(m).unwrap_or_default();
                let ok_ty = if response_struct.is_empty() {
                    "Value".into()
                } else {
                    format!("{0}Response", capitalize(&m.name))
                };

                /* ---------- source file ---------- */
                let src = format!(
                    r#"{docs}

use serde_json::{{json, Value}};
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
