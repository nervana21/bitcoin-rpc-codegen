use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the dumptxoutset RPC call.
///
/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DumptxoutsetResponse {
    /// the number of coins written in the snapshot
    pub coins_written: f64,
    /// the hash of the base of the snapshot
    pub base_hash: Hex,
    /// the height of the base of the snapshot
    pub base_height: f64,
    /// the absolute path that the snapshot was written to
    pub path: String,
    /// the hash of the UTXO set contents
    pub txoutset_hash: Hex,
    /// the number of transactions in the chain up to and including the base block
    pub nchaintx: f64,

}

/// Response for the getbestblockhash RPC call.
///
/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbestblockhashResponse {
    pub result: hex,
}

/// Response for the getblock RPC call.
///
/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockResponse {
    pub result: hex,
}

/// Response for the getblockchaininfo RPC call.
///
/// Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockchaininfoResponse {
    /// current network name (main, test, testnet4, signet, regtest)
    pub chain: String,
    /// the height of the most-work fully-validated chain. The genesis block has height 0
    pub blocks: f64,
    /// the current number of headers we have validated
    pub headers: f64,
    /// the hash of the currently best block
    pub bestblockhash: String,
    /// the current difficulty
    pub difficulty: f64,
    /// The block time expressed in UNIX epoch time
    pub time: Time,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: Time,
    /// estimate of verification progress [0..1]
    pub verificationprogress: f64,
    /// (debug information) estimate of whether this node is in Initial Block Download mode
    pub initialblockdownload: bool,
    /// total amount of work in active chain, in hexadecimal
    pub chainwork: Hex,
    /// the estimated size of the block and undo files on disk
    pub size_on_disk: f64,
    /// if the blocks are subject to pruning
    pub pruned: bool,
    /// height of the last block pruned, plus one (only present if pruning is enabled)
    pub pruneheight: f64,
    /// whether automatic pruning is enabled (only present if pruning is enabled)
    pub automatic_pruning: bool,
    /// the target size used by pruning (only present if automatic pruning is enabled)
    pub prune_target_size: f64,
    /// any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
    pub warnings: Vec<String>,

}

/// Response for the getblockcount RPC call.
///
/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockcountResponse {
    pub result: number,
}

/// Response for the getblockfilter RPC call.
///
/// Retrieve a BIP 157 content filter for a particular block.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockfilterResponse {
    /// the hex-encoded filter data
    pub filter: Hex,
    /// the hex-encoded filter header
    pub header: Hex,

}

/// Response for the getblockfrompeer RPC call.
///
/// Attempt to fetch block from a given peer.
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// Returns an empty JSON object if the request was successfully scheduled.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ObjectResponse {
    pub result: serde_json::Value,
}

/// Response for the getblockhash RPC call.
///
/// Returns hash of block in best-block-chain at height provided.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockhashResponse {
    pub result: hex,
}

/// Response for the getblockheader RPC call.
///
/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockheaderResponse {
    /// the block hash (same as provided)
    pub hash: Hex,
    /// The number of confirmations, or -1 if the block is not on the main chain
    pub confirmations: f64,
    /// The block height or index
    pub height: f64,
    /// The block version
    pub version: f64,
    /// The block version formatted in hexadecimal
    pub versionhex: Hex,
    /// The merkle root
    pub merkleroot: Hex,
    /// The block time expressed in UNIX epoch time
    pub time: Time,
    /// The median block time expressed in UNIX epoch time
    pub mediantime: Time,
    /// The nonce
    pub nonce: f64,
    /// The bits
    pub bits: Hex,
    /// The difficulty
    pub difficulty: f64,
    /// Expected number of hashes required to produce the current chain
    pub chainwork: Hex,
    /// The number of transactions in the block
    pub ntx: f64,
    /// The hash of the previous block (if available)
    pub previousblockhash: Hex,
    /// The hash of the next block (if available)
    pub nextblockhash: Hex,

}

