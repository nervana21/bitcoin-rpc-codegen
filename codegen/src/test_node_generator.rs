//! Code-gen: build a thin `TestNode` client with typed-parameter helpers.
//!
//! Until we have a TypesCodeGenerator that emits concrete `*Response` structs
//! every RPC simply returns `serde_json::Value`.

use crate::{doc_comment_generator, CodeGenerator, TYPE_REGISTRY};
use rpc_api::ApiMethod;
use std::fmt::Write as _;

/// Generates a `test_node.rs` module containing:
/// * `params::{Method}Params` structs for each RPC that accepts arguments
/// * A `TestNode` wrapper with one async method per RPC
pub struct TestNodeGenerator;

impl CodeGenerator for TestNodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut code = String::new();

        /* ---------- prelude ---------- */
        writeln!(code, "use anyhow::Result;").unwrap();
        writeln!(code, "use serde_json::Value;").unwrap();
        writeln!(
            code,
            "use bitcoin::{{Amount, Txid, BlockHash, ScriptBuf, PublicKey}};"
        )
        .unwrap();
        writeln!(code, "use transport::Transport;\n").unwrap();

        /* ---------- parameter structs ---------- */
        writeln!(code, "pub mod params {{").unwrap();
        writeln!(code, "    use super::*;\n").unwrap();
        for m in methods {
            if m.arguments.is_empty() {
                continue;
            }
            writeln!(code, "    /// Parameters for the `{}` RPC call.", m.name).unwrap();
            writeln!(code, "    #[derive(Debug, serde::Serialize)]").unwrap();
            writeln!(code, "    pub struct {}Params {{", camel(&m.name)).unwrap();

            for p in &m.arguments {
                let field = if p.names[0] == "type" {
                    "_type".to_string()
                } else {
                    snake(&p.names[0]).replace('-', "_")
                };
                let ty = rust_type_for(&p.names[0], &p.type_);
                writeln!(code, "        pub {}: {},", field, ty).unwrap();
            }
            writeln!(code, "    }}\n").unwrap();
        }
        writeln!(code, "}}\n").unwrap();

        /* ---------- TestNode wrapper ---------- */
        writeln!(
            code,
            "/// High-level wrapper around a [`transport::Transport`]."
        )
        .unwrap();
        writeln!(code, "#[derive(Clone)]").unwrap();
        writeln!(code, "pub struct TestNode {{").unwrap();
        writeln!(code, "    client: Transport,").unwrap();
        writeln!(code, "}}\n").unwrap();

        writeln!(code, "impl TestNode {{").unwrap();
        writeln!(code, "    /// Build from an already-initialised transport.").unwrap();
        writeln!(code, "    pub fn new(client: Transport) -> Self {{").unwrap();
        writeln!(code, "        Self {{ client }}").unwrap();
        writeln!(code, "    }}\n").unwrap();

        /* ---------- per-RPC helpers ---------- */
        for m in methods {
            let method_snake = snake(&m.name);
            let ret_ty = if m.results.is_empty() { "()" } else { "Value" };

            writeln!(
                code,
                "{}",
                doc_comment_generator::format_doc_comment(&m.description)
            )
            .unwrap();

            if m.arguments.is_empty() {
                writeln!(
                    code,
                    "    pub async fn {}(&self) -> Result<{}, anyhow::Error> {{",
                    method_snake, ret_ty
                )
                .unwrap();
                writeln!(
                    code,
                    "        self.client.call::<(), _>(\"{}\", &[]).await.map_err(anyhow::Error::from)",
                    m.name
                )
                .unwrap();
            } else {
                writeln!(
                    code,
                    "    pub async fn {}(&self, params: params::{}Params) \
                     -> Result<{}, anyhow::Error> {{",
                    method_snake,
                    camel(&m.name),
                    ret_ty
                )
                .unwrap();
                writeln!(
                    code,
                    "        self.client.call(\"{}\", &[params]).await.map_err(anyhow::Error::from)",
                    m.name
                )
                .unwrap();
            }
            writeln!(code, "    }}\n").unwrap();
        }
        writeln!(code, "}}").unwrap();

        vec![("test_node".to_owned(), code)]
    }
}

/* ── helpers ──────────────────────────────────────────────────── */

fn rust_type_for(param_name: &str, api_ty: &str) -> &'static str {
    let (ty, _) = TYPE_REGISTRY.map_type(api_ty, param_name);
    ty
}

fn snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut up = true;
    for ch in s.chars() {
        if ch == '_' || ch == '-' {
            up = true;
        } else if up {
            out.push(ch.to_ascii_uppercase());
            up = false;
        } else {
            out.push(ch);
        }
    }
    out
}
