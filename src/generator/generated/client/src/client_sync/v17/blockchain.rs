/// Implements Bitcoin Core JSON-RPC API method `dumptxoutset` for version v17
///
/// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[macro_export]
macro_rules! impl_client_v17__dumptxoutset {
    () => {
        impl Client {
            pub fn dumptxoutset(&self, path: String, type: Option<String>, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(path)?];
                if let Some(type) = type {
                    params.push(into_json(type)?);
                }
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("dumptxoutset", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getbestblockhash` for version v17
///
/// Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[macro_export]
macro_rules! impl_client_v17__getbestblockhash {
    () => {
        impl Client {
            pub fn getbestblockhash(&self) -> Result<hex> {
                self.call("getbestblockhash", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblock` for version v17
///
/// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
#[macro_export]
macro_rules! impl_client_v17__getblock {
    () => {
        impl Client {
            pub fn getblock(&self, blockhash: String, verbosity: Option<i64>) -> Result<hex> {
                let mut params = vec![into_json(blockhash)?];
                if let Some(verbosity) = verbosity {
                    params.push(into_json(verbosity)?);
                }
                self.call("getblock", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockchaininfo` for version v17
///
/// Returns an object containing various state info regarding blockchain processing.
#[macro_export]
macro_rules! impl_client_v17__getblockchaininfo {
    () => {
        impl Client {
            pub fn getblockchaininfo(&self) -> Result<serde_json::Value> {
                self.call("getblockchaininfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockcount` for version v17
///
/// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
#[macro_export]
macro_rules! impl_client_v17__getblockcount {
    () => {
        impl Client {
            pub fn getblockcount(&self) -> Result<number> {
                self.call("getblockcount", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockfilter` for version v17
///
/// Retrieve a BIP 157 content filter for a particular block.
#[macro_export]
macro_rules! impl_client_v17__getblockfilter {
    () => {
        impl Client {
            pub fn getblockfilter(&self, blockhash: String, filtertype: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(blockhash)?];
                if let Some(filtertype) = filtertype {
                    params.push(into_json(filtertype)?);
                }
                self.call("getblockfilter", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockfrompeer` for version v17
///
/// Attempt to fetch block from a given peer.
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// Returns an empty JSON object if the request was successfully scheduled.
#[macro_export]
macro_rules! impl_client_v17__getblockfrompeer {
    () => {
        impl Client {
            pub fn getblockfrompeer(&self, blockhash: String, peer_id: i64) -> Result<serde_json::Value> {
                self.call("getblockfrompeer", &[into_json(blockhash)?, into_json(peer_id)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockhash` for version v17
///
/// Returns hash of block in best-block-chain at height provided.
#[macro_export]
macro_rules! impl_client_v17__getblockhash {
    () => {
        impl Client {
            pub fn getblockhash(&self, height: i64) -> Result<hex> {
                self.call("getblockhash", &[into_json(height)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockheader` for version v17
///
/// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
#[macro_export]
macro_rules! impl_client_v17__getblockheader {
    () => {
        impl Client {
            pub fn getblockheader(&self, blockhash: String, verbose: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(blockhash)?];
                if let Some(verbose) = verbose {
                    params.push(into_json(verbose)?);
                }
                self.call("getblockheader", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getblockstats` for version v17
///
/// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
#[macro_export]
macro_rules! impl_client_v17__getblockstats {
    () => {
        impl Client {
            pub fn getblockstats(&self, hash_or_height: i64, stats: Option<Vec<String>>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(hash_or_height)?];
                if let Some(stats) = stats {
                    params.push(into_json(stats)?);
                }
                self.call("getblockstats", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getchainstates` for version v17
///
/// Return information about chainstates.
#[macro_export]
macro_rules! impl_client_v17__getchainstates {
    () => {
        impl Client {
            pub fn getchainstates(&self) -> Result<serde_json::Value> {
                self.call("getchainstates", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getchaintips` for version v17
///
/// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
#[macro_export]
macro_rules! impl_client_v17__getchaintips {
    () => {
        impl Client {
            pub fn getchaintips(&self) -> Result<Vec<serde_json::Value>> {
                self.call("getchaintips", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getchaintxstats` for version v17
///
/// Compute statistics about the total number and rate of transactions in the chain.
#[macro_export]
macro_rules! impl_client_v17__getchaintxstats {
    () => {
        impl Client {
            pub fn getchaintxstats(&self, nblocks: Option<i64>, blockhash: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(nblocks) = nblocks {
                    params.push(into_json(nblocks)?);
                }
                if let Some(blockhash) = blockhash {
                    params.push(into_json(blockhash)?);
                }
                self.call("getchaintxstats", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdeploymentinfo` for version v17
///
/// Returns an object containing various state info regarding deployments of consensus changes.
#[macro_export]
macro_rules! impl_client_v17__getdeploymentinfo {
    () => {
        impl Client {
            pub fn getdeploymentinfo(&self, blockhash: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(blockhash) = blockhash {
                    params.push(into_json(blockhash)?);
                }
                self.call("getdeploymentinfo", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdifficulty` for version v17
///
/// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
#[macro_export]
macro_rules! impl_client_v17__getdifficulty {
    () => {
        impl Client {
            pub fn getdifficulty(&self) -> Result<number> {
                self.call("getdifficulty", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolancestors` for version v17
///
/// If txid is in the mempool, returns all in-mempool ancestors.
#[macro_export]
macro_rules! impl_client_v17__getmempoolancestors {
    () => {
        impl Client {
            pub fn getmempoolancestors(&self, txid: String, verbose: Option<bool>) -> Result<Vec<Hex>> {
                let mut params = vec![into_json(txid)?];
                if let Some(verbose) = verbose {
                    params.push(into_json(verbose)?);
                }
                self.call("getmempoolancestors", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempooldescendants` for version v17
///
/// If txid is in the mempool, returns all in-mempool descendants.
#[macro_export]
macro_rules! impl_client_v17__getmempooldescendants {
    () => {
        impl Client {
            pub fn getmempooldescendants(&self, txid: String, verbose: Option<bool>) -> Result<Vec<Hex>> {
                let mut params = vec![into_json(txid)?];
                if let Some(verbose) = verbose {
                    params.push(into_json(verbose)?);
                }
                self.call("getmempooldescendants", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolentry` for version v17
///
/// Returns mempool data for given transaction
#[macro_export]
macro_rules! impl_client_v17__getmempoolentry {
    () => {
        impl Client {
            pub fn getmempoolentry(&self, txid: String) -> Result<serde_json::Value> {
                self.call("getmempoolentry", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmempoolinfo` for version v17
///
/// Returns details on the active state of the TX memory pool.
#[macro_export]
macro_rules! impl_client_v17__getmempoolinfo {
    () => {
        impl Client {
            pub fn getmempoolinfo(&self) -> Result<serde_json::Value> {
                self.call("getmempoolinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawmempool` for version v17
///
/// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
#[macro_export]
macro_rules! impl_client_v17__getrawmempool {
    () => {
        impl Client {
            pub fn getrawmempool(&self, verbose: Option<bool>, mempool_sequence: Option<bool>) -> Result<Vec<Hex>> {
                let mut params = vec![];
                if let Some(verbose) = verbose {
                    params.push(into_json(verbose)?);
                }
                if let Some(mempool_sequence) = mempool_sequence {
                    params.push(into_json(mempool_sequence)?);
                }
                self.call("getrawmempool", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxout` for version v17
///
/// Returns details about an unspent transaction output.
#[macro_export]
macro_rules! impl_client_v17__gettxout {
    () => {
        impl Client {
            pub fn gettxout(&self, txid: String, n: i64, include_mempool: Option<bool>) -> Result<()> {
                let mut params = vec![into_json(txid)?, into_json(n)?];
                if let Some(include_mempool) = include_mempool {
                    params.push(into_json(include_mempool)?);
                }
                self.call("gettxout", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxoutproof` for version v17
///
/// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
#[macro_export]
macro_rules! impl_client_v17__gettxoutproof {
    () => {
        impl Client {
            pub fn gettxoutproof(&self, txids: Vec<String>, blockhash: Option<String>) -> Result<String> {
                let mut params = vec![into_json(txids)?];
                if let Some(blockhash) = blockhash {
                    params.push(into_json(blockhash)?);
                }
                self.call("gettxoutproof", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxoutsetinfo` for version v17
///
/// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
#[macro_export]
macro_rules! impl_client_v17__gettxoutsetinfo {
    () => {
        impl Client {
            pub fn gettxoutsetinfo(&self, hash_type: Option<String>, hash_or_height: Option<i64>, use_index: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(hash_type) = hash_type {
                    params.push(into_json(hash_type)?);
                }
                if let Some(hash_or_height) = hash_or_height {
                    params.push(into_json(hash_or_height)?);
                }
                if let Some(use_index) = use_index {
                    params.push(into_json(use_index)?);
                }
                self.call("gettxoutsetinfo", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettxspendingprevout` for version v17
///
/// Scans the mempool to find transactions spending any of the given outputs
#[macro_export]
macro_rules! impl_client_v17__gettxspendingprevout {
    () => {
        impl Client {
            pub fn gettxspendingprevout(&self, outputs: Vec<Output>) -> Result<Vec<serde_json::Value>> {
                self.call("gettxspendingprevout", &[into_json(outputs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importmempool` for version v17
///
/// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
#[macro_export]
macro_rules! impl_client_v17__importmempool {
    () => {
        impl Client {
            pub fn importmempool(&self, filepath: String, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(filepath)?];
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("importmempool", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `loadtxoutset` for version v17
///
/// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
#[macro_export]
macro_rules! impl_client_v17__loadtxoutset {
    () => {
        impl Client {
            pub fn loadtxoutset(&self, path: String) -> Result<serde_json::Value> {
                self.call("loadtxoutset", &[into_json(path)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `preciousblock` for version v17
///
/// Treats a block as if it were received before others with the same work.
/// A later preciousblock call can override the effect of an earlier one.
/// The effects of preciousblock are not retained across restarts.
#[macro_export]
macro_rules! impl_client_v17__preciousblock {
    () => {
        impl Client {
            pub fn preciousblock(&self, blockhash: String) -> Result<()> {
                self.call("preciousblock", &[into_json(blockhash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `pruneblockchain` for version v17
///
/// 
#[macro_export]
macro_rules! impl_client_v17__pruneblockchain {
    () => {
        impl Client {
            pub fn pruneblockchain(&self, height: i64) -> Result<number> {
                self.call("pruneblockchain", &[into_json(height)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `savemempool` for version v17
///
/// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
#[macro_export]
macro_rules! impl_client_v17__savemempool {
    () => {
        impl Client {
            pub fn savemempool(&self) -> Result<serde_json::Value> {
                self.call("savemempool", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `scanblocks` for version v17
///
/// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[macro_export]
macro_rules! impl_client_v17__scanblocks {
    () => {
        impl Client {
            pub fn scanblocks(&self, action: String, scanobjects: Option<Vec<String>>, start_height: Option<i64>, stop_height: Option<i64>, filtertype: Option<String>, options: Option<serde_json::Value>) -> Result<()> {
                let mut params = vec![into_json(action)?];
                if let Some(scanobjects) = scanobjects {
                    params.push(into_json(scanobjects)?);
                }
                if let Some(start_height) = start_height {
                    params.push(into_json(start_height)?);
                }
                if let Some(stop_height) = stop_height {
                    params.push(into_json(stop_height)?);
                }
                if let Some(filtertype) = filtertype {
                    params.push(into_json(filtertype)?);
                }
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("scanblocks", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `scantxoutset` for version v17
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
#[macro_export]
macro_rules! impl_client_v17__scantxoutset {
    () => {
        impl Client {
            pub fn scantxoutset(&self, action: String, scanobjects: Option<Vec<String>>) -> Result<()> {
                let mut params = vec![into_json(action)?];
                if let Some(scanobjects) = scanobjects {
                    params.push(into_json(scanobjects)?);
                }
                self.call("scantxoutset", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `verifychain` for version v17
///
/// Verifies blockchain database.
#[macro_export]
macro_rules! impl_client_v17__verifychain {
    () => {
        impl Client {
            pub fn verifychain(&self, checklevel: Option<i64>, nblocks: Option<i64>) -> Result<bool> {
                let mut params = vec![];
                if let Some(checklevel) = checklevel {
                    params.push(into_json(checklevel)?);
                }
                if let Some(nblocks) = nblocks {
                    params.push(into_json(nblocks)?);
                }
                self.call("verifychain", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `verifytxoutproof` for version v17
///
/// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
#[macro_export]
macro_rules! impl_client_v17__verifytxoutproof {
    () => {
        impl Client {
            pub fn verifytxoutproof(&self, proof: String) -> Result<Vec<Hex>> {
                self.call("verifytxoutproof", &[into_json(proof)?])
            }
        }
    };
}

