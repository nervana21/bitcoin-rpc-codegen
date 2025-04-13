use anyhow::Result;
use bitcoincore_rpc::{Auth, Client as RpcClient, RpcApi}; // Import RpcApi to bring the call method into scope
use serde::{Serialize, de::DeserializeOwned};

pub struct NodeClient {
    rpc: RpcClient,
}

impl NodeClient {
    /// Connect to a Bitcoin Core node using regtest parameters.
    pub fn new(rpc_url: &str, user: &str, password: &str) -> Result<Self> {
        let auth = Auth::UserPass(user.to_owned(), password.to_owned());
        let rpc = RpcClient::new(rpc_url, auth)?;
        Ok(Self { rpc })
    }

    /// A generic call method that sends JSON-RPC requests.
    ///
    /// This version is generic over parameter type `P` (where `P: Serialize`)
    /// and converts the slice of parameters into a Vec of serde_json::Value.
    pub fn call<T, P>(&self, method: &str, params: &[P]) -> Result<T>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        // Convert each parameter into serde_json::Value.
        let json_params: Vec<serde_json::Value> = params
            .iter()
            .map(|p| serde_json::to_value(p))
            .collect::<Result<Vec<_>, _>>()?;
        // Call the RPC method with a slice of JSON values.
        let result = self.rpc.call(method, &json_params)?;
        Ok(result)
    }
}
