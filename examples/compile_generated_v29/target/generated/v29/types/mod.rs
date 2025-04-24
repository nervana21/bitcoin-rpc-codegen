use serde::{Deserialize, Serialize};

/// Response for the AbortrescanResponse RPC call.
/// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AbortrescanResponse {
    pub result: bool,
}

/// Response for the CreaterawtransactionResponse RPC call.
/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreaterawtransactionResponse {
    /// it is not stored in the wallet or transmitted to the network.
    pub result: String,
}

/// Response for the EnumeratesignersResponse RPC call.
/// Returns a list of external signers from -signer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct EnumeratesignersResponse {
    /// [                 (json array)
    pub signers: String,
    /// }
    pub result: String,
}

/// Response for the GetaddednodeinfoResponse RPC call.
/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddednodeinfoResponse {
    pub result: String,
}

/// Response for the GetaddrmaninfoResponse RPC call.
/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    /// {      the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
    pub network: serde_json::Value,
    /// }
    pub result: String,
}

/// Response for the GetbalanceResponse RPC call.
/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalanceResponse {
    /// thus affected by options which limit spendability such as -spendzeroconfchange.
    pub result: String,
}

/// Response for the GetbalancesResponse RPC call.
/// Returns an object with all balances in BTC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    /// {                     balances from outputs that the wallet can sign
    pub mine: serde_json::Value,
    /// }
    pub result: String,
}

/// Response for the GetbestblockhashResponse RPC call.
/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbestblockhashResponse {
    pub result: String,
}

/// Response for the GetblockchaininfoResponse RPC call.
/// Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockchaininfoResponse {
    /// "str",                         current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// }
    pub result: String,
}

/// Response for the GetblockcountResponse RPC call.
/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockcountResponse {
    /// n     The current block count
    pub result: f64,
}

/// Response for the GetchainstatesResponse RPC call.
/// Return information about chainstates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchainstatesResponse {
    /// n,                        the number of headers seen so far
    pub headers: f64,
    /// }
    pub result: String,
}

/// Response for the GetchaintipsResponse RPC call.
/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchaintipsResponse {
    /// {
    pub result: serde_json::Value,
}

/// Response for the GetconnectioncountResponse RPC call.
/// Returns the number of connections to other nodes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetconnectioncountResponse {
    pub result: f64,
}

/// Response for the GetdifficultyResponse RPC call.
/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdifficultyResponse {
    pub result: f64,
}

/// Response for the GetmempoolinfoResponse RPC call.
/// Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    /// true|false,         True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// }
    pub result: String,
}

/// Response for the GetmininginfoResponse RPC call.
/// Returns a json object containing mining-related information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    /// n,                   The current block
    pub blocks: f64,
    /// }
    pub result: String,
}

/// Response for the GetnettotalsResponse RPC call.
/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnettotalsResponse {
    /// {
    pub result: serde_json::Value,
}

/// Response for the GetnetworkhashpsResponse RPC call.
/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkhashpsResponse {
    /// Pass in [height] to estimate the network speed at the time when a certain block was found.
    pub result: String,
}

/// Response for the GetnetworkinfoResponse RPC call.
/// Returns an object containing various state info regarding P2P networking.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    /// n,                                      the server version
    pub version: f64,
    /// }
    pub result: String,
}

/// Response for the GetnewaddressResponse RPC call.
/// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnewaddressResponse {
    /// so payments received with the address will be associated with 'label'.
    pub result: String,
}

/// Response for the GetpeerinfoResponse RPC call.
/// Returns data about each connected network peer as a json array of objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetpeerinfoResponse {
    /// {
    pub result: serde_json::Value,
}

/// Response for the GetprioritisedtransactionsResponse RPC call.
/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetprioritisedtransactionsResponse {
    /// {
    pub transactionid: serde_json::Value,
    /// }
    pub result: String,
}

/// Response for the GetrawchangeaddressResponse RPC call.
/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawchangeaddressResponse {
    pub result: String,
}

