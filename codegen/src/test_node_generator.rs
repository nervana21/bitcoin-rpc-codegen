//! Code-gen: build a thin `TestNode` client with typed-parameter helpers.
//!
//! Until we have a TypesCodeGenerator that emits concrete `*Response` structs
//! every RPC simply returns `serde_json::Value`.

use crate::{doc_comment_generator, CodeGenerator, TYPE_REGISTRY};
use rpc_api::ApiMethod;
use std::fmt::Write as _;
// use transport::{Transport, TransportError};

/// Generates a `test_node.rs` module containing:
/// * `params::{Method}Params` structs for each RPC that accepts arguments
/// * A `TestNode` wrapper with one async method per RPC
/// * A `BitcoinTestClient` high-level abstraction that combines node management and RPC functionality
pub struct TestNodeGenerator;

impl CodeGenerator for TestNodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut params_code = String::new();
        let mut test_node_code = String::new();
        let mut mod_rs_code = String::new();

        /* ---------- params.rs ---------- */
        writeln!(params_code, "//! Parameter structs for RPC method calls").unwrap();
        writeln!(params_code, "use serde::Serialize;\n").unwrap();

        for m in methods {
            if m.arguments.is_empty() {
                continue;
            }
            writeln!(params_code, "/// Parameters for the `{}` RPC call.", m.name).unwrap();
            writeln!(params_code, "#[derive(Debug, Serialize)]").unwrap();
            writeln!(params_code, "pub struct {}Params {{", camel(&m.name)).unwrap();

            for p in &m.arguments {
                let field = if p.names[0] == "type" {
                    "_type".to_string()
                } else {
                    camel_to_snake_case(&p.names[0]).replace('-', "_")
                };
                let ty = rust_type_for(&p.names[0], &p.type_);
                writeln!(params_code, "    pub {}: {},", field, ty).unwrap();
            }
            writeln!(params_code, "}}\n").unwrap();
        }

        /* ---------- test_node.rs ---------- */
        writeln!(test_node_code, "use anyhow::Result;").unwrap();
        writeln!(test_node_code, "use serde_json::Value;").unwrap();
        writeln!(
            test_node_code,
            "use crate::transport::core::{{TransportExt, DefaultTransport, TransportError}};"
        )
        .unwrap();
        writeln!(test_node_code, "use crate::test_node::params;\n").unwrap();
        writeln!(
            test_node_code,
            "use node::{{NodeManager, TestConfig, BitcoinNodeManager}};\n"
        )
        .unwrap();
        writeln!(test_node_code, "\n").unwrap();

        /* ---------- TestNode wrapper ---------- */
        writeln!(test_node_code, "/// Low-level wrapper around a Transport.").unwrap();
        writeln!(test_node_code, "#[derive(Clone)]").unwrap();
        writeln!(test_node_code, "pub struct TestNode {{").unwrap();
        writeln!(test_node_code, "    client: Box<DefaultTransport>,").unwrap();
        writeln!(test_node_code, "}}\n").unwrap();

        writeln!(test_node_code, "impl TestNode {{").unwrap();
        writeln!(
            test_node_code,
            "    /// Build from an already-initialised transport."
        )
        .unwrap();
        writeln!(
            test_node_code,
            "    pub fn new(client: Box<DefaultTransport>) -> Self {{"
        )
        .unwrap();
        writeln!(test_node_code, "        Self {{ client }}").unwrap();
        writeln!(test_node_code, "    }}\n").unwrap();

        /* ---------- per-RPC helpers ---------- */
        for m in methods {
            let method_snake = camel_to_snake_case(&m.name);
            let ret_ty = if m.results.is_empty() { "()" } else { "Value" };

            writeln!(
                test_node_code,
                "{}",
                doc_comment_generator::format_doc_comment(&m.description)
            )
            .unwrap();

            if m.arguments.is_empty() {
                writeln!(
                    test_node_code,
                    "    pub async fn {}(&self) -> Result<{}, TransportError> {{",
                    method_snake, ret_ty
                )
                .unwrap();
                writeln!(
                    test_node_code,
                    "        self.client.call(\"{}\", &[]).await",
                    m.name
                )
                .unwrap();
            } else {
                writeln!(
                    test_node_code,
                    "    pub async fn {}(&self, params: params::{}Params) \
                     -> Result<{}, TransportError> {{",
                    method_snake,
                    camel(&m.name),
                    ret_ty
                )
                .unwrap();
                writeln!(
                    test_node_code,
                    "        let params = serde_json::to_value(params)?;"
                )
                .unwrap();
                writeln!(
                    test_node_code,
                    "        self.client.call(\"{}\", &[params]).await",
                    m.name
                )
                .unwrap();
            }
            writeln!(test_node_code, "    }}\n").unwrap();
        }
        writeln!(test_node_code, "}}\n").unwrap();

        /* ---------- BitcoinTestClient high-level abstraction ---------- */
        writeln!(
            test_node_code,
            "/// High-level abstraction that combines node management and RPC functionality."
        )
        .unwrap();
        writeln!(
            test_node_code,
            "/// This is the recommended way to interact with a Bitcoin test node."
        )
        .unwrap();
        writeln!(test_node_code, "pub struct BitcoinTestClient {{").unwrap();
        writeln!(test_node_code, "    manager: Box<dyn NodeManager>,").unwrap();
        writeln!(test_node_code, "    node: TestNode,").unwrap();
        writeln!(test_node_code, "}}\n").unwrap();

        writeln!(test_node_code, "impl BitcoinTestClient {{").unwrap();
        writeln!(
            test_node_code,
            "    /// Create a new test client with default configuration."
        )
        .unwrap();
        writeln!(
            test_node_code,
            "    pub async fn new() -> Result<Self, TransportError> {{"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "        Self::new_with_config(&TestConfig::default()).await"
        )
        .unwrap();
        writeln!(test_node_code, "    }}\n").unwrap();

        writeln!(
            test_node_code,
            "    /// Create a new test client with custom configuration."
        )
        .unwrap();
        writeln!(
            test_node_code,
            "    pub async fn new_with_config(config: &TestConfig) -> Result<Self, TransportError> {{"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "        let manager = Box::new(BitcoinNodeManager::new_with_config(config)?);"
        )
        .unwrap();
        writeln!(test_node_code, "        manager.start().await?;").unwrap();
        writeln!(
            test_node_code,
            "        let client = Box::new(DefaultTransport::new("
        )
        .unwrap();
        write!(
            test_node_code,
            "            format!(\"http://127.0.0.1:{{}}\", manager.rpc_port()),"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "            Some((config.rpc_username.clone(), config.rpc_password.clone())),"
        )
        .unwrap();
        writeln!(test_node_code, "        ));").unwrap();
        writeln!(test_node_code, "        let node = TestNode::new(client);").unwrap();
        writeln!(test_node_code, "        Ok(Self {{ manager, node }})").unwrap();
        writeln!(test_node_code, "    }}\n").unwrap();

        writeln!(
            test_node_code,
            "    /// Shutdown the node and clean up resources."
        )
        .unwrap();
        writeln!(
            test_node_code,
            "    pub async fn shutdown(mut self) -> Result<(), TransportError> {{"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "        self.manager.stop().await.map_err(TransportError::from)"
        )
        .unwrap();
        writeln!(test_node_code, "    }}\n").unwrap();

        writeln!(
            test_node_code,
            "    /// Check if the node is ready by verifying it's fully initialized."
        )
        .unwrap();
        writeln!(
            test_node_code,
            "    pub async fn is_ready(&self) -> bool {{"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "        // First check if we can get network info"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "        match self.node.getnetworkinfo().await {{"
        )
        .unwrap();
        writeln!(test_node_code, "            Ok(_) => {{").unwrap();
        writeln!(
            test_node_code,
            "                // Then check if the node is fully initialized"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "                match self.node.getblockchaininfo().await {{"
        )
        .unwrap();
        writeln!(test_node_code, "                    Ok(info) => {{").unwrap();
        writeln!(
            test_node_code,
            "                        // Check if the node is still in initial block download"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "                        if let Some(initial_block_download) = info.get(\"initialblockdownload\") {{"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "                            if let Some(ibd) = initial_block_download.as_bool() {{"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "                                return !ibd;"
        )
        .unwrap();
        writeln!(test_node_code, "                            }}").unwrap();
        writeln!(test_node_code, "                        }}").unwrap();
        writeln!(test_node_code, "                        false").unwrap();
        writeln!(test_node_code, "                    }}").unwrap();
        writeln!(test_node_code, "                    Err(_) => false").unwrap();
        writeln!(test_node_code, "                }}").unwrap();
        writeln!(test_node_code, "            }}").unwrap();
        writeln!(test_node_code, "            Err(_) => false").unwrap();
        writeln!(test_node_code, "        }}").unwrap();
        writeln!(test_node_code, "    }}").unwrap();

        /* ---------- Delegate RPC methods to TestNode ---------- */
        for m in methods {
            let method_snake = camel_to_snake_case(&m.name);
            let ret_ty = if m.results.is_empty() { "()" } else { "Value" };

            writeln!(
                test_node_code,
                "{}",
                doc_comment_generator::format_doc_comment(&m.description)
            )
            .unwrap();

            if m.arguments.is_empty() {
                writeln!(
                    test_node_code,
                    "    pub async fn {}(&self) -> Result<{}, TransportError> {{",
                    method_snake, ret_ty
                )
                .unwrap();
                writeln!(test_node_code, "        self.node.{}().await", method_snake).unwrap();
            } else {
                writeln!(
                    test_node_code,
                    "    pub async fn {}(&self, params: params::{}Params) \
                     -> Result<{}, TransportError> {{",
                    method_snake,
                    camel(&m.name),
                    ret_ty
                )
                .unwrap();
                writeln!(
                    test_node_code,
                    "        self.node.{}(params).await",
                    method_snake
                )
                .unwrap();
            }
            writeln!(test_node_code, "    }}\n").unwrap();
        }
        writeln!(test_node_code, "}}").unwrap();

        /* ---------- mod.rs ---------- */
        writeln!(mod_rs_code, "//! Test node module for Bitcoin RPC testing").unwrap();
        writeln!(mod_rs_code, "pub mod params;").unwrap();
        writeln!(mod_rs_code, "pub mod test_node;").unwrap();
        writeln!(
            mod_rs_code,
            "pub use test_node::{{TestNode, BitcoinTestClient}};"
        )
        .unwrap();

        vec![
            ("test_node.rs".to_string(), test_node_code),
            ("params.rs".to_string(), params_code),
            ("mod.rs".to_string(), mod_rs_code),
        ]
    }
}

/* ── helpers ──────────────────────────────────────────────────── */

fn rust_type_for(param_name: &str, api_ty: &str) -> &'static str {
    let (ty, _) = TYPE_REGISTRY.map_type(api_ty, param_name);
    ty
}

fn camel_to_snake_case(s: &str) -> String {
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
