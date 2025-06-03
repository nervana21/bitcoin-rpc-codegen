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

/// Generates typed Rust clients for interacting with a Bitcoin Core node in test environments.
///
/// `TestNodeGenerator` produces thin wrappers around RPC methods exposed by Bitcoin Core,
/// generating type-safe parameter structs (`params.rs`), result wrappers (`result.rs`),
/// and high-level clients (`BitcoinTestClient`, `BitcoinNodeClient`, and `BitcoinWalletClient`)
/// for use in integration tests or test harnesses.
///
/// This generator assumes an API schema input and emits idiomatic Rust code, enabling
/// easy and ergonomic testing of RPC interfaces without hand-rolled serialization logic.
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
        let doc_comment = doc_comment::format_doc_comment(&m.description);
        for line in doc_comment.lines() {
            writeln!(code, "/// {}", line).unwrap();
        }
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
    writeln!(code, "//! Test node module for Bitcoin RPC testing").unwrap();
    writeln!(code, "pub mod params;").unwrap();
    writeln!(code, "pub mod result;").unwrap();
    writeln!(code, "pub mod wallet;").unwrap();
    writeln!(code, "pub mod node;").unwrap();
    writeln!(code, "pub use test_node::test_node::BitcoinTestClient;").unwrap();
    writeln!(code, "pub use wallet::BitcoinWalletClient;").unwrap();
    writeln!(code, "pub use node::BitcoinNodeClient;").unwrap();
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
        "use anyhow::Result;\nuse serde_json::Value;\nuse std::sync::Arc;"
    )
    .unwrap();
    writeln!(code, "use crate::transport::core::TransportExt;").unwrap();
    writeln!(
        code,
        "use crate::transport::{{DefaultTransport, TransportError}};\n"
    )
    .unwrap();

    writeln!(
        code,
        "#[derive(Debug, Clone)]\npub struct {} {{\n    client: Arc<DefaultTransport>,\n}}\n",
        client_name
    )
    .unwrap();

    writeln!(code, "impl {} {{\n    pub fn new(client: Arc<DefaultTransport>) -> Self {{\n        Self {{ client }}\n    }}\n", client_name).unwrap();

    writeln!(code, "    pub fn with_transport(&mut self, client: Arc<DefaultTransport>) {{\n        self.client = client;\n    }}\n").unwrap();

    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let ret_ty = "Value";
        let doc_comment = doc_comment::format_doc_comment(&m.description);
        for line in doc_comment.lines() {
            writeln!(code, "    {}", line).unwrap();
        }

        if m.arguments.is_empty() {
            writeln!(code, "    pub async fn {}(&self) -> Result<{}, TransportError> {{\n        Ok(self.client.call::<{}>(\"{}\", &[]).await?.into())\n    }}\n", method_snake, ret_ty, ret_ty, m.name).unwrap();
        } else {
            let param_list = m
                .arguments
                .iter()
                .map(|arg| {
                    let name = if arg.names[0] == "type" {
                        "_type".to_string()
                    } else {
                        camel_to_snake_case(&arg.names[0])
                    };
                    let ty = if arg.names[0] == "fee_rate" {
                        "Option<bitcoin::Amount>".to_string()
                    } else {
                        rust_type_for(&arg.names[0], &arg.type_)
                    };
                    format!("{}: {}", name, ty)
                })
                .collect::<Vec<_>>()
                .join(", ");

            writeln!(
                code,
                "    pub async fn {}(&self, {}) -> Result<{}, TransportError> {{",
                method_snake, param_list, ret_ty
            )
            .unwrap();
            writeln!(code, "        let mut vec = vec![];").unwrap();

            for arg in &m.arguments {
                let name = if arg.names[0] == "type" {
                    "_type"
                } else {
                    &camel_to_snake_case(&arg.names[0])
                };
                if arg.names[0] == "fee_rate" {
                    writeln!(code, "        vec.push(match {} {{ Some(v) => serde_json::to_value(v.to_btc())?, None => serde_json::Value::Null }});", name).unwrap();
                } else {
                    writeln!(code, "        vec.push(serde_json::to_value({})?);", name).unwrap();
                }
            }
            writeln!(
                code,
                "        Ok(self.client.call::<{}>(\"{}\", &vec).await?.into())\n    }}\n",
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
    writeln!(code, "use anyhow::Result;").unwrap();
    writeln!(code, "use serde_json::Value;").unwrap();
    writeln!(code, "use std::sync::Arc;").unwrap();
    writeln!(code, "use crate::transport::core::TransportExt;").unwrap();
    writeln!(
        code,
        "use crate::transport::{{DefaultTransport, TransportError}};\n"
    )
    .unwrap();
    writeln!(
        code,
        "use crate::node::{{BitcoinNodeManager, TestConfig}};\n"
    )
    .unwrap();
    writeln!(code, "use super::node::BitcoinNodeClient;").unwrap();
    writeln!(code, "use super::wallet::BitcoinWalletClient;\n").unwrap();
    writeln!(code, "use std::str::FromStr;").unwrap();
    writeln!(code, "use bitcoin::Amount;\n").unwrap();
    Ok(())
}

fn emit_node_manager_trait(code: &mut String) -> std::io::Result<()> {
    writeln!(code, "/// Trait for managing a Bitcoin node's lifecycle").unwrap();
    writeln!(
        code,
        "pub trait NodeManager: Send + Sync + std::fmt::Debug + std::any::Any {{"
    )
    .unwrap();
    writeln!(code, "    fn start(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;").unwrap();
    writeln!(code, "    fn stop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;").unwrap();
    writeln!(code, "    fn rpc_port(&self) -> u16;").unwrap();
    writeln!(code, "    fn as_any(&self) -> &dyn std::any::Any;").unwrap();
    writeln!(code, "}}\n").unwrap();
    Ok(())
}

fn emit_struct_definition(code: &mut String, client_name: &str) -> std::io::Result<()> {
    writeln!(code, "#[derive(Debug)]").unwrap();
    writeln!(code, "pub struct {} {{", client_name).unwrap();
    writeln!(code, "    node_client: BitcoinNodeClient,").unwrap();
    writeln!(code, "    wallet_client: BitcoinWalletClient,").unwrap();
    writeln!(code, "    node_manager: Option<Box<dyn NodeManager>>,").unwrap();
    writeln!(code, "}}\n").unwrap();
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
        "    pub async fn new() -> Result<Self, TransportError> {{\n\
         println!(\"[DEBUG] BitcoinTestClient::new called\");\n\
         let config = TestConfig::default();\n\
         let node_manager = BitcoinNodeManager::new_with_config(&config)?;\n\
         Self::new_with_manager(node_manager).await\n\
     }}\n"
    )
    .unwrap();

    // Add new_with_manager constructor
    writeln!(
        code,
        "/// Creates a new Bitcoin test client with a specific node manager.\n\
         /// This allows for custom node configuration and lifecycle management.\n\
         /// The node manager must implement the `NodeManager` trait.\n\
         /// ```\n"
    )
    .unwrap();
    writeln!(
        code,
        "    pub async fn new_with_manager<M: NodeManager + 'static>(mut node_manager: M) -> Result<Self, TransportError> {{\n\
         println!(\"[DEBUG] BitcoinTestClient::new_with_manager called\");\n\
         // Start the node\n\
         println!(\"[DEBUG] Calling node_manager.start()\");\n\
         node_manager.start().await?;\n\
         println!(\"[DEBUG] node_manager.start() completed successfully\");\n\
         \n\
         // Wait for node to be ready for RPC\n\
         println!(\"[DEBUG] Creating transport with port {{}}\", node_manager.rpc_port());\n\
         let client = Arc::new(DefaultTransport::new(\n\
             &format!(\"http://127.0.0.1:{{}}\", node_manager.rpc_port()),\n\
             Some((\"rpcuser\".to_string(), \"rpcpassword\".to_string())),\n\
         ));\n\
         let node_client = BitcoinNodeClient::new(client.clone());\n\
         \n\
         // Core initialization states that require waiting\n\
         // -28: RPC in warmup\n\
         // -4:  RPC in warmup (alternative code)\n\
         let init_states = [\n\
             \"\\\"code\\\":-28\",\n\
             \"\\\"code\\\":-4\",\n\
         ];\n\
         \n\
         let max_retries = 60; // Increased from 30 to 60 for slower systems\n\
         let mut retries = 0;\n\
         \n\
         loop {{\n\
             match node_client.getblockchaininfo().await {{\n\
                 Ok(_) => break,\n\
                 Err(TransportError::Rpc(e)) => {{\n\
                     // Check if the error matches any known initialization state\n\
                     let is_init_state = init_states.iter().any(|state| e.contains(state));\n\
                     if is_init_state && retries < max_retries {{\n\
                         println!(\"[DEBUG] Waiting for initialization: {{}} (attempt {{}}/{{}})\", e, retries + 1, max_retries);\n\
                         tokio::time::sleep(std::time::Duration::from_secs(1)).await;\n\
                         retries += 1;\n\
                         continue;\n\
                     }}\n\
                     return Err(TransportError::Rpc(e));\n\
                 }}\n\
                 Err(e) => return Err(e),\n\
             }}\n\
         }}\n\
         \n\
         if retries > 0 {{\n\
             println!(\"[DEBUG] Node initialization completed after {{}} attempts\", retries);\n\
         }}\n\
         \n\
         Ok(Self {{\n\
             node_client,\n\
             wallet_client: BitcoinWalletClient::new(client),\n\
             node_manager: Some(Box::new(node_manager)),\n\
         }})\n\
     }}\n"
    )
    .unwrap();
    Ok(())
}

