use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the getaddednodeinfo RPC call.
///
/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddednodeinfoResponse {

    pub result: serde_json::Value,

}

/// Response for the getaddrmaninfo RPC call.
///
/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    /// the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
    pub network: serde_json::Network,

}

/// Response for the getconnectioncount RPC call.
///
/// Returns the number of connections to other nodes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetconnectioncountResponse {
    pub result: number,
}

/// Response for the getnettotals RPC call.
///
/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnettotalsResponse {
    /// Total bytes received
    pub totalbytesrecv: f64,
    /// Total bytes sent
    pub totalbytessent: f64,
    /// Current system UNIX epoch time in milliseconds
    pub timemillis: Time,

    pub uploadtarget: serde_json::Uploadtarget,

}

/// Response for the getnetworkinfo RPC call.
///
/// Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    /// the server version
    pub version: f64,
    /// the server subversion string
    pub subversion: String,
    /// the protocol version
    pub protocolversion: f64,
    /// the services we offer to the network
    pub localservices: Hex,
    /// the services we offer to the network, in human-readable form
    pub localservicesnames: Vec<String>,
    /// true if transaction relay is requested from peers
    pub localrelay: bool,
    /// the time offset
    pub timeoffset: f64,
    /// the total number of connections
    pub connections: f64,
    /// the number of inbound connections
    pub connections_in: f64,
    /// the number of outbound connections
    pub connections_out: f64,
    /// whether p2p networking is enabled
    pub networkactive: bool,
    /// information per network
    pub networks: Vec<serde_json::Value>,
    /// minimum relay fee rate for transactions in BTC/kvB
    pub relayfee: f64,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalfee: f64,
    /// list of local addresses
    pub localaddresses: Vec<serde_json::Value>,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,

}

/// Response for the getnodeaddresses RPC call.
///
/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnodeaddressesResponse {

    pub result: serde_json::Value,

}

/// Response for the getpeerinfo RPC call.
///
/// Returns data about each connected network peer as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetpeerinfoResponse {

    pub result: serde_json::Value,

}

/// Response for the listbanned RPC call.
///
/// List all manually banned IPs/Subnets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListbannedResponse {

    pub result: serde_json::Value,

}

/// Response for the setnetworkactive RPC call.
///
/// Disable/enable all p2p network activity.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

