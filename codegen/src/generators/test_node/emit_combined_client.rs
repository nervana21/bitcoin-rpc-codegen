//! Generate combined client with constructors and lifecycle helpers

use crate::generators::doc_comment;
use crate::utils::{camel_to_snake_case, rust_type_for_argument};
use crate::wallet_methods::WALLET_METHODS;
use rpc_api::ApiMethod;

use std::fmt::Write;

use super::utils::camel;
use crate::generators::test_node::versions::get_helpers_for_version;

/// Generates a complete Rust client struct and implementation for a collection of Bitcoin RPC methods.
///
/// This function creates a test-only client that wraps a transport layer and provides
/// async methods for each RPC call. The generated client:
/// - Converts RPC method names to snake_case for Rust conventions
/// - Handles parameter serialization to JSON values
pub fn generate_combined_client(
    client_name: &str,
    methods: &[ApiMethod],
    version: &str,
) -> std::io::Result<String> {
    let mut code = String::new();

    emit_imports(&mut code, version)?;
    emit_node_manager_trait(&mut code)?;
    emit_struct_definition(&mut code, client_name)?;
    emit_node_manager_impl(&mut code)?;
    let helpers = get_helpers_for_version(version);
    helpers.emit_wallet_options_struct(&mut code)?;
    writeln!(code, "impl {client_name} {{").unwrap();
    emit_constructors(&mut code)?;
    emit_wallet_methods(&mut code)?;
    helpers.emit_block_mining_helpers(&mut code)?;
    helpers.emit_reset_chain(&mut code)?;
    emit_stop_node(&mut code)?;
    emit_node_manager_accessor(&mut code)?;
    emit_rpc_accessor(&mut code)?;
    emit_batch_method(&mut code)?;
    emit_delegated_rpc_methods(&mut code, methods)?;
    helpers.emit_send_to_address_helpers(&mut code)?;
    writeln!(code, "}}\n").unwrap();
    emit_drop_impl(&mut code, client_name)?;

    Ok(code)
}

/// Generates the import statements for the combined Bitcoin test client.
///
/// This function emits all necessary Rust imports required for the `BitcoinTestClient`
/// implementation, including:
/// - Core dependencies (anyhow, std::sync::Arc)
/// - Transport layer types (TransportError, DefaultTransport, RpcClient, BatchBuilder)
/// - Version-specific type definitions from the generated types module
/// - Node management types (BitcoinNodeManager, TestConfig)
/// - Subclient types (BitcoinNodeClient, BitcoinWalletClient)
/// - Bitcoin-specific types (Amount)
///
/// The version parameter is used to generate the correct import path for
/// version-specific type definitions (e.g., `v29_types::*`).
///
/// # Arguments
/// * `code` - The string buffer to append the imports to
/// * `version` - The Bitcoin Core version string (e.g., "v29") used for type imports
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_imports(code: &mut String, version: &str) -> std::io::Result<()> {
    // Convert the version to lowercase for types module import
    let version_lowercase = version.to_lowercase();

    writeln!(
        code,
        "use anyhow::Result;
use std::sync::Arc;
use crate::transport::core::{{TransportError}};
use crate::transport::{{DefaultTransport, RpcClient, BatchBuilder}};
use crate::types::{version_lowercase}_types::*;
use serde_json::Value;

use crate::node::{{BitcoinNodeManager, TestConfig}};

use super::node::BitcoinNodeClient;
use super::wallet::BitcoinWalletClient;

use bitcoin::Amount;"
    )
    .unwrap();
    Ok(())
}