fn emit_wallet_methods(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Ensures a wallet exists with the given name and parameters.\n\
         /// If the wallet already exists, it will be unloaded and recreated with the new parameters.\n\
         /// If the wallet doesn't exist, it will be created.\n\
         /// Returns the wallet name that was created/ensured."
    )
    .unwrap();
    writeln!(
        code,
        "    pub async fn ensure_wallet(\n\
         &mut self,\n\
         wallet_name: Option<String>,\n\
         disable_private_keys: bool,\n\
         blank: bool,\n\
         passphrase: String,\n\
         avoid_reuse: bool,\n\
         descriptors: bool,\n\
         load_on_startup: bool,\n\
         external_signer: bool,\n\
         ) -> Result<String, TransportError> {{"
    )
    .unwrap();
    writeln!(
        code,
        "        let wallet_name = wallet_name.unwrap_or_else(|| \"default\".to_string());"
    )
    .unwrap();
    writeln!(code, "        ").unwrap();
    writeln!(code, "        // Check if wallet exists").unwrap();
    writeln!(
        code,
        "        let wallets = self.wallet_client.listwallets().await?;"
    )
    .unwrap();
    writeln!(code, "        if wallets.as_array().map_or(false, |w| w.contains(&wallet_name.clone().into())) {{").unwrap();
    writeln!(code, "            // Unload existing wallet").unwrap();
    writeln!(
        code,
        "            self.wallet_client.unloadwallet(wallet_name.clone(), false).await?;"
    )
    .unwrap();
    writeln!(code, "        }}").unwrap();
    writeln!(code, "        ").unwrap();
    writeln!(code, "        // Create the wallet").unwrap();
    writeln!(code, "        match self.wallet_client.createwallet(").unwrap();
    writeln!(code, "            wallet_name.clone(),").unwrap();
    writeln!(code, "            disable_private_keys,").unwrap();
    writeln!(code, "            blank,").unwrap();
    writeln!(code, "            passphrase,").unwrap();
    writeln!(code, "            avoid_reuse,").unwrap();
    writeln!(code, "            descriptors,").unwrap();
    writeln!(code, "            load_on_startup,").unwrap();
    writeln!(code, "            external_signer,").unwrap();
    writeln!(code, "        ).await {{").unwrap();
    writeln!(code, "            Ok(_) => Ok(wallet_name),").unwrap();
    writeln!(
        code,
        "            Err(TransportError::Rpc(err)) if err.contains(\"\\\"code\\\":-4\") => {{"
    )
    .unwrap();
    writeln!(
        code,
        "                // If the wallet database already exists, try to load it instead"
    )
    .unwrap();
    writeln!(
        code,
        "                self.wallet_client.loadwallet(wallet_name.clone(), false).await?;"
    )
    .unwrap();
    writeln!(
        code,
        "                // Update both clients' transports to use this wallet"
    )
    .unwrap();
    writeln!(
        code,
        "                let new_transport = Arc::new(DefaultTransport::new("
    )
    .unwrap();
    writeln!(
        code,
        "                    &format!(\"http://127.0.0.1:{{}}\", self.node_manager.as_ref().unwrap().rpc_port()),"
    )
    .unwrap();
    writeln!(
        code,
        "                    Some((\"rpcuser\".to_string(), \"rpcpassword\".to_string())),"
    )
    .unwrap();
    writeln!(code, "                ).with_wallet(wallet_name.clone()));").unwrap();
    writeln!(
        code,
        "                self.wallet_client.with_transport(new_transport.clone());"
    )
    .unwrap();
    writeln!(
        code,
        "                self.node_client.with_transport(new_transport);"
    )
    .unwrap();
    writeln!(code, "                Ok(wallet_name)").unwrap();
    writeln!(code, "            }},").unwrap();
    writeln!(code, "            Err(e) => Err(e),").unwrap();
    writeln!(code, "        }}").unwrap();
    writeln!(code, "    }}\n").unwrap();
    Ok(())
}

