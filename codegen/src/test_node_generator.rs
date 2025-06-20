//! Code-gen: build a thin `TestNode` client with typed-parameter helpers.
//!
//! Until we have a TypesCodeGenerator that emits concrete `*Response` structs
//! every RPC simply returns `serde_json::Value`.
/*!
TODO: Modularize this generator.

This file will eventually become thousands of lines long.
Split into dedicated modules under `codegen/src/test_node/`:

  - `emit_params.rs`: Generate `Params` structs
  - `emit_results.rs`: Generate `Result` structs
  - `emit_subclient.rs`: Generate node/wallet subclients
  - `emit_combined_client.rs`: Generate `BitcoinTestClient`, constructors, and lifecycle helpers
      → Add all ergonomic helpers here (e.g., `reset_chain()`, `mine_blocks()`, etc.)
  - `utils.rs`: Utility functions (e.g. `camel()`, `rust_type_for()`)

Benefits:
- Better maintainability
- Easier to test and extend
- Faster onboarding for contributors
*/

use crate::utils::camel_to_snake_case;
use crate::wallet_methods::WALLET_METHODS;
use crate::{generators::doc_comment, CodeGenerator, TYPE_REGISTRY};
use rpc_api::{ApiArgument, ApiMethod};
use std::fmt::Write as _;

/// A code generator that creates a type-safe Rust client library for Bitcoin Core test environments.
///
/// This generator takes Bitcoin Core RPC API definitions and produces a complete Rust client library
/// that provides a high-level, type-safe interface for:
/// - Node lifecycle management (start/stop)
/// - Wallet management and operations
/// - Block mining and chain manipulation
/// - All Bitcoin Core RPC methods with proper typing
///
/// The generated client library serves as a test harness that bridges Bitcoin Core's RPC interface
/// with Rust's type system, making it easier to write reliable Bitcoin Core integration tests
/// without dealing with low-level RPC details.
///
/// The generator produces several key components:
/// - Type-safe parameter structs for RPC calls
/// - Type-safe result structs for RPC responses
/// - A high-level `BitcoinTestClient` with ergonomic helpers
/// - Separate node and wallet client interfaces
///
/// This abstraction layer enables developers to focus on test logic rather than RPC mechanics,
/// while maintaining type safety and proper error handling throughout the test suite.
pub struct TestNodeGenerator {
    version: String,
}

impl TestNodeGenerator {
    /// Creates a new `TestNodeGenerator` configured for a specific Bitcoin Core version.
    ///
    /// The `version` string determines which RPC methods and structures are used when generating
    /// type-safe test clients and associated modules. This allows test code to stay in sync with
    /// version-specific behavior in Bitcoin Core.
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }
}

impl CodeGenerator for TestNodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let params_code = generate_params_code(methods);
        let result_code = generate_result_code(methods);

        let wallet_methods: Vec<_> = methods
            .iter()
            .filter(|m| WALLET_METHODS.contains(&m.name.as_str()))
            .cloned()
            .collect();
        let node_methods: Vec<_> = methods
            .iter()
            .filter(|m| !WALLET_METHODS.contains(&m.name.as_str()))
            .cloned()
            .collect();

        let wallet_code =
            generate_subclient("BitcoinWalletClient", &wallet_methods, &self.version).unwrap();
        let node_code =
            generate_subclient("BitcoinNodeClient", &node_methods, &self.version).unwrap();
        let client_code =
            generate_combined_client("BitcoinTestClient", methods, &self.version).unwrap();

        let mod_rs_code = generate_mod_rs();

        vec![
            ("wallet.rs".to_string(), wallet_code),
            ("node.rs".to_string(), node_code),
            ("client.rs".to_string(), client_code),
            ("params.rs".to_string(), params_code),
            ("response.rs".to_string(), result_code),
            ("mod.rs".to_string(), mod_rs_code),
        ]
    }
}

