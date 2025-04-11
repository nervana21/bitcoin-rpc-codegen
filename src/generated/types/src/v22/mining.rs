use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the getmininginfo RPC call.
///
/// Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    /// The current block
    pub blocks: f64,
    /// The block weight of the last assembled block (only present if a block was ever assembled)
    pub currentblockweight: f64,
    /// The number of block transactions of the last assembled block (only present if a block was ever assembled)
    pub currentblocktx: f64,
    /// The current difficulty
    pub difficulty: f64,
    /// The network hashes per second
    pub networkhashps: f64,
    /// The size of the mempool
    pub pooledtx: f64,
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,

}

/// Response for the getnetworkhashps RPC call.
///
/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkhashpsResponse {
    pub result: number,
}

/// Response for the getprioritisedtransactions RPC call.
///
/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetprioritisedtransactionsResponse {

    pub <transactionid>: serde_json::<transactionid>,

}

/// Response for the prioritisetransaction RPC call.
///
/// Accepts the transaction into mined blocks at a higher (or lower) priority
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

