//! Code-gen: build a thin `TestNode` client with typed-parameter helpers.
//!
//! Until we have a TypesCodeGenerator that emits concrete `*Response` structs
//! every RPC simply returns `serde_json::Value`.

use crate::utils::camel_to_snake_case;
use crate::wallet_methods::WALLET_METHODS;
use crate::{generators::doc_comment, CodeGenerator, TYPE_REGISTRY};
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
        let mut result_code = String::new();
        let mut mod_rs_code = String::new();

        /* ---------- params.rs ---------- */
        writeln!(params_code, "//! Parameter structs for RPC method calls").unwrap();
        writeln!(params_code, "use serde::Serialize;\n").unwrap();

        for m in methods {
            if m.arguments.is_empty() {
                continue;
            }
            let doc_comment = doc_comment::format_doc_comment(&m.description);
            for line in doc_comment.lines() {
                writeln!(params_code, "/// {}", line).unwrap();
            }
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

        // Split methods into wallet and node methods
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

        // Generate wallet and node client code
        let wallet_code = generate_subclient("BitcoinWalletClient", &wallet_methods).unwrap();
        let node_code = generate_subclient("BitcoinNodeClient", &node_methods).unwrap();

        // Generate combined client code
        let combined_code = generate_combined_client("BitcoinTestClient", methods).unwrap();

        // Update mod.rs to export all clients
        writeln!(mod_rs_code, "//! Test node module for Bitcoin RPC testing").unwrap();
        writeln!(mod_rs_code, "pub mod params;").unwrap();
        writeln!(mod_rs_code, "pub mod result;").unwrap();
        writeln!(mod_rs_code, "pub mod wallet;").unwrap();
        writeln!(mod_rs_code, "pub mod node;").unwrap();
        writeln!(mod_rs_code, "pub mod test_node;").unwrap();
        writeln!(mod_rs_code, "pub use wallet::BitcoinWalletClient;").unwrap();
        writeln!(mod_rs_code, "pub use node::BitcoinNodeClient;").unwrap();
        writeln!(mod_rs_code, "pub use test_node::BitcoinTestClient;").unwrap();

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

    writeln!(code, "use anyhow::Result;").unwrap();
    writeln!(code, "use serde_json::Value;").unwrap();
    writeln!(
        code,
        "use crate::transport::core::{{TransportExt, DefaultTransport, TransportError}};\n"
    )
    .unwrap();
    writeln!(code, "use bitcoin::Amount;").unwrap();
    writeln!(code, "use std::option::Option;\n").unwrap();

    writeln!(code, "#[derive(Debug, Clone)]").unwrap();
    writeln!(code, "pub struct {} {{", client_name).unwrap();
    writeln!(code, "    client: Box<DefaultTransport>,").unwrap();
    writeln!(code, "}}\n").unwrap();

    writeln!(code, "impl {} {{", client_name).unwrap();
    writeln!(
        code,
        "    pub fn new(client: Box<DefaultTransport>) -> Self {{"
    )
    .unwrap();
    writeln!(code, "        Self {{ client }}").unwrap();
    writeln!(code, "    }}\n").unwrap();

    // Add a method to update the transport
    writeln!(
        code,
        "    pub fn with_transport(&mut self, client: Box<DefaultTransport>) {{"
    )
    .unwrap();
    writeln!(code, "        self.client = client;").unwrap();
    writeln!(code, "    }}\n").unwrap();

    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let ret_ty = if m.results.len() == 1 {
            format!("Value")
        } else {
            "Value".to_string()
        };

        let doc_comment = doc_comment::format_doc_comment(&m.description);
        for line in doc_comment.lines() {
            writeln!(code, "    {}", line).unwrap();
        }

        if m.arguments.is_empty() {
            writeln!(
                code,
                "    pub async fn {}(&self) -> Result<{}, TransportError> {{",
                method_snake, ret_ty
            )
            .unwrap();
            writeln!(
                code,
                "        Ok(self.client.call::<{}>(\"{}\", &[]).await?.into())",
                ret_ty, m.name
            )
            .unwrap();
        } else {
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
                // Special case for fee_rate to always be Option<Amount>
                let param_ty = if arg.names[0] == "fee_rate" {
                    "Option<Amount>".to_string()
                } else {
                    rust_type_for(&arg.names[0], &arg.type_)
                };
                write!(param_list, "{}: {}", param_name, param_ty).unwrap();
            }

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
                "        Ok(self.client.call::<{}>(\"{}\", &vec).await?.into())",
                ret_ty, m.name
            )
            .unwrap();
        }

        writeln!(code, "    }}\n").unwrap();
    }

    writeln!(code, "}}\n").unwrap();
    Ok(code)
}

