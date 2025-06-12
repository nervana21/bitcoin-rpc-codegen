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