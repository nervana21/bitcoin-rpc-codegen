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
      → ✅ Add all ergonomic helpers here (e.g., `reset_chain()`, `mine_blocks()`, etc.)
  - `utils.rs`: Utility functions (e.g. `camel()`, `rust_type_for()`)

Benefits:
- Better maintainability
- Easier to test and extend
- Faster onboarding for contributors
*/

use crate::utils::camel_to_snake_case;
use crate::wallet_methods::WALLET_METHODS;
use crate::{generators::doc_comment, CodeGenerator, TYPE_REGISTRY};
use rpc_api::ApiMethod;
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
pub struct TestNodeGenerator;

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

        let wallet_code = generate_subclient("BitcoinWalletClient", &wallet_methods).unwrap();
        let node_code = generate_subclient("BitcoinNodeClient", &node_methods).unwrap();
        let combined_code = generate_combined_client("BitcoinTestClient", methods).unwrap();

        let mod_rs_code = generate_mod_rs();

        vec![
            ("wallet.rs".to_string(), wallet_code),
            ("node.rs".to_string(), node_code),
            ("test_node.rs".to_string(), combined_code),
            ("params.rs".to_string(), params_code),
            ("result.rs".to_string(), result_code),
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
            writeln!(code, "    pub {}: {},", field, ty).unwrap();
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
            "#[derive(Debug, Deserialize)]\npub struct {}Result(pub {});\n",
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
        "//! Test node module for Bitcoin RPC testing\n\
         pub mod params;\n\
         pub mod result;\n\
         pub mod wallet;\n\
         pub mod node;\n\
         pub use test_node::test_node::BitcoinTestClient;\n\
         pub use wallet::BitcoinWalletClient;\n\
         pub use node::BitcoinNodeClient;"
    )
    .unwrap();
    code
}

fn rust_type_for(param_name: &str, api_ty: &str) -> String {
    let (base_ty, is_option) = TYPE_REGISTRY.map_type(api_ty, param_name);
    if is_option {
        format!("Option<{}>", base_ty)
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

fn generate_subclient(client_name: &str, methods: &[ApiMethod]) -> std::io::Result<String> {
    let mut code = String::new();
    writeln!(
        code,
        "use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use crate::transport::core::TransportExt;
use crate::transport::{{DefaultTransport, TransportError}};

#[derive(Debug, Clone)]
pub struct {} {{
    client: Arc<DefaultTransport>,
}}

impl {} {{
    pub fn new(client: Arc<DefaultTransport>) -> Self {{
        Self {{ client }}
    }}

    pub fn with_transport(&mut self, client: Arc<DefaultTransport>) {{
        self.client = client;
    }}",
        client_name, client_name
    )
    .unwrap();
    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let ret_ty = "Value";
        writeln!(
            code,
            "\n{}",
            doc_comment::format_doc_comment(&m.description)
        )
        .unwrap();

        if m.arguments.is_empty() {
            writeln!(
                code,
                "    pub async fn {}(&self) -> Result<{}, TransportError> {{
        Ok(self.client.call::<{}>(\"{}\", &[]).await?.into())
    }}",
                method_snake, ret_ty, ret_ty, m.name
            )
            .unwrap();
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
                    let ty = if arg.names[0] == "fee_rate" {
                        "Option<bitcoin::Amount>"
                    } else {
                        &rust_type_for(&arg.names[0], &arg.type_)
                    };
                    format!("{}: {}", name, ty)
                })
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(
                code,
                "    pub async fn {}(&self, {}) -> Result<{}, TransportError> {{
        let mut vec = vec![];",
                method_snake, param_list, ret_ty
            )
            .unwrap();

            for arg in &m.arguments {
                let name = if arg.names[0] == "type" {
                    "_type"
                } else {
                    &camel_to_snake_case(&arg.names[0])
                };
                if arg.names[0] == "fee_rate" {
                    writeln!(
                        code,
                        "        vec.push(match {} {{ Some(v) => serde_json::to_value(v.to_btc())?, None => serde_json::Value::Null }});",
                        name
                    ).unwrap();
                } else {
                    writeln!(code, "        vec.push(serde_json::to_value({})?);", name).unwrap();
                }
            }
            writeln!(
                code,
                "        Ok(self.client.call::<{}>(\"{}\", &vec).await?.into())
    }}",
                ret_ty, m.name
            )
            .unwrap();
        }
    }
    writeln!(code, "}}\n").unwrap();
    Ok(code)
}