fn emit_block_mining_helpers(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Helper method to mine blocks to a new address"
    )
    .unwrap();
    writeln!(code, "    pub async fn mine_blocks(&mut self, num_blocks: u64, maxtries: u64) -> Result<(String, Value), TransportError> {{").unwrap();
    writeln!(
        code,
        "        // Ensure we have a wallet with default settings"
    )
    .unwrap();
    writeln!(code, "        let _wallet_name = self.ensure_wallet(").unwrap();
    writeln!(
        code,
        "            Some(\"test_wallet\".to_string()),  // Use specific wallet name"
    )
    .unwrap();
    writeln!(code, "            false, // enable private keys").unwrap();
    writeln!(code, "            false, // not blank").unwrap();
    writeln!(code, "            \"\".to_string(), // no passphrase").unwrap();
    writeln!(code, "            false, // don't avoid reuse").unwrap();
    writeln!(code, "            true,  // use descriptors").unwrap();
    writeln!(code, "            false, // don't load on startup").unwrap();
    writeln!(code, "            false, // no external signer").unwrap();
    writeln!(code, "        ).await?;").unwrap();
    writeln!(code, "        ").unwrap();
    writeln!(code, "        println!(\"[debug] Getting new address\");").unwrap();
    writeln!(code, "        let address_value = self.wallet_client.getnewaddress(\"\".to_string(), \"bech32m\".to_string()).await?;").unwrap();
    writeln!(
        code,
        "        println!(\"[debug] Address value: {{:?}}\", address_value);"
    )
    .unwrap();
    writeln!(code, "        let address = address_value.as_str().ok_or_else(|| TransportError::Rpc(\"Expected string address\".into()))?.to_string();").unwrap();
    writeln!(
        code,
        "        println!(\"[debug] Generated address: {{}}\", address);"
    )
    .unwrap();
    writeln!(code, "        println!(\"[debug] Generating blocks\");").unwrap();
    writeln!(code, "        let blocks = self.node_client.generatetoaddress(num_blocks, address.clone(), maxtries).await?;").unwrap();
    writeln!(
        code,
        "        println!(\"[debug] Generated blocks: {{:?}}\", blocks);"
    )
    .unwrap();
    writeln!(code, "        Ok((address, blocks))").unwrap();
    writeln!(code, "    }}\n").unwrap();
    Ok(())
}