/// Generates the NodeManager trait definition for Bitcoin node lifecycle management.
///
/// This function emits a trait that abstracts the core operations needed to manage
/// a Bitcoin node's lifecycle in a test environment. The trait provides:
/// - Asynchronous start/stop operations for node lifecycle control
/// - RPC port access for network communication
/// - Type erasure support for dynamic dispatch
///
/// The trait is designed to be implemented by concrete node managers (like BitcoinNodeManager)
/// and used by the combined test client to abstract away the specific node implementation
/// details while providing a consistent interface for node management.
///
/// # Arguments
/// * `code` - The string buffer to append the trait definition to
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_node_manager_trait(code: &mut String) -> std::io::Result<()> {
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

/// Generates the struct definition for the combined Bitcoin test client.
///
/// This function emits a struct that contains the necessary components for the test client:
/// - A reference to the node client
/// - A reference to the wallet client
/// - A reference to the node manager
/// - A reference to the RPC client
pub fn emit_struct_definition(code: &mut String, client_name: &str) -> std::io::Result<()> {
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

/// Generates the implementation for the NodeManager trait for Bitcoin node lifecycle management.
///
/// This function emits the implementation of the NodeManager trait for the BitcoinNodeManager struct.
/// The implementation provides the concrete methods for starting and stopping the node,
/// accessing the RPC port, and providing type erasure.
///
pub fn emit_node_manager_impl(code: &mut String) -> std::io::Result<()> {
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

/// Generates the constructors for the combined Bitcoin test client.
///
/// This function emits the constructors for the `{client_name}` struct.
/// The constructors provide the means to create instances of the test client.
///
/// # Arguments
/// * `code` - The string buffer to append the constructors to
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_constructors(code: &mut String) -> std::io::Result<()> {
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
/// Generates the methods for the wallet for the combined Bitcoin test client.
///
/// This function emits the methods for the wallet for the `{client_name}` struct.
/// The methods provide the means to create and manage wallets.
///
/// # Arguments
/// * `code` - The string buffer to append the wallet methods to
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_wallet_methods(code: &mut String) -> std::io::Result<()> {
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

/// Generates the method for stopping the node for the combined Bitcoin test client.
///
/// This function emits the method for stopping the node for the `{client_name}` struct.
/// The method provides the means to stop the node.
///
/// # Arguments
/// * `code` - The string buffer to append the stop node method to
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_stop_node(code: &mut String) -> std::io::Result<()> {
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

/// Generates the method for accessing the node manager for the combined Bitcoin test client.
///
/// This function emits the method for accessing the node manager for the `{client_name}` struct.
/// The method provides the means to access the node manager.
///
/// # Arguments
/// * `code` - The string buffer to append the node manager accessor method to
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_node_manager_accessor(code: &mut String) -> std::io::Result<()> {
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

/// Generates the method for accessing the RPC client for the combined Bitcoin test client.
///
/// This function emits the method for accessing the RPC client for the `{client_name}` struct.
/// The method provides the means to access the RPC client.
///
/// # Arguments
/// * `code` - The string buffer to append the RPC accessor method to
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_rpc_accessor(code: &mut String) -> std::io::Result<()> {
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

/// Generates the method for beginning a JSON-RPC batch against the test node.
///
/// This function emits the method for beginning a JSON-RPC batch against the test node.
/// The method provides the means to begin a JSON-RPC batch against the test node.
///
/// # Arguments
pub fn emit_batch_method(code: &mut String) -> std::io::Result<()> {
    writeln!(
        code,
        "    /// Begin a JSON-RPC batch against this test node\n    pub fn batch(&self) -> BatchBuilder {{\n        self.rpc.batch()\n    }}\n"
    ).unwrap();
    Ok(())
}

/// Generates the methods for the delegated RPC methods for the combined Bitcoin test client.
///
/// This function emits the methods for the delegated RPC methods for the `{client_name}` struct.
/// The methods provide the means to delegate RPC methods to the node or wallet client.
///
/// # Arguments
/// * `code` - The string buffer to append the delegated RPC methods to
/// * `methods` - The methods to emit
///
/// # Returns
/// * `std::io::Result<()>` - Success or failure of writing to the code buffer
pub fn emit_delegated_rpc_methods(code: &mut String, methods: &[ApiMethod]) -> std::io::Result<()> {
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
                    let ty = rust_type_for_argument(&arg.names[0], &arg.type_);
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

/// Generates the implementation for the Drop trait for the combined Bitcoin test client.
///
/// This function emits the implementation for the Drop trait for the `{client_name}` struct.
/// The implementation provides the means to drop the client.
///
/// # Arguments
pub fn emit_drop_impl(code: &mut String, client_name: &str) -> std::io::Result<()> {
    writeln!(code, "impl Drop for {client_name} {{").unwrap();
    writeln!(code, "    fn drop(&mut self) {{").unwrap();
    writeln!(code, "        let _ = self.node_manager.take();").unwrap();
    writeln!(code, "    }}").unwrap();
    writeln!(code, "}}\n").unwrap();
    Ok(())
}
