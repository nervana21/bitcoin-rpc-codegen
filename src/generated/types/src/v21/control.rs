use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the api RPC call.
///
/// Return JSON description of RPC API.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ObjectResponse {
    pub result: serde_json::Value,
}

/// Response for the getmemoryinfo RPC call.
///
/// Returns an object containing information about memory usage.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmemoryinfoResponse {
    /// Information about locked memory manager
    pub locked: serde_json::Locked,

}

/// Response for the getrpcinfo RPC call.
///
/// Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    /// All active commands
    pub active_commands: Vec<serde_json::Value>,
    /// The complete file path to the debug log
    pub logpath: String,

}

/// Response for the help RPC call.
///
/// List all commands, or get help for a specified command.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the logging RPC call.
///
/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoggingResponse {
    /// if being debug logged or not. false:inactive, true:active
    pub category: bool,

}

/// Response for the stop RPC call.
///
/// Request a graceful shutdown of Bitcoin Core.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the uptime RPC call.
///
/// Returns the total uptime of the server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UptimeResponse {
    pub result: number,
}

