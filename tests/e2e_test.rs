use anyhow::Result;

mod generated {
    pub mod v28 {
        /// The client module: we define our custom Client and inject the generated RPC macros.
        pub mod client {
            use anyhow::Result;
            use bitcoincore_rpc::{Auth, Client as RpcClient, RpcApi};
            use serde::{Deserialize, Serialize};

            /// Our custom Client that wraps bitcoincore_rpc::Client.
            pub struct Client {
                pub rpc: RpcClient,
            }

            impl Client {
                /// Create a new client instance using hardcoded regtest credentials.
                pub fn new(rpc_url: &str, user: &str, password: &str) -> Result<Self> {
                    let auth = Auth::UserPass(user.to_string(), password.to_string());
                    let rpc = RpcClient::new(rpc_url, auth)?;
                    Ok(Client { rpc })
                }

                /// A generic call method that converts parameters to JSON.
                pub fn call<T, P>(&self, method: &str, params: &[P]) -> Result<T>
                where
                    T: for<'de> Deserialize<'de>,
                    P: Serialize,
                {
                    // Use our library's patched serde_json.
                    let json_params: Vec<bitcoin_rpc_codegen::serde_json::Value> = params
                        .iter()
                        .map(|p| bitcoin_rpc_codegen::serde_json::to_value(p))
                        .collect::<Result<Vec<_>, _>>()?;
                    let result = self.rpc.call(method, &json_params)?;
                    Ok(result)
                }
            }

            // Include the generated blockchain RPC method macros.
            // (The build script must generate the file into OUT_DIR.)
            include!(concat!(env!("OUT_DIR"), "/client/src/v28/blockchain.rs"));
        }

        /// The types module: include the generated response types.
        pub mod types {
            // Define object_dynamic as our patched serde_json Value.
            #[allow(non_camel_case_types)]
            pub type object_dynamic = bitcoin_rpc_codegen::serde_json::Value;
            include!(concat!(env!("OUT_DIR"), "/types/src/v28/blockchain.rs"));
        }
    }
}

// Re-export items as a user would.
use generated::v28::client::Client;

// Implement getblockcount RPC method that takes no parameters
// and returns the current block height as a JSON value
impl Client {
    pub fn getblockcount(&self) -> anyhow::Result<bitcoin_rpc_codegen::serde_json::Value> {
        self.call("getblockcount", &[] as &[()])
    }
    pub fn getblockhash(
        &self,
        height: i64,
    ) -> anyhow::Result<bitcoin_rpc_codegen::serde_json::Value> {
        self.call("getblockhash", &[height])
    }
}

#[test]
fn e2e_test_getblockcount() -> Result<()> {
    // Hardcoded connection details for your regtest bitcoind instance.
    let rpc_url = "http://127.0.0.1:18443";
    let rpc_user = "rpcuser";
    let rpc_pass = "rpcpassword";

    // Instantiate the client.
    let client =
        Client::new(rpc_url, rpc_user, rpc_pass).expect("Failed to create client instance");
    println!("Client instantiated successfully.");

    // Call the getblockcount RPC.
    let json_resp = client
        .getblockcount()
        .expect("RPC call to getblockcount failed");
    println!("Raw JSON response: {}", json_resp);

    // Deserialize the JSON response into a number
    let block_count: i64 = bitcoin_rpc_codegen::serde_json::from_value(json_resp)
        .expect("Failed to deserialize block count");
    println!("Current block count: {}", block_count);

    // For regtest mode, the block count should be >= 0
    assert!(block_count >= 0, "Block count should be non-negative");
    println!("End-to-end test succeeded!");

    Ok(())
}

#[test]
fn e2e_test_getblockhash() -> Result<()> {
    // Hardcoded connection details for your regtest bitcoind instance.
    let rpc_url = "http://127.0.0.1:18443";
    let rpc_user = "rpcuser";
    let rpc_pass = "rpcpassword";

    // Instantiate the client.
    let client =
        Client::new(rpc_url, rpc_user, rpc_pass).expect("Failed to create client instance");
    let block_hash = client
        .getblockhash(0)
        .expect("RPC call to getblockhash failed");
    println!("Block hash: {}", block_hash);

    Ok(())
}