fn generate_params_code(methods: &[ApiMethod]) -> String {
    let mut code =
        String::from("//! Parameter structs for RPC method calls\nuse serde::Serialize;\n\n");
    for m in methods {
        if m.arguments.is_empty() {
            continue;
        }
        writeln!(code, "{}", doc_comment::format_doc_comment(&m.description)).unwrap();
        writeln!(
            code,
            "#[derive(Debug, Serialize)]\npub struct {}Params {{",
            camel(&m.name)
        )
        .unwrap();
        for p in &m.arguments {
            let field = if p.names[0] == "type" {
                "_type"
            } else {
                &camel_to_snake_case(&p.names[0])
            };
            let ty = rust_type_for(&p.names[0], &p.type_);
            writeln!(code, "    pub {field}: {ty},").unwrap();
        }
        writeln!(code, "}}\n").unwrap();
    }
    code
}

fn generate_result_code(methods: &[ApiMethod]) -> String {
    let mut code =
        String::from("//! Result structs for RPC method returns\nuse serde::Deserialize;\n\n");
    for m in methods {
        if m.results.len() != 1 {
            continue;
        }
        let r = &m.results[0];
        let ty = rust_type_for(&r.key_name, &r.type_);
        writeln!(
            code,
            "#[derive(Debug, Deserialize)]\n#[serde(transparent)]\npub struct {}Response(pub {});\n",
            camel(&m.name),
            ty
        )
        .unwrap();
    }
    code
}

fn generate_mod_rs() -> String {
    let mut code = String::new();
    writeln!(
        code,
        "//! Test node module for Bitcoin RPC testing
#[cfg(test)]
pub mod params;
pub mod response;
pub mod wallet;
pub mod node;
pub mod client;

// re-export common clients
pub use client::BitcoinTestClient;
pub use wallet::BitcoinWalletClient;
pub use node::BitcoinNodeClient;
"
    )
    .unwrap();
    code
}