fn generate_combined_client(client_name: &str, methods: &[ApiMethod]) -> std::io::Result<String> {
    let mut code = String::new();

    emit_imports(&mut code).unwrap();
    emit_node_manager_trait(&mut code).unwrap();
    emit_struct_definition(&mut code, client_name).unwrap();
    emit_node_manager_impl(&mut code).unwrap();
    emit_impl_block_start(&mut code, client_name).unwrap();
    emit_constructors(&mut code).unwrap();
    emit_wallet_methods(&mut code).unwrap();
    emit_block_mining_helpers(&mut code).unwrap();
    emit_reset_chain(&mut code).unwrap();
    emit_stop_node(&mut code).unwrap();
    emit_node_manager_accessor(&mut code).unwrap();
    emit_delegated_rpc_methods(&mut code, methods).unwrap();
    emit_send_to_address_helpers(&mut code).unwrap();
    emit_impl_block_end(&mut code).unwrap();
    emit_drop_impl(&mut code, client_name).unwrap();

    Ok(code)
}

fn emit_imports(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;
use crate::transport::core::TransportExt;
use crate::transport::{{DefaultTransport, TransportError}};
use async_trait::async_trait;

use crate::node::{{BitcoinNodeManager, TestConfig}};

use super::node::BitcoinNodeClient;
use super::wallet::BitcoinWalletClient;

use std::str::FromStr;
use bitcoin::Amount;
"
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
         pub struct {} {{\n\
             node_client: BitcoinNodeClient,\n\
             wallet_client: BitcoinWalletClient,\n\
             node_manager: Option<Box<dyn NodeManager>>,\n\
         }}\n",
        client_name
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
    writeln!(code, "impl {} {{", client_name).unwrap();
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
        let client = Arc::new(DefaultTransport::new(
            &format!(\"http://127.0.0.1:{{}}\", node_manager.rpc_port()),
            Some((\"rpcuser\".to_string(), \"rpcpassword\".to_string())),
        ));
        
        // Create node and wallet clients
        let node_client = BitcoinNodeClient::new(client.clone());
        let wallet_client = BitcoinWalletClient::new(client.clone());
        
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
            wallet_client: BitcoinWalletClient::new(client.clone()),
            node_manager: Some(Box::new(node_manager)),
        }})
    }}"
    ).unwrap();
    Ok(())
}

