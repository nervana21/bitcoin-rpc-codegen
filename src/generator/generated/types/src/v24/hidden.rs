use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the addconnection RPC call.
///
/// Open an outbound connection to a specified node. This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddconnectionResponse {
    /// Address of newly added connection.
    pub address: String,
    /// Type of connection opened.
    pub connection_type: String,

}

/// Response for the addpeeraddress RPC call.
///
/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddpeeraddressResponse {
    /// whether the peer address was successfully added to the address manager table
    pub success: bool,
    /// error description, if the address could not be added
    pub error: String,

}

/// Response for the echo RPC call.
///
/// Simply echo back the input arguments. This command is for testing.
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EchoResponse {
    pub result: any,
}

/// Response for the echoipc RPC call.
///
/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the echojson RPC call.
///
/// Simply echo back the input arguments. This command is for testing.
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EchojsonResponse {
    pub result: any,
}

/// Response for the estimaterawfee RPC call.
///
/// WARNING: This interface is unstable and may disappear or change!
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EstimaterawfeeResponse {
    /// estimate for short time horizon
    pub short: serde_json::Short,
    /// estimate for medium time horizon
    pub medium: serde_json::Medium,
    /// estimate for long time horizon
    pub long: serde_json::Long,

}

/// Response for the generateblock RPC call.
///
/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenerateblockResponse {
    /// hash of generated block
    pub hash: Hex,
    /// hex of generated block, only present when submit=false
    pub hex: Hex,

}

/// Response for the generatetoaddress RPC call.
///
/// Mine to a specified address and return the block hashes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GeneratetoaddressResponse {
    /// blockhash
    pub result: Hex,

}

/// Response for the generatetodescriptor RPC call.
///
/// Mine to a specified descriptor and return the block hashes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GeneratetodescriptorResponse {
    /// blockhash
    pub result: Hex,

}

/// Response for the getorphantxs RPC call.
///
/// Shows transactions in the tx orphanage.
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetorphantxsResponse {
    /// The transaction hash in hex
    pub txid: Hex,

}

/// Response for the getrawaddrman RPC call.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Returns information on all address manager entries for the new and tried tables.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawaddrmanResponse {
    /// buckets with addresses in the address manager table ( new, tried )
    pub table: object_dynamic,

}

/// Response for the sendmsgtopeer RPC call.
///
/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ObjectResponse {
    pub result: serde_json::Value,
}

/// Response for the waitforblock RPC call.
///
/// Waits for a specific new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitforblockResponse {
    /// The blockhash
    pub hash: Hex,
    /// Block height
    pub height: f64,

}

/// Response for the waitforblockheight RPC call.
///
/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitforblockheightResponse {
    /// The blockhash
    pub hash: Hex,
    /// Block height
    pub height: f64,

}

/// Response for the waitfornewblock RPC call.
///
/// Waits for any new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WaitfornewblockResponse {
    /// The blockhash
    pub hash: Hex,
    /// Block height
    pub height: f64,

}