fn rust_type_for(param_name: &str, api_ty: &str) -> String {
    let (base_ty, is_option) = TYPE_REGISTRY.map_argument_type(&ApiArgument {
        type_: api_ty.to_string(),
        names: vec![param_name.to_string()],
        optional: false,
        description: String::new(),
    });
    if is_option {
        format!("Option<{base_ty}>")
    } else {
        base_ty.to_string()
    }
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

fn generate_subclient(
    client_name: &str,
    methods: &[ApiMethod],
    version: &str,
) -> std::io::Result<String> {
    use std::fmt::Write;
    let mut code = String::new();

    // Add #[cfg(test)] at the top of the file
    writeln!(code, "#[cfg(test)]").unwrap();

    // Check if any method uses serde_json::Value
    let needs_value = methods.iter().any(|m| {
        !m.arguments.is_empty()
            || (m.results.len() == 1 && m.results[0].type_.to_lowercase() != "none")
    });

    writeln!(
        code,
        "use anyhow::Result;
use std::sync::Arc;
use crate::transport::core::{{TransportExt, TransportError}};
use crate::transport::{{DefaultTransport}};
use crate::types::{}_types::*;
{}",
        version,
        if needs_value {
            "#[cfg(test)]\nuse serde_json::Value;\n"
        } else {
            ""
        }
    )
    .map_err(std::io::Error::other)?;

    writeln!(
        code,
        "#[derive(Debug, Clone)]
pub struct {client_name} {{
    client: Arc<DefaultTransport>,
}}

impl {client_name} {{
    pub fn new(client: Arc<DefaultTransport>) -> Self {{
        Self {{ client }}
    }}

    pub fn with_transport(&mut self, client: Arc<DefaultTransport>) {{
        self.client = client;
    }}"
    )
    .map_err(std::io::Error::other)?;

    // 2) One method per RPC
    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let returns_unit = m.results.is_empty() || m.results[0].type_.to_lowercase() == "none";
        let ret_ty = if returns_unit {
            "()".to_string()
        } else {
            format!("{}Response", camel(&m.name))
        };

        // doc comments
        writeln!(
            code,
            "\n{}",
            doc_comment::format_doc_comment(&m.description)
        )
        .map_err(std::io::Error::other)?;

        // signature line
        let params_sig = if m.arguments.is_empty() {
            "".to_string()
        } else {
            m.arguments
                .iter()
                .map(|arg| {
                    let name = if arg.names[0] == "type" {
                        "_type".to_string()
                    } else {
                        camel_to_snake_case(&arg.names[0])
                    };
                    let ty = rust_type_for(&arg.names[0], &arg.type_);
                    format!("{name}: {ty}")
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        writeln!(
            code,
            "    pub async fn {method}(&self{sig}) -> Result<{ret}, TransportError> {{",
            method = method_snake,
            sig = if params_sig.is_empty() {
                "".into()
            } else {
                format!(", {params_sig}")
            },
            ret = ret_ty,
        )
        .map_err(std::io::Error::other)?;

        // build params vector
        if !m.arguments.is_empty() {
            writeln!(code, "        let mut params = Vec::new();")
                .map_err(std::io::Error::other)?;
            for arg in &m.arguments {
                let name = if arg.names[0] == "type" {
                    "_type"
                } else {
                    &camel_to_snake_case(&arg.names[0])
                };
                // Always convert to Value, regardless of type
                writeln!(code, "        params.push(serde_json::to_value({name})?);")
                    .map_err(std::io::Error::other)?;
            }
        }

        // call
        writeln!(code, "        // dispatch and deserialize to `{ret_ty}`")
            .map_err(std::io::Error::other)?;
        writeln!(
            code,
            "        self.client.call::<{ret}>(\"{rpc}\", &{vec}).await",
            ret = ret_ty,
            rpc = m.name,
            vec = if m.arguments.is_empty() {
                "[]".to_string()
            } else {
                "params".to_string()
            },
        )
        .map_err(std::io::Error::other)?;

        // close fn
        writeln!(code, "    }}").map_err(std::io::Error::other)?;
    }

    // 3) Close impl block
    writeln!(code, "}}\n").map_err(std::io::Error::other)?;
    Ok(code)
}

fn generate_combined_client(
    client_name: &str,
    methods: &[ApiMethod],
    version: &str,
) -> std::io::Result<String> {
    let mut code = String::new();

    emit_imports(&mut code, version)?;
    emit_node_manager_trait(&mut code)?;
    emit_struct_definition(&mut code, client_name)?;
    emit_node_manager_impl(&mut code)?;
    emit_wallet_options_struct(&mut code)?;
    emit_impl_block_start(&mut code, client_name)?;
    emit_constructors(&mut code)?;
    emit_wallet_methods(&mut code)?;
    emit_block_mining_helpers(&mut code)?;
    emit_reset_chain(&mut code)?;
    emit_stop_node(&mut code)?;
    emit_node_manager_accessor(&mut code)?;
    emit_rpc_accessor(&mut code)?;
    emit_batch_method(&mut code)?;
    emit_delegated_rpc_methods(&mut code, methods)?;
    emit_send_to_address_helpers(&mut code)?;
    emit_impl_block_end(&mut code)?;
    emit_drop_impl(&mut code, client_name)?;

    Ok(code)
}

fn emit_imports(code: &mut String, version: &str) -> std::io::Result<()> {
    writeln!(
        code,
        "use anyhow::Result;
use std::sync::Arc;
use crate::transport::core::{{TransportError}};
use crate::transport::{{DefaultTransport, RpcClient, BatchBuilder}};
use crate::types::{version}_types::*;
use serde_json::Value;

use crate::node::{{BitcoinNodeManager, TestConfig}};

use super::node::BitcoinNodeClient;
use super::wallet::BitcoinWalletClient;

use std::str::FromStr;
use bitcoin::Amount;"
    )
    .unwrap();
    Ok(())
}

fn emit_node_manager_trait(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "/// Trait for managing a Bitcoin node's lifecycle
pub trait NodeManager: Send + Sync + std::fmt::Debug + std::any::Any {{
    fn start(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;
    fn stop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;
    fn rpc_port(&self) -> u16;
    fn as_any(&self) -> &dyn std::any::Any;
}}\n"
    )
    .unwrap();
    Ok(())
}

fn emit_struct_definition(code: &mut String, client_name: &str) -> std::io::Result<()> {
    writeln!(
        code,
        "#[derive(Debug)]\n\
         pub struct {client_name} {{\n\
             node_client: BitcoinNodeClient,\n\
             wallet_client: BitcoinWalletClient,\n\
             node_manager: Option<Box<dyn NodeManager>>,\n\
             /// A thin RPC wrapper around the transport, with batching built in\n\
             rpc: RpcClient,\n\
         }}\n"
    )
    .unwrap();
    Ok(())
}

fn emit_node_manager_impl(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "impl NodeManager for BitcoinNodeManager {{\n\
             fn start(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>> {{\n\
                 println!(\"[DEBUG] NodeManager::start called on BitcoinNodeManager\");\n\
                 Box::pin(async move {{\n\
                     println!(\"[DEBUG] Inside NodeManager::start async block\");\n\
                     let result = self.start_internal().await;\n\
                     println!(\"[DEBUG] NodeManager::start result: {{:?}}\", result);\n\
                     result.map_err(|e| TransportError::Rpc(e.to_string()))\n\
                 }})\n\
             }}\n\
             \n\
             fn stop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>> {{\n\
                 println!(\"[DEBUG] NodeManager::stop called on BitcoinNodeManager\");\n\
                 Box::pin(async move {{\n\
                     println!(\"[DEBUG] Inside NodeManager::stop async block\");\n\
                     let result = self.stop_internal().await;\n\
                     println!(\"[DEBUG] NodeManager::stop result: {{:?}}\", result);\n\
                     result.map_err(|e| TransportError::Rpc(e.to_string()))\n\
                 }})\n\
             }}\n\
             \n\
             fn rpc_port(&self) -> u16 {{\n\
                 println!(\"[DEBUG] NodeManager::rpc_port called on BitcoinNodeManager\");\n\
                 self.rpc_port\n\
             }}\n\
             \n\
             fn as_any(&self) -> &dyn std::any::Any {{\n\
                 println!(\"[DEBUG] NodeManager::as_any called on BitcoinNodeManager\");\n\
                 self\n\
             }}\n\
         }}\n"
    ).unwrap();
    Ok(())
}

fn emit_impl_block_start(code: &mut String, client_name: &str) -> std::io::Result<()> {
    writeln!(code, "impl {client_name} {{").unwrap();
    Ok(())
}

fn emit_constructors(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    pub async fn new() -> Result<Self, TransportError> {{
        println!(\"[DEBUG] BitcoinTestClient::new called\");
        let config = TestConfig::default();
        let node_manager = BitcoinNodeManager::new_with_config(&config)?;
        Self::new_with_manager(node_manager).await
    }}

    /// Creates a new Bitcoin test client with a specific node manager.
    /// This allows for custom node configuration and lifecycle management.
    /// The node manager must implement the `NodeManager` trait.
    /// ```
    pub async fn new_with_manager<M: NodeManager + 'static>(mut node_manager: M) -> Result<Self, TransportError> {{
        println!(\"[DEBUG] BitcoinTestClient::new_with_manager called\");
        // Start the node
        println!(\"[DEBUG] Calling node_manager.start()\");
        node_manager.start().await?;
        println!(\"[DEBUG] node_manager.start() completed successfully\");
        
        // Wait for node to be ready for RPC
        println!(\"[DEBUG] Creating transport with port {{}}\", node_manager.rpc_port());
        let transport = Arc::new(DefaultTransport::new(\n\
            &format!(\"http://127.0.0.1:{{}}\", node_manager.rpc_port()),
            Some((\"rpcuser\".to_string(), \"rpcpassword\".to_string())),
        ));
        
        // Create RPC client for batching support
        let rpc = RpcClient::from_transport(transport.clone());
        
        // Create node and wallet clients
        let node_client = BitcoinNodeClient::new(transport.clone());
        
        // Wait for node to be ready for RPC
        // Core initialization states that require waiting:
        // -28: RPC in warmup
        // -4:  RPC in warmup (alternative code)
        let init_states = [
            \"\\\"code\\\":-28\",
            \"\\\"code\\\":-4\",
        ];
        
        let max_retries = 30;
        let mut retries = 0;
        
        loop {{
            match node_client.getblockchaininfo().await {{
                Ok(_) => break,
                Err(TransportError::Rpc(e)) => {{
                    // Check if the error matches any known initialization state
                    let is_init_state = init_states.iter().any(|state| e.contains(state));
                    if is_init_state && retries < max_retries {{
                        println!(\"[DEBUG] Waiting for initialization: {{}} (attempt {{}}/{{}})\", e, retries + 1, max_retries);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        retries += 1;
                        continue;
                    }}
                    return Err(TransportError::Rpc(e));
                }}
                Err(e) => return Err(e),
            }}
        }}
        
        if retries > 0 {{
            println!(\"[DEBUG] Node initialization completed after {{}} attempts\", retries);
        }}
        
        Ok(Self {{
            node_client,
            wallet_client: BitcoinWalletClient::new(transport.clone()),
            node_manager: Some(Box::new(node_manager)),
            rpc,
        }})
    }}"
    ).unwrap();
    Ok(())
}

fn emit_wallet_options_struct(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        r#"/// Options for creating or loading a Bitcoin Core wallet
#[derive(Debug, Clone)]
pub struct WalletOptions {{
    pub disable_private_keys: bool,
    pub blank: bool,
    pub passphrase: String,
    pub avoid_reuse: bool,
    pub descriptors: bool,
    pub load_on_startup: bool,
    pub external_signer: bool,
}}

impl Default for WalletOptions {{
    fn default() -> Self {{
        WalletOptions {{
            disable_private_keys: false,
            blank: false,
            passphrase: "".to_string(),
            avoid_reuse: false,
            descriptors: false,
            load_on_startup: false,
            external_signer: false,
        }}
    }}
}}

impl WalletOptions {{
    pub fn with_descriptors(mut self) -> Self {{
        self.descriptors = true;
        self
    }}
}}
"#
    )
    .map_err(std::io::Error::other)?;
    Ok(())
}

fn emit_wallet_methods(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Ensures a wallet exists using the given options.\n\
         /// Loads the wallet if it already exists. Returns the wallet name.\n\
         pub async fn ensure_wallet_with_options(\n\
             &mut self,\n\
             wallet_name: impl Into<String>,\n\
             opts: WalletOptions,\n\
         ) -> Result<String, TransportError> {{\n\
             let wallet_name = wallet_name.into();\n\n\
             // Check if wallet is currently loaded\n\
             let wallets = self.wallet_client.listwallets().await?;\n\
             if wallets.0.iter().any(|w| w == &wallet_name) {{\n\
                 self.wallet_client.unloadwallet(wallet_name.clone(), false).await?;\n\
             }}\n\n\
             // Try to create wallet\n\
             match self.wallet_client\n\
                 .createwallet(\n\
                     wallet_name.clone(),\n\
                     opts.disable_private_keys,\n\
                     opts.blank,\n\
                     opts.passphrase.clone(),\n\
                     opts.avoid_reuse,\n\
                     opts.descriptors,\n\
                     opts.load_on_startup,\n\
                     opts.external_signer,\n\
                 )\n\
                 .await\n\
             {{\n\
                 Ok(_) => Ok(wallet_name),\n\
                 Err(TransportError::Rpc(err)) if err.contains(\"\\\"code\\\":-4\") => {{\n\
                     // Try loading instead\n\
                     self.wallet_client.loadwallet(wallet_name.clone(), false).await?;\n\n\
                     let new_transport = Arc::new(\n\
                         DefaultTransport::new(\n\
                             &format!(\"http://127.0.0.1:{{}}\", self.node_manager.as_ref().unwrap().rpc_port()),\n\
                             Some((\"rpcuser\".to_string(), \"rpcpassword\".to_string())),\n\
                         )\n\
                         .with_wallet(wallet_name.clone())\n\
                     );\n\n\
                     self.wallet_client.with_transport(new_transport.clone());\n\
                     self.node_client.with_transport(new_transport);\n\n\
                     Ok(wallet_name)\n\
                 }},\n\
                 Err(e) => Err(e),\n\
             }}\n\
         }}\n\n\
         /// Shortcut for `ensure_wallet_with_options(\"test_wallet\", WalletOptions::default().with_descriptors())`\n\
         pub async fn ensure_default_wallet(&mut self, name: impl Into<String>) -> Result<String, TransportError> {{\n\
             self.ensure_wallet_with_options(name, WalletOptions::default().with_descriptors()).await\n\
         }}\n"
    ).unwrap();
    Ok(())
}

fn emit_block_mining_helpers(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Helper method to mine blocks to a new address
    pub async fn mine_blocks(&mut self, num_blocks: u64, maxtries: u64) -> Result<(String, Value), TransportError> {{
        // Ensure we have a wallet with default settings
        let _wallet_name = self.ensure_default_wallet(\"test_wallet\").await?;

        println!(\"[debug] Getting new address\");
        let address = self.wallet_client.getnewaddress(\"\".to_string(), \"bech32m\".to_string()).await?;
        println!(\"[debug] Generated address: {{:?}}\", address);
        println!(\"[debug] Generating blocks\");
        let blocks = self.node_client.generatetoaddress(
            num_blocks,
            address.0.clone(),
            maxtries
        ).await?;
        println!(\"[debug] Generated blocks: {{:?}}\", blocks);
        Ok((address.0, serde_json::to_value(blocks)?))
    }}\n"
    ).unwrap();
    Ok(())
}

fn emit_reset_chain(code: &mut String) -> std::io::Result<()> {
    let block_hash_type = TYPE_REGISTRY
        .map_argument_type(&ApiArgument {
            type_: "hex".to_string(),
            names: vec!["blockhash".to_string()],
            optional: false,
            description: String::new(),
        })
        .0;
    writeln!(
        code,
        "    /// Resets the blockchain to a clean state.\n\
         /// This method:\n\
         /// 1. First attempts to prune the blockchain to height 0\n\
         /// 2. If blocks remain, invalidates all blocks except genesis\n\
         /// 3. Reconsiders the genesis block to maintain a valid chain\n\
         pub async fn reset_chain(&mut self) -> Result<(), TransportError> {{\n\
             // First try pruning to height 0\n\
             self.node_client.pruneblockchain(0).await?;\n\
             // Check if we still have blocks\n\
             let info = self.node_client.getblockchaininfo().await?;\n\
             let current_height = info.blocks;\n\
             if current_height > 1 {{\n\
                 // Invalidate all blocks except genesis\n\
                 for height in (1..=current_height).rev() {{\n\
                     let hash_str = self.node_client.getblockhash(height).await?.0;\n\
                     let block_hash = {block_hash_type}::from_str(&hash_str).map_err(|e| TransportError::Rpc(format!(\"Failed to parse block hash: {{}}\", e)))?;\n\
                     self.node_client.invalidateblock(block_hash).await?;\n\
                 }}\n\
                 // Reconsider genesis block\n\
                 let genesis_hash = {block_hash_type}::from_str(&self.node_client.getblockhash(0).await?.0).map_err(|e| TransportError::Rpc(format!(\"Failed to parse block hash: {{}}\", e)))?;\n\
                 self.node_client.reconsiderblock(genesis_hash).await?;\n\
             }}\n\
             Ok(())\n\
         }}\n"
    ).map_err(std::io::Error::other)
}

fn emit_stop_node(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Stops the Bitcoin node if one is running.\n\
         /// This is automatically called when the client is dropped.\n\
         pub async fn stop_node(&mut self) -> Result<(), TransportError> {{\n\
             if let Some(mut manager) = self.node_manager.take() {{\n\
                 manager.stop().await?;\n\
             }}\n\
             Ok(())\n\
         }}\n"
    )
    .unwrap();
    Ok(())
}

fn emit_node_manager_accessor(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Returns a reference to the node manager if one exists.\n\
         /// This can be used to access node configuration and control the node lifecycle.\n\
         pub fn node_manager(&self) -> Option<&dyn NodeManager> {{\n\
         self.node_manager.as_deref()\n\
         }}\n"
    )
    .unwrap();
    Ok(())
}

fn emit_rpc_accessor(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Give callers the full RPC client (incl. `.batch()`)\n\
         pub fn rpc(&self) -> &RpcClient {{\n\
             &self.rpc\n\
         }}\n"
    )
    .unwrap();
    Ok(())
}

fn emit_batch_method(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Begin a JSON-RPC batch against this test node\n    pub fn batch(&self) -> BatchBuilder {{\n        self.rpc.batch()\n    }}\n"
    ).unwrap();
    Ok(())
}

fn emit_delegated_rpc_methods(code: &mut String, methods: &[ApiMethod]) -> std::io::Result<()> {
    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let doc_comment = doc_comment::format_doc_comment(&m.description);
        let target = if WALLET_METHODS.contains(&m.name.as_str()) {
            "wallet_client"
        } else {
            "node_client"
        };

        // Get the specific return type for this method
        let ret_ty = if m.results.is_empty() || m.results[0].type_.to_lowercase() == "none" {
            "()".to_string()
        } else {
            format!("{}Response", camel(&m.name))
        };

        let (param_list, args) = if m.arguments.is_empty() {
            (String::new(), String::new())
        } else {
            let param_list = m
                .arguments
                .iter()
                .map(|arg| {
                    let name = if arg.names[0] == "type" {
                        "_type"
                    } else {
                        &camel_to_snake_case(&arg.names[0])
                    };
                    let ty = rust_type_for(&arg.names[0], &arg.type_);
                    format!("{name}: {ty}")
                })
                .collect::<Vec<_>>()
                .join(", ");

            let args = m
                .arguments
                .iter()
                .map(|arg| {
                    if arg.names[0] == "type" {
                        "_type".to_string()
                    } else {
                        camel_to_snake_case(&arg.names[0])
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            (param_list, args)
        };

        writeln!(
            code,
            "{}\n    pub async fn {}(&self{}{}) -> Result<{}, TransportError> {{\n        self.{}.{}({}).await\n    }}\n",
            doc_comment,
            method_snake,
            if param_list.is_empty() { "" } else { ", " },
            param_list,
            ret_ty,
            target,
            method_snake,
            args
        ).unwrap();
    }
    Ok(())
}

fn emit_send_to_address_helpers(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "/// Helper method to send bitcoin to an address with either a confirmation target or fee rate.\n\
         /// This is a more ergonomic wrapper around sendtoaddress that prevents specifying both conf_target and fee_rate.\n\
         /// \n\
         /// Parameters:\n\
         /// - address: The destination address\n\
         /// - amount: The amount to send\n\
         /// - conf_target: The confirmation target in blocks\n\
         /// - estimate_mode: The fee estimate mode (\"economical\" or \"conservative\")\n\
         /// ```\n\
         \n\
         pub async fn send_to_address_with_conf_target(\n\
         &self,\n\
         address: String,\n\
         amount: Amount,\n\
         conf_target: u64,\n\
         estimate_mode: String,\n\
     ) -> Result<Value, TransportError> {{\n\
         Ok(serde_json::to_value(self.wallet_client.sendtoaddress(\n\
             address,\n\
             serde_json::to_value(amount.to_btc().to_string())?,\n\
             \"\".to_string(),\n\
             \"\".to_string(),\n\
             false,\n\
             true,\n\
             conf_target,\n\
             estimate_mode,\n\
             false,\n\
             serde_json::Value::Null,\n\
             false,\n\
         ).await?)?)\n\
     }}\n\
     \n\
     pub async fn send_to_address_with_fee_rate(\n\
     &self,\n\
     address: String,\n\
     amount: Amount,\n\
     fee_rate: Amount,\n\
 ) -> Result<Value, TransportError> {{\n\
     Ok(serde_json::to_value(self.wallet_client.sendtoaddress(\n\
         address,\n\
         serde_json::to_value(amount.to_btc().to_string())?,\n\
         \"\".to_string(),\n\
         \"\".to_string(),\n\
         false,\n\
         true,\n\
         0u64,\n\
         \"unset\".to_string(),\n\
         false,\n\
         serde_json::to_value(fee_rate.to_btc().to_string())?,\n\
         false,\n\
     ).await?)?)\n\
 }}\n"
    ).unwrap();
    Ok(())
}

fn emit_impl_block_end(code: &mut String) -> std::io::Result<()> {
    writeln!(code, "}}\n").unwrap();
    Ok(())
}

fn emit_drop_impl(code: &mut String, client_name: &str) -> std::io::Result<()> {
    writeln!(code, "impl Drop for {client_name} {{").unwrap();
    writeln!(code, "    fn drop(&mut self) {{").unwrap();
    writeln!(code, "        let _ = self.node_manager.take();").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}\n").unwrap();
    Ok(())
}