/// Response for the getblockstats RPC call.
///
/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockstatsResponse {
    /// Average fee in the block
    pub avgfee: f64,
    /// Average feerate (in satoshis per virtual byte)
    pub avgfeerate: f64,
    /// Average transaction size
    pub avgtxsize: f64,
    /// The block hash (to check for potential reorgs)
    pub blockhash: Hex,
    /// Feerates at the 10th, 25th, 50th, 75th, and 90th percentile weight unit (in satoshis per virtual byte)
    pub feerate_percentiles: Vec<f64>,
    /// The height of the block
    pub height: f64,
    /// The number of inputs (excluding coinbase)
    pub ins: f64,
    /// Maximum fee in the block
    pub maxfee: f64,
    /// Maximum feerate (in satoshis per virtual byte)
    pub maxfeerate: f64,
    /// Maximum transaction size
    pub maxtxsize: f64,
    /// Truncated median fee in the block
    pub medianfee: f64,
    /// The block median time past
    pub mediantime: f64,
    /// Truncated median transaction size
    pub mediantxsize: f64,
    /// Minimum fee in the block
    pub minfee: f64,
    /// Minimum feerate (in satoshis per virtual byte)
    pub minfeerate: f64,
    /// Minimum transaction size
    pub mintxsize: f64,
    /// The number of outputs
    pub outs: f64,
    /// The block subsidy
    pub subsidy: f64,
    /// Total size of all segwit transactions
    pub swtotal_size: f64,
    /// Total weight of all segwit transactions
    pub swtotal_weight: f64,
    /// The number of segwit transactions
    pub swtxs: f64,
    /// The block time
    pub time: f64,
    /// Total amount in all outputs (excluding coinbase and thus reward [ie subsidy + totalfee])
    pub total_out: f64,
    /// Total size of all non-coinbase transactions
    pub total_size: f64,
    /// Total weight of all non-coinbase transactions
    pub total_weight: f64,
    /// The fee total
    pub totalfee: f64,
    /// The number of transactions (including coinbase)
    pub txs: f64,
    /// The increase/decrease in the number of unspent outputs (not discounting op_return and similar)
    pub utxo_increase: f64,
    /// The increase/decrease in size for the utxo index (not discounting op_return and similar)
    pub utxo_size_inc: f64,
    /// The increase/decrease in the number of unspent outputs, not counting unspendables
    pub utxo_increase_actual: f64,
    /// The increase/decrease in size for the utxo index, not counting unspendables
    pub utxo_size_inc_actual: f64,

}

/// Response for the getchainstates RPC call.
///
/// Return information about chainstates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchainstatesResponse {
    /// the number of headers seen so far
    pub headers: f64,
    /// list of the chainstates ordered by work, with the most-work (active) chainstate last
    pub chainstates: Vec<serde_json::Value>,

}

/// Response for the getchaintips RPC call.
///
/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchaintipsResponse {

    pub result: serde_json::Value,

}

/// Response for the getchaintxstats RPC call.
///
/// Compute statistics about the total number and rate of transactions in the chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchaintxstatsResponse {
    /// The timestamp for the final block in the window, expressed in UNIX epoch time
    pub time: Time,
    /// The total number of transactions in the chain up to that point, if known. It may be unknown when using assumeutxo.
    pub txcount: f64,
    /// The hash of the final block in the window
    pub window_final_block_hash: Hex,
    /// The height of the final block in the window.
    pub window_final_block_height: f64,
    /// Size of the window in number of blocks
    pub window_block_count: f64,
    /// The elapsed time in the window in seconds. Only returned if "window_block_count" is > 0
    pub window_interval: f64,
    /// The number of transactions in the window. Only returned if "window_block_count" is > 0 and if txcount exists for the start and end of the window.
    pub window_tx_count: f64,
    /// The average rate of transactions per second in the window. Only returned if "window_interval" is > 0 and if window_tx_count exists.
    pub txrate: f64,

}

/// Response for the getdeploymentinfo RPC call.
///
/// Returns an object containing various state info regarding deployments of consensus changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdeploymentinfoResponse {
    /// requested block hash (or tip)
    pub hash: String,
    /// requested block height (or tip)
    pub height: f64,

    pub deployments: object_dynamic,

}

/// Response for the getdifficulty RPC call.
///
/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdifficultyResponse {
    pub result: number,
}

/// Response for the getmempoolancestors RPC call.
///
/// If txid is in the mempool, returns all in-mempool ancestors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolancestorsResponse {
    /// The transaction id of an in-mempool ancestor transaction
    pub result: Hex,

}

/// Response for the getmempooldescendants RPC call.
///
/// If txid is in the mempool, returns all in-mempool descendants.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempooldescendantsResponse {
    /// The transaction id of an in-mempool descendant transaction
    pub result: Hex,

}

/// Response for the getmempoolentry RPC call.
///
/// Returns mempool data for given transaction
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolentryResponse {
    /// virtual transaction size as defined in BIP 141. This is different from actual serialized size for witness transactions as witness data is discounted.
    pub vsize: f64,
    /// transaction weight as defined in BIP 141.
    pub weight: f64,
    /// local time transaction entered pool in seconds since 1 Jan 1970 GMT
    pub time: Time,
    /// block height when transaction entered pool
    pub height: f64,
    /// number of in-mempool descendant transactions (including this one)
    pub descendantcount: f64,
    /// virtual transaction size of in-mempool descendants (including this one)
    pub descendantsize: f64,
    /// number of in-mempool ancestor transactions (including this one)
    pub ancestorcount: f64,
    /// virtual transaction size of in-mempool ancestors (including this one)
    pub ancestorsize: f64,
    /// hash of serialized transaction, including witness data
    pub wtxid: Hex,

    pub fees: serde_json::Fees,
    /// unconfirmed transactions used as inputs for this transaction
    pub depends: Vec<Hex>,
    /// unconfirmed transactions spending outputs from this transaction
    pub spentby: Vec<Hex>,
    /// Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability. (DEPRECATED)
    pub bip125_replaceable: bool,
    /// Whether this transaction is currently unbroadcast (initial broadcast not yet acknowledged by any peers)
    pub unbroadcast: bool,

}