/// Response for the GetrawmempoolResponse RPC call.
/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawmempoolResponse {
    pub result: String,
}

/// Response for the GetrpcinfoResponse RPC call.
/// Returns details of the RPC server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    /// [    (json array) All active commands
    pub active_commands: String,
    /// }
    pub result: String,
}

/// Response for the GettxoutproofResponse RPC call.
/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutproofResponse {
    /// unspent output in the utxo for this transaction. To make it always work,
    pub result: String,
}

/// Response for the GettxoutsetinfoResponse RPC call.
/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutsetinfoResponse {
    pub result: String,
}

/// Response for the GetunconfirmedbalanceResponse RPC call.
/// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetunconfirmedbalanceResponse {
    pub result: f64,
}

/// Response for the GetwalletinfoResponse RPC call.
/// Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetwalletinfoResponse {
    /// "str",                    the wallet name
    pub walletname: String,
    /// }
    pub result: String,
}

/// Response for the GetzmqnotificationsResponse RPC call.
/// Returns information about the active ZeroMQ notifications.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetzmqnotificationsResponse {
    /// {
    pub result: serde_json::Value,
}

/// Response for the ListaddressgroupingsResponse RPC call.
/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListaddressgroupingsResponse {
    /// [             (json array)
    pub result: String,
}

/// Response for the ListbannedResponse RPC call.
/// List all manually banned IPs/Subnets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListbannedResponse {
    /// {
    pub result: serde_json::Value,
}

/// Response for the ListlockunspentResponse RPC call.
/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListlockunspentResponse {
    /// [                      (json array)
    pub result: String,
}

/// Response for the ListunspentResponse RPC call.
/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListunspentResponse {
    /// Optionally filter to only include txouts paid to specified addresses.
    pub result: String,
}

/// Response for the ListwalletdirResponse RPC call.
/// Returns a list of wallets in the wallet directory.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletdirResponse {
    /// [          (json array)
    pub wallets: String,
    /// }
    pub result: String,
}

/// Response for the ListwalletsResponse RPC call.
/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletsResponse {
    /// [           (json array)
    pub result: String,
}

/// Response for the PingResponse RPC call.
/// Requests that a ping be sent to all other nodes, to measure ping time.
/// Results provided in getpeerinfo, pingtime and pingwait fields are decimal seconds.
/// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PingResponse {
    /// null
    pub result: String,
}

/// Response for the PsbtbumpfeeResponse RPC call.
/// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
/// Returns a PSBT instead of creating and signing a new transaction.
/// An opt-in RBF transaction with the given txid must be in the wallet.
/// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
/// It may add a new change output if one does not already exist.
/// All inputs in the original transaction will be included in the replacement transaction.
/// The command will fail if the wallet or mempool contains a transaction that spends one of T's outputs.
/// By default, the new fee will be calculated automatically using the estimatesmartfee RPC.
/// The user can specify a confirmation target for estimatesmartfee.
/// Alternatively, the user can specify a fee rate in sat/vB for the new transaction.
/// At a minimum, the new fee rate must be high enough to pay an additional new relay fee (incrementalfee
/// returned by getnetworkinfo) to enter the node's mempool.
/// * WARNING: before version 0.21, fee_rate was in BTC/kvB. As of 0.21, fee_rate is in sat/vB. *
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PsbtbumpfeeResponse {
    /// The command will pay the additional fee by reducing change outputs or adding inputs when necessary.
    pub result: String,
}

/// Response for the SavemempoolResponse RPC call.
/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavemempoolResponse {
    /// "str"      the directory and file where the mempool was saved
    pub filename: String,
    /// }
    pub result: String,
}

/// Response for the StopResponse RPC call.
/// Request a graceful shutdown of Bitcoin Core.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StopResponse {
    pub result: String,
}

/// Response for the TestmempoolacceptResponse RPC call.
/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TestmempoolacceptResponse {
    /// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
    pub result: String,
}

/// Response for the UptimeResponse RPC call.
/// Returns the total uptime of the server.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UptimeResponse {
    pub result: f64,
}
