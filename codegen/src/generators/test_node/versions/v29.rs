use super::VersionedClientHelpers;
use std::fmt::Write;

/// Bitcoin Core v29 API helper implementations
pub struct V29Helpers;

impl VersionedClientHelpers for V29Helpers {
    fn emit_send_to_address_helpers(&self, code: &mut String) -> std::io::Result<()> {
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
             amount,\n\
             \"\".to_string(),\n\
             \"\".to_string(),\n\
             false,\n\
             true,\n\
             conf_target,\n\
             estimate_mode,\n\
             false,\n\
             0.0,\n\
             false,\n\
         ).await?)?)\n\
     }}\n\
     \n\
     pub async fn send_to_address_with_fee_rate(\n\
     &self,\n\
     address: String,\n\
     amount: Amount,\n\
     fee_rate: f64,\n\
 ) -> Result<Value, TransportError> {{\n\
     Ok(serde_json::to_value(self.wallet_client.sendtoaddress(\n\
         address,\n\
         amount,\n\
         \"\".to_string(),\n\
         \"\".to_string(),\n\
         false,\n\
         true,\n\
         0u64,\n\
         \"unset\".to_string(),\n\
         false,\n\
         fee_rate,\n\
         false,\n\
     ).await?)?)\n\
 }}\n"
    ).unwrap();
        Ok(())
    }

    /// Generates the struct definition for the wallet options for the combined Bitcoin test client.
    ///
    /// This function emits a struct that contains the necessary components for the wallet options:
    /// - A reference to the node client
    /// - A reference to the wallet client
    /// - A reference to the node manager
    /// - A reference to the RPC client
    ///
    /// # Arguments
    /// * `code` - The string buffer to append the wallet options struct to
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Success or failure of writing to the code buffer
    fn emit_wallet_options_struct(&self, code: &mut String) -> std::io::Result<()> {
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

    /// Generates the methods for the block mining for the combined Bitcoin test client.
    ///
    /// This function emits the methods for the block mining for the `{client_name}` struct.
    /// The methods provide the means to mine blocks.
    ///
    /// # Arguments
    /// * `code` - The string buffer to append the block mining methods to
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Success or failure of writing to the code buffer
    fn emit_block_mining_helpers(&self, code: &mut String) -> std::io::Result<()> {
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
        let blocks = self.node.generatetoaddress(
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

    /// Generates the method for resetting the chain for the combined Bitcoin test client.
    ///
    /// This function emits the method for resetting the chain for the `{client_name}` struct.
    /// The method provides the means to reset the chain to a clean state.
    ///
    /// # Arguments
    /// * `code` - The string buffer to append the reset chain method to
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Success or failure of writing to the code buffer
    fn emit_reset_chain(&self, code: &mut String) -> std::io::Result<()> {
        writeln!(
            code,
            "    /// Resets the blockchain to a clean state.\n\
         /// This method:\n\
         /// 1. First attempts to prune the blockchain to height 0\n\
         /// 2. If blocks remain, invalidates all blocks except genesis\n\
         /// 3. Reconsiders the genesis block to maintain a valid chain\n\
         pub async fn reset_chain(&mut self) -> Result<(), TransportError> {{\n\
             // First try pruning to height 0\n\
             self.node.pruneblockchain(0).await?;\n\
             // Check if we still have blocks\n\
             let info = self.node.getblockchaininfo().await?;\n\
             let current_height = info.blocks;\n\
             if current_height > 1 {{\n\
                 // Invalidate all blocks except genesis\n\
                 for height in (1..=current_height).rev() {{\n\
                     let block_hash = self.node.getblockhash(height).await?.0;\n\
                     self.node.invalidateblock(block_hash).await?;\n\
                 }}\n\
                 // Reconsider genesis block\n\
                 let genesis_hash = self.node.getblockhash(0).await?.0;\n\
                 self.node.reconsiderblock(genesis_hash).await?;\n\
             }}\n\
             Ok(())\n\
         }}\n"
        )
        .map_err(std::io::Error::other)
    }
}
