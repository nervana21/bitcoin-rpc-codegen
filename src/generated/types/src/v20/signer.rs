use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the enumeratesigners RPC call.
///
/// Returns a list of external signers from -signer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EnumeratesignersResponse {

    pub signers: Vec<serde_json::Value>,

}