fn generate_combined_client(client_name: &str, methods: &[ApiMethod]) -> std::io::Result<String> {
    let mut code = String::new();

    writeln!(code, "use anyhow::Result;").unwrap();
    writeln!(code, "use serde_json::Value;").unwrap();
    writeln!(
        code,
        "use crate::transport::core::{{DefaultTransport, TransportError}};\n"
    )
    .unwrap();
    writeln!(code, "use super::node::BitcoinNodeClient;").unwrap();
    writeln!(code, "use super::wallet::BitcoinWalletClient;\n").unwrap();
    writeln!(code, "use std::str::FromStr;").unwrap();
    writeln!(code, "use bitcoin::Amount;\n").unwrap();

    // Add a trait for node management
    writeln!(
        code,
        "/// Trait for managing a Bitcoin node's lifecycle\n\
         pub trait NodeManager: Send + Sync + std::fmt::Debug + std::any::Any {{\n\
             /// Start the node\n\
             fn start(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;\n\
             /// Stop the node\n\
             fn stop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>>;\n\
             /// Get the RPC port\n\
             fn rpc_port(&self) -> u16;\n\
             /// Convert to Any for downcasting\n\
             fn as_any(&self) -> &dyn std::any::Any;\n\
             /// Synchronous cleanup - called during drop\n\
             fn sync_cleanup(&mut self);\n\
         }}\n"
    )
    .unwrap();

    writeln!(code, "#[derive(Debug)]").unwrap();
    writeln!(code, "pub struct {} {{", client_name).unwrap();
    writeln!(code, "    node_client: BitcoinNodeClient,").unwrap();
    writeln!(code, "    wallet_client: BitcoinWalletClient,").unwrap();
    writeln!(code, "    node_manager: Option<Box<dyn NodeManager>>,").unwrap();
    writeln!(code, "}}\n").unwrap();

    // Update the NodeManager implementation for BitcoinNodeManager
    writeln!(
        code,
        "impl NodeManager for node::BitcoinNodeManager {{\n\
             fn start(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>> {{\n\
                 println!(\"[DEBUG] NodeManager::start called on BitcoinNodeManager\");\n\
                 Box::pin(async move {{\n\
                     println!(\"[DEBUG] Inside NodeManager::start async block\");\n\
                     let result = <node::BitcoinNodeManager as node::NodeManager>::start(self).await;\n\
                     println!(\"[DEBUG] NodeManager::start result: {{:?}}\", result);\n\
                     result.map_err(|e| TransportError::Rpc(e.to_string()))\n\
                 }})\n\
             }}\n\
             \n\
             fn stop(&mut self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), TransportError>> + Send + '_>> {{\n\
                 println!(\"[DEBUG] NodeManager::stop called on BitcoinNodeManager\");\n\
                 Box::pin(async move {{\n\
                     println!(\"[DEBUG] Inside NodeManager::stop async block\");\n\
                     let result = node::BitcoinNodeManager::stop(self).await;\n\
                     println!(\"[DEBUG] NodeManager::stop result: {{:?}}\", result);\n\
                     result.map_err(|e| TransportError::Rpc(e.to_string()))\n\
                 }})\n\
             }}\n\
             \n\
             fn rpc_port(&self) -> u16 {{\n\
                 println!(\"[DEBUG] NodeManager::rpc_port called on BitcoinNodeManager\");\n\
                 self.rpc_port()\n\
             }}\n\
             \n\
             fn as_any(&self) -> &dyn std::any::Any {{\n\
                 println!(\"[DEBUG] NodeManager::as_any called on BitcoinNodeManager\");\n\
                 self\n\
             }}\n\
             \n\
             fn sync_cleanup(&mut self) {{\n\
                 println!(\"[DEBUG] NodeManager::sync_cleanup called on BitcoinNodeManager\");\n\
                 // This is a no-op since BitcoinNodeManager handles cleanup in its own Drop impl\n\
             }}\n\
         }}\n"
    )
    .unwrap();

    writeln!(code, "impl {} {{", client_name).unwrap();

    // Add new_with_manager constructor
    writeln!(
        code,
        "    /// Creates a new Bitcoin test client with a specific node manager.\n\
         /// This allows for custom node configuration and lifecycle management.\n\
         /// # Example\n\
         /// ```no_run\n\
         /// use midas::test_node::test_node::{{BitcoinTestClient, NodeManager}};\n\
         /// #[tokio::main]\n\
         /// async fn main() -> anyhow::Result<()> {{\n\
         ///     let node_manager = Box::new(YourNodeManager::new()?);\n\
         ///     let mut client = BitcoinTestClient::new_with_manager(node_manager).await?;\n\
         ///     // Use the client...\n\
         ///     Ok(())\n\
         /// }}\n\
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
         let client = Box::new(DefaultTransport::new(\n\
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

    // Update the ensure_wallet method
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
        "                let new_transport = Box::new(DefaultTransport::new("
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

    // Update the mine_blocks method to use ensure_wallet
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

    // Add this after the mine_blocks method in generate_combined_client
    writeln!(
        code,
        "    /// Resets the blockchain to a clean state.\n\
         /// This method:\n\
         /// 1. First attempts to prune the blockchain to height 0\n\
         /// 2. If blocks remain, invalidates all blocks except genesis\n\
         /// 3. Reconsiders the genesis block to maintain a valid chain\n\
         /// # Example\n\
         /// ```no_run\n\
         /// use midas::test_node::test_node::BitcoinTestClient;\n\
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

    // Add stop() method
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

    // Add node_manager() method
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

    // Generate methods that delegate to either node_client or wallet_client
    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let ret_ty = if m.results.len() == 1 {
            format!("Value")
        } else {
            "Value".to_string()
        };

        let doc_comment = doc_comment::format_doc_comment(&m.description);
        for line in doc_comment.lines() {
            writeln!(code, "    {}", line).unwrap();
        }

        if m.arguments.is_empty() {
            writeln!(
                code,
                "    pub async fn {}(&self) -> Result<{}, TransportError> {{",
                method_snake, ret_ty
            )
            .unwrap();
            // Check if this is a wallet method
            if WALLET_METHODS.contains(&m.name.as_str()) {
                writeln!(code, "        self.wallet_client.{}().await", method_snake).unwrap();
            } else {
                writeln!(code, "        self.node_client.{}().await", method_snake).unwrap();
            }
        } else {
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
                code,
                "    pub async fn {}(&self, {}) -> Result<{}, TransportError> {{",
                method_snake, param_list, ret_ty
            )
            .unwrap();

            let mut args = String::new();
            for (i, arg) in m.arguments.iter().enumerate() {
                if i > 0 {
                    args.push_str(", ");
                }
                let param_name = if arg.names[0] == "type" {
                    "_type"
                } else {
                    &camel_to_snake_case(&arg.names[0])
                };
                write!(args, "{}", param_name).unwrap();
            }

            // Check if this is a wallet method
            if WALLET_METHODS.contains(&m.name.as_str()) {
                writeln!(
                    code,
                    "        self.wallet_client.{}({}).await",
                    method_snake, args
                )
                .unwrap();
            } else {
                writeln!(
                    code,
                    "        self.node_client.{}({}).await",
                    method_snake, args
                )
                .unwrap();
            }
        }
        writeln!(code, "    }}\n").unwrap();
    }

    writeln!(
        code,
        "    /// Creates a new Bitcoin test client with default settings.\n\
         /// This will start a new Bitcoin node in regtest mode.\n\
         /// # Example\n\
         /// ```no_run\n\
         /// use midas::test_node::test_node::BitcoinTestClient;\n\
         /// #[tokio::main]\n\
         /// async fn main() -> anyhow::Result<()> {{\n\
         ///     let mut client = BitcoinTestClient::new().await?;\n\
         ///     // Use the client...\n\
         ///     Ok(())\n\
         /// }}\n\
         /// ```\n"
    )
    .unwrap();
    writeln!(
        code,
        "    pub async fn new() -> Result<Self, TransportError> {{"
    )
    .unwrap();
    writeln!(
        code,
        "        let node_manager = node::BitcoinNodeManager::new()?;"
    )
    .unwrap();
    writeln!(code, "        Self::new_with_manager(node_manager).await").unwrap();
    writeln!(code, "    }}\n").unwrap();

    // Update the helper methods for sendtoaddress
    writeln!(
        code,
        "    /// Helper method to send bitcoin to an address with either a confirmation target or fee rate.\n\
         /// This is a more ergonomic wrapper around sendtoaddress that prevents specifying both conf_target and fee_rate.\n\
         /// # Example\n\
         /// ```no_run\n\
         /// use midas::test_node::test_node::BitcoinTestClient;\n\
         /// use bitcoin::Amount;\n\
         /// #[tokio::main]\n\
         /// async fn main() -> anyhow::Result<()> {{\n\
         ///     let mut client = BitcoinTestClient::new().await?;\n\
         ///     // Send with confirmation target\n\
         ///     client.send_to_address_with_conf_target(\n\
         ///         \"bc1q...\",\n\
         ///         Amount::from_btc(0.1).unwrap(),\n\
         ///         6u64,\n\
         ///         \"economical\".to_string(),\n\
         ///     ).await?;\n\
         ///     // Or send with fee rate\n\
         ///     client.send_to_address_with_fee_rate(\n\
         ///         \"bc1q...\",\n\
         ///         Amount::from_btc(0.1).unwrap(),\n\
         ///         Amount::from_sat(1.1),\n\
         ///     ).await?;\n\
         ///     Ok(())\n\
         /// }}\n\
         /// ```"
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

    // Close the impl block
    writeln!(code, "}}\n").unwrap();

    // Add Drop implementation
    writeln!(code, "impl Drop for {} {{", client_name).unwrap();
    writeln!(code, "    fn drop(&mut self) {{").unwrap();
    writeln!(
        code,
        "        if let Some(mut manager) = self.node_manager.take() {{"
    )
    .unwrap();
    writeln!(code, "            manager.sync_cleanup();").unwrap();
    writeln!(code, "        }}").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}\n").unwrap();

    Ok(code)
}
