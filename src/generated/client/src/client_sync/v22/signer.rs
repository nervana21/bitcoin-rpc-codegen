/// Implements Bitcoin Core JSON-RPC API method `enumeratesigners` for version v22
///
/// Returns a list of external signers from -signer.
#[macro_export]
macro_rules! impl_client_v22__enumeratesigners {
    () => {
        impl Client {
            pub fn enumeratesigners(&self) -> Result<serde_json::Value> {
                self.call("enumeratesigners", &[])
            }
        }
    };
}

