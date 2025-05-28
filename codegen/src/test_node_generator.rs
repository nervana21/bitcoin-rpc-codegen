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
/// * A `BitcoinTestClient` high-level abstraction that combines node management and RPC functionality
pub struct TestNodeGenerator;

impl CodeGenerator for TestNodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut params_code = String::new();
        let mut test_node_code = String::new();
        let mut result_code = String::new();
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

        /* ---------- result.rs ---------- */
        writeln!(result_code, "//! Result structs for RPC method returns").unwrap();
        writeln!(result_code, "use serde::Deserialize;\n").unwrap();
        for m in methods {
            if m.results.len() != 1 {
                continue;
            }
            let r = &m.results[0];
            let ty = rust_type_for(&r.key_name, &r.type_);
            writeln!(result_code, "#[derive(Debug, Deserialize)]").unwrap();
            writeln!(
                result_code,
                "pub struct {}Result(pub {});\n",
                camel(&m.name),
                ty
            )
            .unwrap();
        }

        /* ---------- test_node.rs ---------- */
        writeln!(test_node_code, "use anyhow::Result;").unwrap();
        writeln!(test_node_code, "use serde_json::Value;").unwrap();
        writeln!(
            test_node_code,
            "use crate::transport::core::{{TransportExt, DefaultTransport, TransportError}};"
        )
        .unwrap();
        writeln!(test_node_code, "use crate::test_node::result;").unwrap();
        writeln!(
            test_node_code,
            "use node::{{NodeManager, TestConfig, BitcoinNodeManager}};\n"
        )
        .unwrap();

        writeln!(test_node_code, "/// Low-level wrapper around a Transport.").unwrap();
        writeln!(test_node_code, "#[derive(Clone, Debug)]").unwrap();
        writeln!(test_node_code, "pub struct TestNode {{").unwrap();
        writeln!(test_node_code, "    client: Box<DefaultTransport>,").unwrap();
        writeln!(test_node_code, "}}\n").unwrap();

        writeln!(test_node_code, "impl TestNode {{").unwrap();
        writeln!(
            test_node_code,
            "    pub fn new(client: Box<DefaultTransport>) -> Self {{"
        )
        .unwrap();
        writeln!(test_node_code, "        Self {{ client }}").unwrap();
        writeln!(test_node_code, "    }}\n").unwrap();

        for m in methods {
            let method_snake = camel_to_snake_case(&m.name);
            let ret_ty = if m.results.len() == 1 {
                format!("result::{}Result", camel(&m.name))
            } else {
                "Value".to_string()
            };

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
                    "        Ok(self.client.call::<{}>(\"{}\", &[]).await?.into())",
                    ret_ty, m.name
                )
                .unwrap();
            } else {
                // Generate direct parameter list
                let mut param_list = String::new();
                for (i, arg) in m.arguments.iter().enumerate() {
                    if i > 0 {
                        param_list.push_str(", ");
                    }
                    let param_name = if arg.names[0] == "type" {
                        "_type"
                    } else {
                        &camel_to_snake_case(&arg.names[0])
                    };
                    let param_ty = rust_type_for(&arg.names[0], &arg.type_);
                    write!(param_list, "{}: {}", param_name, param_ty).unwrap();
                }

                writeln!(
                    test_node_code,
                    "    pub async fn {}(&self, {}) -> Result<{}, TransportError> {{",
                    method_snake, param_list, ret_ty
                )
                .unwrap();

                // Generate parameter serialization
                writeln!(test_node_code, "        let mut vec = vec![];").unwrap();
                for arg in &m.arguments {
                    let name = if arg.names[0] == "type" {
                        "_type"
                    } else {
                        &camel_to_snake_case(&arg.names[0])
                    };
                    writeln!(
                        test_node_code,
                        "        vec.push(serde_json::to_value({})?);",
                        name
                    )
                    .unwrap();
                }
                writeln!(
                    test_node_code,
                    "        Ok(self.client.call::<{}>(\"{}\", &vec).await?.into())",
                    ret_ty, m.name
                )
                .unwrap();
            }
            writeln!(test_node_code, "    }}\n").unwrap();
        }
        writeln!(test_node_code, "}}\n").unwrap();

        writeln!(
            test_node_code,
            "/// High-level abstraction that combines node management and RPC functionality."
        )
        .unwrap();
        writeln!(test_node_code, "#[derive(Debug)]").unwrap();
        writeln!(test_node_code, "pub struct BitcoinTestClient {{").unwrap();
        writeln!(test_node_code, "    manager: Box<dyn NodeManager>,").unwrap();
        writeln!(test_node_code, "    node: TestNode,").unwrap();
        writeln!(test_node_code, "}}\n").unwrap();

        writeln!(test_node_code, "impl BitcoinTestClient {{").unwrap();
        writeln!(
            test_node_code,
            "    async fn wait_for_node_ready(&self, max_retries: u32, delay_ms: u64) -> Result<(), TransportError> {{"
        )
        .unwrap();
        writeln!(test_node_code, "        let mut retries = 0;").unwrap();
        writeln!(test_node_code, "        while retries < max_retries {{").unwrap();
        writeln!(
            test_node_code,
            "            match self.getblockchaininfo().await {{"
        )
        .unwrap();
        writeln!(test_node_code, "                Ok(_) => return Ok(()),").unwrap();
        writeln!(test_node_code, "                Err(e) => {{").unwrap();
        writeln!(
            test_node_code,
            "                    let err_str = e.to_string();"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "                    if err_str.contains(\"Loading\")"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "                        || err_str.contains(\"Verifying\")"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "                        || err_str.contains(\"Starting\") {{"
        )
        .unwrap();
        writeln!(test_node_code, "                        retries += 1;").unwrap();
        writeln!(test_node_code, "                        tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;").unwrap();
        writeln!(test_node_code, "                        continue;").unwrap();
        writeln!(test_node_code, "                    }}").unwrap();
        writeln!(test_node_code, "                    return Err(e);").unwrap();
        writeln!(test_node_code, "                }}").unwrap();
        writeln!(test_node_code, "            }}").unwrap();
        writeln!(test_node_code, "        }}").unwrap();
        writeln!(
            test_node_code,
            "        Err(TransportError::Rpc(\"Node failed to become ready\".into()))"
        )
        .unwrap();
        writeln!(test_node_code, "    }}\n").unwrap();

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

        writeln!(test_node_code, "    pub async fn new_with_config(config: &TestConfig) -> Result<Self, TransportError> {{").unwrap();
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
        writeln!(
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
        writeln!(
            test_node_code,
            "        let btc_client = Self {{ manager, node }};"
        )
        .unwrap();
        writeln!(
            test_node_code,
            "        btc_client.wait_for_node_ready(10, 1000).await?;"
        )
        .unwrap();
        writeln!(test_node_code, "        Ok(btc_client)").unwrap();
        writeln!(test_node_code, "    }}\n").unwrap();

        // Add method delegation for all RPC methods
        for m in methods {
            let method_snake = camel_to_snake_case(&m.name);
            let ret_ty = if m.results.len() == 1 {
                format!("result::{}Result", camel(&m.name))
            } else {
                "Value".to_string()
            };

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
                // Generate direct parameter list
                let mut param_list = String::new();
                for (i, arg) in m.arguments.iter().enumerate() {
                    if i > 0 {
                        param_list.push_str(", ");
                    }
                    let param_name = if arg.names[0] == "type" {
                        "_type"
                    } else {
                        &camel_to_snake_case(&arg.names[0])
                    };
                    let param_ty = rust_type_for(&arg.names[0], &arg.type_);
                    write!(param_list, "{}: {}", param_name, param_ty).unwrap();
                }

                writeln!(
                    test_node_code,
                    "    pub async fn {}(&self, {}) -> Result<{}, TransportError> {{",
                    method_snake, param_list, ret_ty
                )
                .unwrap();
                writeln!(
                    test_node_code,
                    "        self.node.{}({}).await",
                    method_snake,
                    m.arguments
                        .iter()
                        .map(|arg| if arg.names[0] == "type" {
                            "_type".to_string()
                        } else {
                            camel_to_snake_case(&arg.names[0])
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .unwrap();
            }
            writeln!(test_node_code, "    }}\n").unwrap();
        }

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
        writeln!(test_node_code, "    }}").unwrap();
        writeln!(test_node_code, "}}").unwrap();

        writeln!(mod_rs_code, "//! Test node module for Bitcoin RPC testing").unwrap();
        writeln!(mod_rs_code, "pub mod params;").unwrap();
        writeln!(mod_rs_code, "pub mod result;").unwrap();
        writeln!(mod_rs_code, "pub mod test_node;").unwrap();
        writeln!(
            mod_rs_code,
            "pub use test_node::{{TestNode, BitcoinTestClient}};"
        )
        .unwrap();

        vec![
            ("test_node.rs".to_string(), test_node_code),
            ("params.rs".to_string(), params_code),
            ("result.rs".to_string(), result_code),
            ("mod.rs".to_string(), mod_rs_code),
        ]
    }
}

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