fn emit_wallet_methods(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Ensures a wallet exists with the given name and parameters.\n\
         /// If the wallet already exists, it will be unloaded and recreated with the new parameters.\n\
         /// If the wallet doesn't exist, it will be created.\n\
         /// Returns the wallet name that was created/ensured.\n\
         pub async fn ensure_wallet(\n\
             &mut self,\n\
             wallet_name: Option<String>,\n\
             disable_private_keys: bool,\n\
             blank: bool,\n\
             passphrase: String,\n\
             avoid_reuse: bool,\n\
             descriptors: bool,\n\
             load_on_startup: bool,\n\
             external_signer: bool,\n\
         ) -> Result<String, TransportError> {{\n\
             let wallet_name = wallet_name.unwrap_or_else(|| \"default\".to_string());\n\
             \n\
             // Check if wallet exists\n\
             let wallets = self.wallet_client.listwallets().await?;\n\
             if wallets.as_array().map_or(false, |w| w.contains(&wallet_name.clone().into())) {{\n\
                 // Unload existing wallet\n\
                 self.wallet_client.unloadwallet(wallet_name.clone(), false).await?;\n\
             }}\n\
             \n\
             // Create the wallet\n\
             match self.wallet_client.createwallet(\n\
                 wallet_name.clone(),\n\
                 disable_private_keys,\n\
                 blank,\n\
                 passphrase,\n\
                 avoid_reuse,\n\
                 descriptors,\n\
                 load_on_startup,\n\
                 external_signer,\n\
             ).await {{\n\
                 Ok(_) => Ok(wallet_name),\n\
                 Err(TransportError::Rpc(err)) if err.contains(\"\\\"code\\\":-4\") => {{\n\
                     // If the wallet database already exists, try to load it instead\n\
                     self.wallet_client.loadwallet(wallet_name.clone(), false).await?;\n\
                     \n\
                     // Update both clients' transports to use this wallet\n\
                     let new_transport = Arc::new(DefaultTransport::new(\n\
                         &format!(\"http://127.0.0.1:{{}}\", self.node_manager.as_ref().unwrap().rpc_port()),\n\
                         Some((\"rpcuser\".to_string(), \"rpcpassword\".to_string())),\n\
                     ).with_wallet(wallet_name.clone()));\n\
                     \n\
                     self.wallet_client.with_transport(new_transport.clone());\n
                     self.node_client.with_transport(new_transport);\n
                     Ok(wallet_name)\n\
                 }},\n\
                 Err(e) => Err(e),\n\
             }}\n\
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
        let _wallet_name = self.ensure_wallet(
            Some(\"test_wallet\".to_string()),  // Use specific wallet name
            false, // enable private keys
            false, // not blank
            \"\".to_string(), // no passphrase
            false, // don't avoid reuse
            true,  // use descriptors
            false, // don't load on startup
            false, // no external signer
        ).await?;

        println!(\"[debug] Getting new address\");
        let address_value = self.wallet_client.getnewaddress(\"\".to_string(), \"bech32m\".to_string()).await?;
        println!(\"[debug] Address value: {{:?}}\", address_value);
        let address = address_value.as_str().ok_or_else(|| TransportError::Rpc(\"Expected string address\".into()))?.to_string();
        println!(\"[debug] Generated address: {{}}\", address);
        println!(\"[debug] Generating blocks\");
        let blocks = self.node_client.generatetoaddress(num_blocks, address.clone(), maxtries).await?;
        println!(\"[debug] Generated blocks: {{:?}}\", blocks);
        Ok((address, blocks))
    }}\n"
    ).unwrap();
    Ok(())
}

fn emit_reset_chain(code: &mut String) -> std::io::Result<()> {
    let block_hash_type = TYPE_REGISTRY.map_type("hex", "blockhash").0;
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
             let current_height = info[\"blocks\"].as_u64().unwrap_or(0);\n\
             if current_height > 1 {{\n\
                 // Invalidate all blocks except genesis\n\
                 for height in (1..=current_height).rev() {{\n\
                     let block_hash = {block_hash_type}::from_str(self.node_client.getblockhash(height).await?.as_str().unwrap()).map_err(|e| TransportError::Rpc(format!(\"Failed to parse block hash: {{}}\", e)))?;\n\
                     self.node_client.invalidateblock(block_hash).await?;\n\
                 }}\n\
                 // Reconsider genesis block\n\
                 let genesis_hash = {block_hash_type}::from_str(self.node_client.getblockhash(0).await?.as_str().unwrap()).map_err(|e| TransportError::Rpc(format!(\"Failed to parse block hash: {{}}\", e)))?;\n\
                 self.node_client.reconsiderblock(genesis_hash).await?;\n\
             }}\n\
             Ok(())\n\
         }}\n"
    ).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
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

fn emit_delegated_rpc_methods(code: &mut String, methods: &[ApiMethod]) -> std::io::Result<()> {
    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let ret_ty = "Value".to_string();
        let doc_comment = doc_comment::format_doc_comment(&m.description);
        let target = if WALLET_METHODS.contains(&m.name.as_str()) {
            "wallet_client"
        } else {
            "node_client"
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
                    format!("{}: {}", name, ty)
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
         self.wallet_client.sendtoaddress(\n\
             address,\n\
             amount,\n\
             \"\".to_string(),\n\
             \"\".to_string(),\n\
             false,\n\
             true,\n\
             conf_target,\n\
             estimate_mode,\n\
             false,\n\
             None, // Changed from Amount::ZERO to None\n\
             false,\n\
         ).await\n\
     }}\n\
     \n\
     pub async fn send_to_address_with_fee_rate(\n\
     &self,\n\
     address: String,\n\
     amount: Amount,\n\
     fee_rate: Amount,\n\
 ) -> Result<Value, TransportError> {{\n\
     self.wallet_client.sendtoaddress(\n\
         address,\n\
         amount,\n\
         \"\".to_string(),\n\
         \"\".to_string(),\n\
         false,\n\
         true,\n\
         0u64,\n\
         \"unset\".to_string(),\n\
         false,\n\
         Some(fee_rate), // Changed to wrap fee_rate in Some()\n\
         false,\n\
     ).await\n\
 }}\n"
    ).unwrap();
    Ok(())
}

fn emit_impl_block_end(code: &mut String) -> std::io::Result<()> {
    writeln!(code, "}}\n").unwrap();
    Ok(())
}

fn emit_drop_impl(code: &mut String, client_name: &str) -> std::io::Result<()> {
    writeln!(code, "impl Drop for {} {{", client_name).unwrap();
    writeln!(code, "    fn drop(&mut self) {{").unwrap();
    writeln!(code, "        let _ = self.node_manager.take();").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}\n").unwrap();
    Ok(())
}