/// Response for the getmempoolinfo RPC call.
///
/// Returns details on the active state of the TX memory pool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    /// True if the initial load attempt of the persisted mempool finished
    pub loaded: bool,
    /// Current tx count
    pub size: f64,
    /// Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
    pub bytes: f64,
    /// Total memory usage for the mempool
    pub usage: f64,
    /// Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
    pub total_fee: Amount,
    /// Maximum memory usage for the mempool
    pub maxmempool: f64,
    /// Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
    pub mempoolminfee: Amount,
    /// Current minimum relay fee for transactions
    pub minrelaytxfee: Amount,
    /// minimum fee rate increment for mempool limiting or replacement in BTC/kvB
    pub incrementalrelayfee: f64,
    /// Current number of transactions that haven't passed initial broadcast yet
    pub unbroadcastcount: f64,
    /// True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
    pub fullrbf: bool,

}

/// Response for the getrawmempool RPC call.
///
/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrawmempoolResponse {
    /// The transaction id
    pub result: Hex,

}

/// Response for the gettxoutproof RPC call.
///
/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the gettxoutsetinfo RPC call.
///
/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxoutsetinfoResponse {
    /// The block height (index) of the returned statistics
    pub height: f64,
    /// The hash of the block at which these statistics are calculated
    pub bestblock: Hex,
    /// The number of unspent transaction outputs
    pub txouts: f64,
    /// Database-independent, meaningless metric indicating the UTXO set size
    pub bogosize: f64,
    /// The serialized hash (only present if 'hash_serialized_3' hash_type is chosen)
    pub hash_serialized_3: Hex,
    /// The serialized hash (only present if 'muhash' hash_type is chosen)
    pub muhash: Hex,
    /// The number of transactions with unspent outputs (not available when coinstatsindex is used)
    pub transactions: f64,
    /// The estimated size of the chainstate on disk (not available when coinstatsindex is used)
    pub disk_size: f64,
    /// The total amount of coins in the UTXO set
    pub total_amount: Amount,
    /// The total amount of coins permanently excluded from the UTXO set (only available if coinstatsindex is used)
    pub total_unspendable_amount: Amount,
    /// Info on amounts in the block at this block height (only available if coinstatsindex is used)
    pub block_info: serde_json::Block_info,

}

/// Response for the gettxspendingprevout RPC call.
///
/// Scans the mempool to find transactions spending any of the given outputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettxspendingprevoutResponse {

    pub result: serde_json::Value,

}

/// Response for the importmempool RPC call.
///
/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ObjectResponse {
    pub result: serde_json::Value,
}

/// Response for the loadtxoutset RPC call.
///
/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoadtxoutsetResponse {
    /// the number of coins loaded from the snapshot
    pub coins_loaded: f64,
    /// the hash of the base of the snapshot
    pub tip_hash: Hex,
    /// the height of the base of the snapshot
    pub base_height: f64,
    /// the absolute path that the snapshot was loaded from
    pub path: String,

}

/// Response for the pruneblockchain RPC call.
///

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PruneblockchainResponse {
    pub result: number,
}

/// Response for the savemempool RPC call.
///
/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavemempoolResponse {
    /// the directory and file where the mempool was saved
    pub filename: String,

}

/// Response for the scantxoutset RPC call.
///
/// Scans the unspent transaction output set for entries that match certain output descriptors.
/// Examples of output descriptors are:
/// addr(<address>)                      Outputs whose output script corresponds to the specified address (does not include P2PK)
/// raw(<hex script>)                    Outputs whose output script equals the specified hex-encoded bytes
/// combo(<pubkey>)                      P2PK, P2PKH, P2WPKH, and P2SH-P2WPKH outputs for the given pubkey
/// pkh(<pubkey>)                        P2PKH outputs for the given pubkey
/// sh(multi(<n>,<pubkey>,<pubkey>,...)) P2SH-multisig outputs for the given threshold and pubkeys
/// tr(<pubkey>)                         P2TR
/// tr(<pubkey>,{pk(<pubkey>)})          P2TR with single fallback pubkey in tapscript
/// rawtr(<pubkey>)                      P2TR with the specified key as output key rather than inner
/// wsh(and_v(v:pk(<pubkey>),after(2)))  P2WSH miniscript with mandatory pubkey and a timelock
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", and optionally ending in "/*" (unhardened), or "/*'" or "/*h" (hardened) to specify all
/// unhardened or hardened child keys.
/// In the latter case, a range needs to be specified by below if different from 1000.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ScantxoutsetResponse {
    /// Whether the scan was completed
    pub success: bool,
    /// The number of unspent transaction outputs scanned
    pub txouts: f64,
    /// The block height at which the scan was done
    pub height: f64,
    /// The hash of the block at the tip of the chain
    pub bestblock: Hex,

    pub unspents: Vec<serde_json::Value>,
    /// The total amount of all found unspent outputs in BTC
    pub total_amount: Amount,

}

/// Response for the verifychain RPC call.
///
/// Verifies blockchain database.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

/// Response for the verifytxoutproof RPC call.
///
/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct VerifytxoutproofResponse {
    /// The txid(s) which the proof commits to, or empty array if the proof cannot be validated.
    pub txid: Hex,

}