fn emit_reset_chain(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Resets the blockchain to a clean state.\n\
         /// This method:\n\
         /// 1. First attempts to prune the blockchain to height 0\n\
         /// 2. If blocks remain, invalidates all blocks except genesis\n\
         /// 3. Reconsiders the genesis block to maintain a valid chain\n\
         /// # Example\n\
         /// ```no_run\n\
         /// use bitcoin_rpc_midas::test_node::test_node::BitcoinTestClient;\n\
         /// #[tokio::main]\n\
         /// async fn main() -> anyhow::Result<()> {{\n\
         ///     let mut client = BitcoinTestClient::new().await?;\n\
         ///     client.reset_chain().await?;\n\
         ///     // Now you have a clean chain state\n\
         ///     Ok(())\n\
         /// }}\n\
         /// ```"
    )
    .unwrap();
    writeln!(
        code,
        "    pub async fn reset_chain(&mut self) -> Result<(), TransportError> {{"
    )
    .unwrap();
    writeln!(code, "        // First try pruning to height 0").unwrap();
    writeln!(code, "        self.node_client.pruneblockchain(0).await?;").unwrap();
    writeln!(code, "        // Check if we still have blocks").unwrap();
    writeln!(
        code,
        "        let info = self.node_client.getblockchaininfo().await?;"
    )
    .unwrap();
    writeln!(
        code,
        "        let current_height = info[\"blocks\"].as_u64().unwrap_or(0);"
    )
    .unwrap();
    writeln!(code, "        if current_height > 1 {{").unwrap();
    writeln!(code, "            // Invalidate all blocks except genesis").unwrap();
    writeln!(
        code,
        "            for height in (1..=current_height).rev() {{"
    )
    .unwrap();
    let block_hash_type = TYPE_REGISTRY.map_type("hex", "blockhash").0;
    writeln!(
        code,
        "                let block_hash = {block_hash_type}::from_str(self.node_client.getblockhash(height).await?.as_str().unwrap()).map_err(|e| TransportError::Rpc(format!(\"Failed to parse block hash: {{}}\", e)))?;"
    )
    .unwrap();
    writeln!(
        code,
        "                self.node_client.invalidateblock(block_hash).await?;"
    )
    .unwrap();
    writeln!(code, "            }}").unwrap();
    writeln!(code, "            // Reconsider genesis block").unwrap();
    writeln!(
        code,
        "            let genesis_hash = {block_hash_type}::from_str(self.node_client.getblockhash(0).await?.as_str().unwrap()).map_err(|e| TransportError::Rpc(format!(\"Failed to parse block hash: {{}}\", e)))?;"
    )
    .unwrap();
    writeln!(
        code,
        "            self.node_client.reconsiderblock(genesis_hash).await?;"
    )
    .unwrap();
    writeln!(code, "        }}").unwrap();
    writeln!(code, "        Ok(())").unwrap();
    writeln!(code, "    }}\n").unwrap();
    Ok(())
}

