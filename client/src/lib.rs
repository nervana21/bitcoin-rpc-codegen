//! High‑level, ergonomic wrapper around [`transport::Transport`].
//!
//! `RpcClient` exposes two flavours of call:
//! * **`call_method()`** – low‑level, returns raw `serde_json::Value`; you
//!   hand‑craft the parameters yourself.  Handy for debugging or when a
//!   response type has not (yet) been code‑generated.
//! * **`call()`** – high‑level, generic over both the *param* and *result*
//!   types.  It JSON‑serialises your input slice and de‑serialises the
//!   `"result"` field into the requested output type `R`.
//!
//! ```rust,ignore
//! let client = RpcClient::new("http://127.0.0.1:8332");
//! let height : GetblockcountResponse = client
//!     .call("getblockcount", &[])
//!     .await?;
//! println!("block height = {}", height.count);
//! ```

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use thiserror::Error;
use transport::{Transport, TransportError};

/// **`RpcClient`**
///
/// Thin convenience layer that keeps a configured [`Transport`] value and
/// offers strongly‑typed helper methods.
///
/// The struct is **`Clone`** so you can pass it around freely – it merely
/// wraps an `Arc<Client>` internally.
#[derive(Clone)]
pub struct RpcClient {
    transport: Transport,
}

/// Errors surfaced by the *high‑level* client layer.
///
/// All networking / protocol failures are converted from
/// `transport::TransportError`; anything else indicates an unexpected JSON
/// shape that the caller attempted to parse into `R`.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error(transparent)]
    Transport(#[from] TransportError),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl RpcClient {
    // ---------------------------------------------------------------------
    //  Constructors
    // ---------------------------------------------------------------------

    /// Build a client against `url`, e.g. `"http://127.0.0.1:18443"`.
    ///
    /// Uses default cookie‑file auth (Bitcoin Core’s standard `~/.cookie`)
    /// or whatever authentication the underlying [`Transport::new()`]
    /// decides to apply.
    pub fn new<U: Into<String>>(url: U) -> Self {
        RpcClient {
            transport: Transport::new(url),
        }
    }

    /// Same as [`new`](Self::new) but wires HTTP *Basic* authentication.
    pub fn new_with_auth<U: Into<String>>(url: U, username: &str, password: &str) -> Self {
        RpcClient {
            transport: Transport::new_with_auth(url, username, password),
        }
    }

    // ---------------------------------------------------------------------
    //  Generic call helpers
    // ---------------------------------------------------------------------

    /// **Low‑level** JSON‑RPC call – returns the raw `"result"` field as
    /// `serde_json::Value`.
    ///
    /// Use this when you want to poke an RPC that does not yet have a
    /// generated response struct.
    pub async fn call_method(&self, method: &str, params: &[Value]) -> Result<Value, ClientError> {
        Ok(self.transport.send_request(method, params).await?)
    }

    /// **High‑level** JSON‑RPC call – automatic (de)serialisation.
    ///
    /// * `T` – type that is *serialisable* (each param is turned into JSON);  
    /// * `R` – type that is *deserialisable* (the `"result"` field will be
    ///   parsed into this, or an error bubbles up).
    ///
    /// # Errors
    /// * Any transport‑level error (`io`, HTTP, RPC‑level) arrives as
    ///   `ClientError::Transport`.
    /// * If the JSON does not match `R`, the underlying `serde_json::Error`
    ///   is wrapped in `ClientError::InvalidResponse`.
    pub async fn call<T: Serialize, R: DeserializeOwned>(
        &self,
        method: &str,
        params: &[T],
    ) -> Result<R, ClientError> {
        self.transport
            .call(method, params)
            .await
            .map_err(Into::into)
    }
}
