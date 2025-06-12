// codegen/templates/client_trait.rs

use async_trait::async_trait;
use crate::transport::Transport;
{{IMPORTS}}

#[doc = r#"A versioned client trait for Bitcoin Core v{{VERSION}}"#]
#[async_trait]
pub trait BitcoinClientV{{VERSION_NODOTS}}: Send + Sync {
{{TRAIT_METHODS}}
}

#[async_trait]
impl<C> BitcoinClientV{{VERSION_NODOTS}} for C
where
    C: Transport + Sync + Send,
{
{{IMPL_METHODS}}
}

/// Helper to route calls to the node or wallet namespace automatically.
pub trait RpcDispatchExt: Transport + TransportExt {
    /// Dispatch JSON-RPC methods by name.
    fn dispatch_json<R: DeserializeOwned>(
        &self,
        method: &str,
        params: &[serde_json::Value],
    ) -> impl Future<Output = Result<R, TransportError>> + Send {
        async move {
            if WALLET_METHODS.contains(&method) {
                self.wallet_call(method, params).await
            } else {
                self.call(method, params).await
            }
        }
    }
}

impl<T: Transport + TransportExt + ?Sized> RpcDispatchExt for T {}
// helper trait, so any Transport gets a wallet_call by default
pub trait WalletTransportExt: Transport + TransportExt {
    fn wallet_call<T: serde::Serialize + std::marker::Sync, R: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &[T],
    ) -> impl std::future::Future<Output = Result<R, crate::transport::TransportError>> + Send { async {
        // Convert params to Value before passing to call
        let value_params: Vec<serde_json::Value> = params
            .iter()
            .map(|p| serde_json::to_value(p).unwrap())
            .collect();
        self.call(method, &value_params).await
    }}
}
impl<T: Transport + TransportExt + ?Sized> WalletTransportExt for T {}