fn emit_stop_node(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Stops the Bitcoin node if one is running.\n\
         /// This is automatically called when the client is dropped."
    )
    .unwrap();
    writeln!(
        code,
        "    pub async fn stop_node(&mut self) -> Result<(), TransportError> {{"
    )
    .unwrap();
    writeln!(
        code,
        "        if let Some(mut manager) = self.node_manager.take() {{"
    )
    .unwrap();
    writeln!(code, "            manager.stop().await?;").unwrap();
    writeln!(code, "        }}").unwrap();
    writeln!(code, "        Ok(())").unwrap();
    writeln!(code, "    }}\n").unwrap();
    Ok(())
}

fn emit_node_manager_accessor(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Returns a reference to the node manager if one exists.\n\
         /// This can be used to access node configuration and control the node lifecycle."
    )
    .unwrap();
    writeln!(
        code,
        "    pub fn node_manager(&self) -> Option<&dyn NodeManager> {{"
    )
    .unwrap();
    writeln!(code, "        self.node_manager.as_deref()").unwrap();
    writeln!(code, "    }}\n").unwrap();
    Ok(())
}

fn emit_delegated_rpc_methods(code: &mut String, methods: &[ApiMethod]) -> std::io::Result<()> {
    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let ret_ty = if m.results.len() == 1 {
            "Value"
        } else {
            "Value"
        }
        .to_string();
        for line in doc_comment::format_doc_comment(&m.description).lines() {
            writeln!(code, "    {}", line).unwrap();
        }
        if m.arguments.is_empty() {
            let target = if WALLET_METHODS.contains(&m.name.as_str()) {
                "wallet_client"
            } else {
                "node_client"
            };
            writeln!(
                code,
                "    pub async fn {}(&self) -> Result<{}, TransportError> {{",
                method_snake, ret_ty
            )
            .unwrap();
            writeln!(code, "        self.{}.{}().await", target, method_snake).unwrap();
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
            writeln!(
                code,
                "    pub async fn {}(&self, {}) -> Result<{}, TransportError> {{",
                method_snake, param_list, ret_ty
            )
            .unwrap();
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
            let target = if WALLET_METHODS.contains(&m.name.as_str()) {
                "wallet_client"
            } else {
                "node_client"
            };
            writeln!(
                code,
                "        self.{}.{}({}).await",
                target, method_snake, args
            )
            .unwrap();
        }
        writeln!(code, "    }}\n").unwrap();
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
         /// ```\n"
    )
    .unwrap();
    writeln!(
        code,
        "    pub async fn send_to_address_with_conf_target(\n\
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
     }}\n"
    )
    .unwrap();
    writeln!(
        code,
        "    pub async fn send_to_address_with_fee_rate(\n\
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
    )
    .unwrap();
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
