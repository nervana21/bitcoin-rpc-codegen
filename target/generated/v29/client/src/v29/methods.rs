/// client impl for `abortrescan` RPC (v29)
macro_rules! impl_client_v29__abortrescan {
    () => {
        /// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
        /// Note: Use "getwalletinfo" to query the scanning progress.
        ///
        /// Result:
        /// true|false    (boolean) Whether the abort was successful
        ///
        /// Examples:
        ///
        /// Import a private key
        /// > bitcoin-cli importprivkey "mykey"
        ///
        /// Abort the running wallet rescan
        /// > bitcoin-cli abortrescan 
        ///
        /// As a JSON-RPC call
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "abortrescan", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn abortrescan(&self) -> RpcResult<AbortrescanResponse> {
            self.call("abortrescan", json!([]))
        }
    };
}


/// client impl for `clearbanned` RPC (v29)
macro_rules! impl_client_v29__clearbanned {
    () => {
        /// Clear all banned IPs.
        ///
        /// Result:
        /// null    (json null)
        ///
        /// Examples:
        /// > bitcoin-cli clearbanned 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "clearbanned", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn clearbanned(&self) -> RpcResult<ClearbannedResponse> {
            self.call("clearbanned", json!([]))
        }
    };
}


/// client impl for `getaddrmaninfo` RPC (v29)
macro_rules! impl_client_v29__getaddrmaninfo {
    () => {
        /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
        ///
        /// Result:
        /// {                   (json object) json object with network type as keys
        ///   "network" : {     (json object) the network (ipv4, ipv6, onion, i2p, cjdns, all_networks)
        ///     "new" : n,      (numeric) number of addresses in the new table, which represent potential peers the node has discovered but hasn't yet successfully connected to.
        ///     "tried" : n,    (numeric) number of addresses in the tried table, which represent peers the node has successfully connected to in the past.
        ///     "total" : n     (numeric) total number of addresses in both new/tried tables
        ///   },
        ///   ...
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getaddrmaninfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getaddrmaninfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getaddrmaninfo(&self) -> RpcResult<GetaddrmaninfoResponse> {
            self.call("getaddrmaninfo", json!([]))
        }
    };
}


/// client impl for `getbalances` RPC (v29)
macro_rules! impl_client_v29__getbalances {
    () => {
        /// Returns an object with all balances in BTC.
        ///
        /// Result:
        /// {                               (json object)
        ///   "mine" : {                    (json object) balances from outputs that the wallet can sign
        ///     "trusted" : n,              (numeric) trusted balance (outputs created by the wallet or confirmed outputs)
        ///     "untrusted_pending" : n,    (numeric) untrusted pending balance (outputs created by others that are in the mempool)
        ///     "immature" : n,             (numeric) balance from immature coinbase outputs
        ///     "used" : n                  (numeric, optional) (only present if avoid_reuse is set) balance from coins sent to addresses that were previously spent from (potentially privacy violating)
        ///   },
        ///   "watchonly" : {               (json object, optional) watchonly balances (not present if wallet does not watch anything)
        ///     "trusted" : n,              (numeric) trusted balance (outputs created by the wallet or confirmed outputs)
        ///     "untrusted_pending" : n,    (numeric) untrusted pending balance (outputs created by others that are in the mempool)
        ///     "immature" : n              (numeric) balance from immature coinbase outputs
        ///   },
        ///   "lastprocessedblock" : {      (json object) hash and height of the block this information was generated on
        ///     "hash" : "hex",             (string) hash of the block this information was generated on
        ///     "height" : n                (numeric) height of the block this information was generated on
        ///   }
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getbalances 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getbalances", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getbalances(&self) -> RpcResult<GetbalancesResponse> {
            self.call("getbalances", json!([]))
        }
    };
}


/// client impl for `getbestblockhash` RPC (v29)
macro_rules! impl_client_v29__getbestblockhash {
    () => {
        /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
        ///
        /// Result:
        /// "hex"    (string) the block hash, hex-encoded
        ///
        /// Examples:
        /// > bitcoin-cli getbestblockhash 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getbestblockhash", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getbestblockhash(&self) -> RpcResult<GetbestblockhashResponse> {
            self.call("getbestblockhash", json!([]))
        }
    };
}


/// client impl for `getblockchaininfo` RPC (v29)
macro_rules! impl_client_v29__getblockchaininfo {
    () => {
        /// Returns an object containing various state info regarding blockchain processing.
        ///
        /// Result:
        /// {                                         (json object)
        ///   "chain" : "str",                        (string) current network name (main, test, testnet4, signet, regtest)
        ///   "blocks" : n,                           (numeric) the height of the most-work fully-validated chain. The genesis block has height 0
        ///   "headers" : n,                          (numeric) the current number of headers we have validated
        ///   "bestblockhash" : "str",                (string) the hash of the currently best block
        ///   "bits" : "hex",                         (string) nBits: compact representation of the block difficulty target
        ///   "target" : "hex",                       (string) The difficulty target
        ///   "difficulty" : n,                       (numeric) the current difficulty
        ///   "time" : xxx,                           (numeric) The block time expressed in UNIX epoch time
        ///   "mediantime" : xxx,                     (numeric) The median block time expressed in UNIX epoch time
        ///   "verificationprogress" : n,             (numeric) estimate of verification progress [0..1]
        ///   "initialblockdownload" : true|false,    (boolean) (debug information) estimate of whether this node is in Initial Block Download mode
        ///   "chainwork" : "hex",                    (string) total amount of work in active chain, in hexadecimal
        ///   "size_on_disk" : n,                     (numeric) the estimated size of the block and undo files on disk
        ///   "pruned" : true|false,                  (boolean) if the blocks are subject to pruning
        ///   "pruneheight" : n,                      (numeric, optional) height of the last block pruned, plus one (only present if pruning is enabled)
        ///   "automatic_pruning" : true|false,       (boolean, optional) whether automatic pruning is enabled (only present if pruning is enabled)
        ///   "prune_target_size" : n,                (numeric, optional) the target size used by pruning (only present if automatic pruning is enabled)
        ///   "signet_challenge" : "hex",             (string, optional) the block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
        ///   "warnings" : [                          (json array) any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
        ///     "str",                                (string) warning
        ///     ...
        ///   ]
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getblockchaininfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getblockchaininfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getblockchaininfo(&self) -> RpcResult<GetblockchaininfoResponse> {
            self.call("getblockchaininfo", json!([]))
        }
    };
}


/// client impl for `getblockcount` RPC (v29)
macro_rules! impl_client_v29__getblockcount {
    () => {
        /// Returns the height of the most-work fully-validated chain.
        /// The genesis block has height 0.
        ///
        /// Result:
        /// n    (numeric) The current block count
        ///
        /// Examples:
        /// > bitcoin-cli getblockcount 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getblockcount", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getblockcount(&self) -> RpcResult<GetblockcountResponse> {
            self.call("getblockcount", json!([]))
        }
    };
}


/// client impl for `getchainstates` RPC (v29)
macro_rules! impl_client_v29__getchainstates {
    () => {
        /// Return information about chainstates.
        ///
        /// Result:
        /// {                                      (json object)
        ///   "headers" : n,                       (numeric) the number of headers seen so far
        ///   "chainstates" : [                    (json array) list of the chainstates ordered by work, with the most-work (active) chainstate last
        ///     {                                  (json object)
        ///       "blocks" : n,                    (numeric) number of blocks in this chainstate
        ///       "bestblockhash" : "hex",         (string) blockhash of the tip
        ///       "bits" : "hex",                  (string) nBits: compact representation of the block difficulty target
        ///       "target" : "hex",                (string) The difficulty target
        ///       "difficulty" : n,                (numeric) difficulty of the tip
        ///       "verificationprogress" : n,      (numeric) progress towards the network tip
        ///       "snapshot_blockhash" : "hex",    (string, optional) the base block of the snapshot this chainstate is based on, if any
        ///       "coins_db_cache_bytes" : n,      (numeric) size of the coinsdb cache
        ///       "coins_tip_cache_bytes" : n,     (numeric) size of the coinstip cache
        ///       "validated" : true|false         (boolean) whether the chainstate is fully validated. True if all blocks in the chainstate were validated, false if the chain is based on a snapshot and the snapshot has not yet been validated.
        ///     },
        ///     ...
        ///   ]
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getchainstates 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getchainstates", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getchainstates(&self) -> RpcResult<GetchainstatesResponse> {
            self.call("getchainstates", json!([]))
        }
    };
}


/// client impl for `getchaintips` RPC (v29)
macro_rules! impl_client_v29__getchaintips {
    () => {
        /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
        ///
        /// Result:
        /// [                        (json array)
        ///   {                      (json object)
        ///     "height" : n,        (numeric) height of the chain tip
        ///     "hash" : "hex",      (string) block hash of the tip
        ///     "branchlen" : n,     (numeric) zero for main chain, otherwise length of branch connecting the tip to the main chain
        ///     "status" : "str"     (string) status of the chain, "active" for the main chain
        ///                          Possible values for status:
        ///                          1.  "invalid"               This branch contains at least one invalid block
        ///                          2.  "headers-only"          Not all blocks for this branch are available, but the headers are valid
        ///                          3.  "valid-headers"         All blocks are available for this branch, but they were never fully validated
        ///                          4.  "valid-fork"            This branch is not part of the active chain, but is fully validated
        ///                          5.  "active"                This is the tip of the active main chain, which is certainly valid
        ///   },
        ///   ...
        /// ]
        ///
        /// Examples:
        /// > bitcoin-cli getchaintips 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getchaintips", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getchaintips(&self) -> RpcResult<GetchaintipsResponse> {
            self.call("getchaintips", json!([]))
        }
    };
}


/// client impl for `getconnectioncount` RPC (v29)
macro_rules! impl_client_v29__getconnectioncount {
    () => {
        /// Returns the number of connections to other nodes.
        ///
        /// Result:
        /// n    (numeric) The connection count
        ///
        /// Examples:
        /// > bitcoin-cli getconnectioncount 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getconnectioncount", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getconnectioncount(&self) -> RpcResult<GetconnectioncountResponse> {
            self.call("getconnectioncount", json!([]))
        }
    };
}


/// client impl for `getdifficulty` RPC (v29)
macro_rules! impl_client_v29__getdifficulty {
    () => {
        /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
        ///
        /// Result:
        /// n    (numeric) the proof-of-work difficulty as a multiple of the minimum difficulty.
        ///
        /// Examples:
        /// > bitcoin-cli getdifficulty 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getdifficulty", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getdifficulty(&self) -> RpcResult<GetdifficultyResponse> {
            self.call("getdifficulty", json!([]))
        }
    };
}


/// client impl for `getmempoolinfo` RPC (v29)
macro_rules! impl_client_v29__getmempoolinfo {
    () => {
        /// Returns details on the active state of the TX memory pool.
        ///
        /// Result:
        /// {                               (json object)
        ///   "loaded" : true|false,        (boolean) True if the initial load attempt of the persisted mempool finished
        ///   "size" : n,                   (numeric) Current tx count
        ///   "bytes" : n,                  (numeric) Sum of all virtual transaction sizes as defined in BIP 141. Differs from actual serialized size because witness data is discounted
        ///   "usage" : n,                  (numeric) Total memory usage for the mempool
        ///   "total_fee" : n,              (numeric) Total fees for the mempool in BTC, ignoring modified fees through prioritisetransaction
        ///   "maxmempool" : n,             (numeric) Maximum memory usage for the mempool
        ///   "mempoolminfee" : n,          (numeric) Minimum fee rate in BTC/kvB for tx to be accepted. Is the maximum of minrelaytxfee and minimum mempool fee
        ///   "minrelaytxfee" : n,          (numeric) Current minimum relay fee for transactions
        ///   "incrementalrelayfee" : n,    (numeric) minimum fee rate increment for mempool limiting or replacement in BTC/kvB
        ///   "unbroadcastcount" : n,       (numeric) Current number of transactions that haven't passed initial broadcast yet
        ///   "fullrbf" : true|false        (boolean) True if the mempool accepts RBF without replaceability signaling inspection (DEPRECATED)
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getmempoolinfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getmempoolinfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getmempoolinfo(&self) -> RpcResult<GetmempoolinfoResponse> {
            self.call("getmempoolinfo", json!([]))
        }
    };
}


/// client impl for `getmininginfo` RPC (v29)
macro_rules! impl_client_v29__getmininginfo {
    () => {
        /// Returns a json object containing mining-related information.
        ///
        /// Result:
        /// {                                (json object)
        ///   "blocks" : n,                  (numeric) The current block
        ///   "currentblockweight" : n,      (numeric, optional) The block weight (including reserved weight for block header, txs count and coinbase tx) of the last assembled block (only present if a block was ever assembled)
        ///   "currentblocktx" : n,          (numeric, optional) The number of block transactions (excluding coinbase) of the last assembled block (only present if a block was ever assembled)
        ///   "bits" : "hex",                (string) The current nBits, compact representation of the block difficulty target
        ///   "difficulty" : n,              (numeric) The current difficulty
        ///   "target" : "hex",              (string) The current target
        ///   "networkhashps" : n,           (numeric) The network hashes per second
        ///   "pooledtx" : n,                (numeric) The size of the mempool
        ///   "chain" : "str",               (string) current network name (main, test, testnet4, signet, regtest)
        ///   "signet_challenge" : "hex",    (string, optional) The block challenge (aka. block script), in hexadecimal (only present if the current network is a signet)
        ///   "next" : {                     (json object) The next block
        ///     "height" : n,                (numeric) The next height
        ///     "bits" : "hex",              (string) The next target nBits
        ///     "difficulty" : n,            (numeric) The next difficulty
        ///     "target" : "hex"             (string) The next target
        ///   },
        ///   "warnings" : [                 (json array) any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
        ///     "str",                       (string) warning
        ///     ...
        ///   ]
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getmininginfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getmininginfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getmininginfo(&self) -> RpcResult<GetmininginfoResponse> {
            self.call("getmininginfo", json!([]))
        }
    };
}


/// client impl for `getnettotals` RPC (v29)
macro_rules! impl_client_v29__getnettotals {
    () => {
        /// Returns information about network traffic, including bytes in, bytes out,
        /// and current system time.
        ///
        /// Result:
        /// {                                              (json object)
        ///   "totalbytesrecv" : n,                        (numeric) Total bytes received
        ///   "totalbytessent" : n,                        (numeric) Total bytes sent
        ///   "timemillis" : xxx,                          (numeric) Current system UNIX epoch time in milliseconds
        ///   "uploadtarget" : {                           (json object)
        ///     "timeframe" : n,                           (numeric) Length of the measuring timeframe in seconds
        ///     "target" : n,                              (numeric) Target in bytes
        ///     "target_reached" : true|false,             (boolean) True if target is reached
        ///     "serve_historical_blocks" : true|false,    (boolean) True if serving historical blocks
        ///     "bytes_left_in_cycle" : n,                 (numeric) Bytes left in current time cycle
        ///     "time_left_in_cycle" : n                   (numeric) Seconds left in current time cycle
        ///   }
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getnettotals 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getnettotals", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getnettotals(&self) -> RpcResult<GetnettotalsResponse> {
            self.call("getnettotals", json!([]))
        }
    };
}


/// client impl for `getnetworkinfo` RPC (v29)
macro_rules! impl_client_v29__getnetworkinfo {
    () => {
        /// Returns an object containing various state info regarding P2P networking.
        ///
        /// Result:
        /// {                                                    (json object)
        ///   "version" : n,                                     (numeric) the server version
        ///   "subversion" : "str",                              (string) the server subversion string
        ///   "protocolversion" : n,                             (numeric) the protocol version
        ///   "localservices" : "hex",                           (string) the services we offer to the network
        ///   "localservicesnames" : [                           (json array) the services we offer to the network, in human-readable form
        ///     "str",                                           (string) the service name
        ///     ...
        ///   ],
        ///   "localrelay" : true|false,                         (boolean) true if transaction relay is requested from peers
        ///   "timeoffset" : n,                                  (numeric) the time offset
        ///   "connections" : n,                                 (numeric) the total number of connections
        ///   "connections_in" : n,                              (numeric) the number of inbound connections
        ///   "connections_out" : n,                             (numeric) the number of outbound connections
        ///   "networkactive" : true|false,                      (boolean) whether p2p networking is enabled
        ///   "networks" : [                                     (json array) information per network
        ///     {                                                (json object)
        ///       "name" : "str",                                (string) network (ipv4, ipv6, onion, i2p, cjdns)
        ///       "limited" : true|false,                        (boolean) is the network limited using -onlynet?
        ///       "reachable" : true|false,                      (boolean) is the network reachable?
        ///       "proxy" : "str",                               (string) ("host:port") the proxy that is used for this network, or empty if none
        ///       "proxy_randomize_credentials" : true|false     (boolean) Whether randomized credentials are used
        ///     },
        ///     ...
        ///   ],
        ///   "relayfee" : n,                                    (numeric) minimum relay fee rate for transactions in BTC/kvB
        ///   "incrementalfee" : n,                              (numeric) minimum fee rate increment for mempool limiting or replacement in BTC/kvB
        ///   "localaddresses" : [                               (json array) list of local addresses
        ///     {                                                (json object)
        ///       "address" : "str",                             (string) network address
        ///       "port" : n,                                    (numeric) network port
        ///       "score" : n                                    (numeric) relative score
        ///     },
        ///     ...
        ///   ],
        ///   "warnings" : [                                     (json array) any network and blockchain warnings (run with `-deprecatedrpc=warnings` to return the latest warning as a single string)
        ///     "str",                                           (string) warning
        ///     ...
        ///   ]
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getnetworkinfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getnetworkinfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getnetworkinfo(&self) -> RpcResult<GetnetworkinfoResponse> {
            self.call("getnetworkinfo", json!([]))
        }
    };
}


/// client impl for `getpeerinfo` RPC (v29)
macro_rules! impl_client_v29__getpeerinfo {
    () => {
        /// Returns data about each connected network peer as a json array of objects.
        ///
        /// Result:
        /// [                                         (json array)
        ///   {                                       (json object)
        ///     "id" : n,                             (numeric) Peer index
        ///     "addr" : "str",                       (string) (host:port) The IP address and port of the peer
        ///     "addrbind" : "str",                   (string, optional) (ip:port) Bind address of the connection to the peer
        ///     "addrlocal" : "str",                  (string, optional) (ip:port) Local address as reported by the peer
        ///     "network" : "str",                    (string) Network (ipv4, ipv6, onion, i2p, cjdns, not_publicly_routable)
        ///     "mapped_as" : n,                      (numeric, optional) Mapped AS (Autonomous System) number at the end of the BGP route to the peer, used for diversifying
        ///                                           peer selection (only displayed if the -asmap config option is set)
        ///     "services" : "hex",                   (string) The services offered
        ///     "servicesnames" : [                   (json array) the services offered, in human-readable form
        ///       "str",                              (string) the service name if it is recognised
        ///       ...
        ///     ],
        ///     "relaytxes" : true|false,             (boolean) Whether we relay transactions to this peer
        ///     "lastsend" : xxx,                     (numeric) The UNIX epoch time of the last send
        ///     "lastrecv" : xxx,                     (numeric) The UNIX epoch time of the last receive
        ///     "last_transaction" : xxx,             (numeric) The UNIX epoch time of the last valid transaction received from this peer
        ///     "last_block" : xxx,                   (numeric) The UNIX epoch time of the last block received from this peer
        ///     "bytessent" : n,                      (numeric) The total bytes sent
        ///     "bytesrecv" : n,                      (numeric) The total bytes received
        ///     "conntime" : xxx,                     (numeric) The UNIX epoch time of the connection
        ///     "timeoffset" : n,                     (numeric) The time offset in seconds
        ///     "pingtime" : n,                       (numeric, optional) The last ping time in milliseconds (ms), if any
        ///     "minping" : n,                        (numeric, optional) The minimum observed ping time in milliseconds (ms), if any
        ///     "pingwait" : n,                       (numeric, optional) The duration in milliseconds (ms) of an outstanding ping (if non-zero)
        ///     "version" : n,                        (numeric) The peer version, such as 70001
        ///     "subver" : "str",                     (string) The string version
        ///     "inbound" : true|false,               (boolean) Inbound (true) or Outbound (false)
        ///     "bip152_hb_to" : true|false,          (boolean) Whether we selected peer as (compact blocks) high-bandwidth peer
        ///     "bip152_hb_from" : true|false,        (boolean) Whether peer selected us as (compact blocks) high-bandwidth peer
        ///     "startingheight" : n,                 (numeric) The starting height (block) of the peer
        ///     "presynced_headers" : n,              (numeric) The current height of header pre-synchronization with this peer, or -1 if no low-work sync is in progress
        ///     "synced_headers" : n,                 (numeric) The last header we have in common with this peer
        ///     "synced_blocks" : n,                  (numeric) The last block we have in common with this peer
        ///     "inflight" : [                        (json array)
        ///       n,                                  (numeric) The heights of blocks we're currently asking from this peer
        ///       ...
        ///     ],
        ///     "addr_relay_enabled" : true|false,    (boolean) Whether we participate in address relay with this peer
        ///     "addr_processed" : n,                 (numeric) The total number of addresses processed, excluding those dropped due to rate limiting
        ///     "addr_rate_limited" : n,              (numeric) The total number of addresses dropped due to rate limiting
        ///     "permissions" : [                     (json array) Any special permissions that have been granted to this peer
        ///       "str",                              (string) bloomfilter (allow requesting BIP37 filtered blocks and transactions),
        ///                                           noban (do not ban for misbehavior; implies download),
        ///                                           forcerelay (relay transactions that are already in the mempool; implies relay),
        ///                                           relay (relay even in -blocksonly mode, and unlimited transaction announcements),
        ///                                           mempool (allow requesting BIP35 mempool contents),
        ///                                           download (allow getheaders during IBD, no disconnect after maxuploadtarget limit),
        ///                                           addr (responses to GETADDR avoid hitting the cache and contain random records with the most up-to-date info).
        ///
        ///       ...
        ///     ],
        ///     "minfeefilter" : n,                   (numeric) The minimum fee rate for transactions this peer accepts
        ///     "bytessent_per_msg" : {               (json object)
        ///       "msg" : n,                          (numeric) The total bytes sent aggregated by message type
        ///                                           When a message type is not listed in this json object, the bytes sent are 0.
        ///                                           Only known message types can appear as keys in the object.
        ///       ...
        ///     },
        ///     "bytesrecv_per_msg" : {               (json object)
        ///       "msg" : n,                          (numeric) The total bytes received aggregated by message type
        ///                                           When a message type is not listed in this json object, the bytes received are 0.
        ///                                           Only known message types can appear as keys in the object and all bytes received
        ///                                           of unknown message types are listed under '*other*'.
        ///       ...
        ///     },
        ///     "connection_type" : "str",            (string) Type of connection: 
        ///                                           outbound-full-relay (default automatic connections),
        ///                                           block-relay-only (does not relay transactions or addresses),
        ///                                           inbound (initiated by the peer),
        ///                                           manual (added via addnode RPC or -addnode/-connect configuration options),
        ///                                           addr-fetch (short-lived automatic connection for soliciting addresses),
        ///                                           feeler (short-lived automatic connection for testing addresses).
        ///                                           Please note this output is unlikely to be stable in upcoming releases as we iterate to
        ///                                           best capture connection behaviors.
        ///     "transport_protocol_type" : "str",    (string) Type of transport protocol: 
        ///                                           detecting (peer could be v1 or v2),
        ///                                           v1 (plaintext transport protocol),
        ///                                           v2 (BIP324 encrypted transport protocol).
        ///
        ///     "session_id" : "str"                  (string) The session ID for this connection, or "" if there is none ("v2" transport protocol only).
        ///
        ///   },
        ///   ...
        /// ]
        ///
        /// Examples:
        /// > bitcoin-cli getpeerinfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getpeerinfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getpeerinfo(&self) -> RpcResult<GetpeerinfoResponse> {
            self.call("getpeerinfo", json!([]))
        }
    };
}


/// client impl for `getprioritisedtransactions` RPC (v29)
macro_rules! impl_client_v29__getprioritisedtransactions {
    () => {
        /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
        ///
        /// Result:
        /// {                                 (json object) prioritisation keyed by txid
        ///   "<transactionid>" : {           (json object)
        ///     "fee_delta" : n,              (numeric) transaction fee delta in satoshis
        ///     "in_mempool" : true|false,    (boolean) whether this transaction is currently in mempool
        ///     "modified_fee" : n            (numeric, optional) modified fee in satoshis. Only returned if in_mempool=true
        ///   },
        ///   ...
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getprioritisedtransactions 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getprioritisedtransactions", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getprioritisedtransactions(&self) -> RpcResult<GetprioritisedtransactionsResponse> {
            self.call("getprioritisedtransactions", json!([]))
        }
    };
}


/// client impl for `getrpcinfo` RPC (v29)
macro_rules! impl_client_v29__getrpcinfo {
    () => {
        /// Returns details of the RPC server.
        ///
        /// Result:
        /// {                          (json object)
        ///   "active_commands" : [    (json array) All active commands
        ///     {                      (json object) Information about an active command
        ///       "method" : "str",    (string) The name of the RPC command
        ///       "duration" : n       (numeric) The running time in microseconds
        ///     },
        ///     ...
        ///   ],
        ///   "logpath" : "str"        (string) The complete file path to the debug log
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getrpcinfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getrpcinfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getrpcinfo(&self) -> RpcResult<GetrpcinfoResponse> {
            self.call("getrpcinfo", json!([]))
        }
    };
}


/// client impl for `getunconfirmedbalance` RPC (v29)
macro_rules! impl_client_v29__getunconfirmedbalance {
    () => {
        /// DEPRECATED
        /// Identical to getbalances().mine.untrusted_pending
        ///
        /// Result:
        /// n    (numeric) The balance
        pub fn getunconfirmedbalance(&self) -> RpcResult<GetunconfirmedbalanceResponse> {
            self.call("getunconfirmedbalance", json!([]))
        }
    };
}


/// client impl for `getwalletinfo` RPC (v29)
macro_rules! impl_client_v29__getwalletinfo {
    () => {
        /// Returns an object containing various wallet state info.
        ///
        /// Result:
        /// {                                         (json object)
        ///   "walletname" : "str",                   (string) the wallet name
        ///   "walletversion" : n,                    (numeric) the wallet version
        ///   "format" : "str",                       (string) the database format (bdb or sqlite)
        ///   "balance" : n,                          (numeric) DEPRECATED. Identical to getbalances().mine.trusted
        ///   "unconfirmed_balance" : n,              (numeric) DEPRECATED. Identical to getbalances().mine.untrusted_pending
        ///   "immature_balance" : n,                 (numeric) DEPRECATED. Identical to getbalances().mine.immature
        ///   "txcount" : n,                          (numeric) the total number of transactions in the wallet
        ///   "keypoololdest" : xxx,                  (numeric, optional) the UNIX epoch time of the oldest pre-generated key in the key pool. Legacy wallets only.
        ///   "keypoolsize" : n,                      (numeric) how many new keys are pre-generated (only counts external keys)
        ///   "keypoolsize_hd_internal" : n,          (numeric, optional) how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
        ///   "unlocked_until" : xxx,                 (numeric, optional) the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
        ///   "paytxfee" : n,                         (numeric) the transaction fee configuration, set in BTC/kvB
        ///   "hdseedid" : "hex",                     (string, optional) the Hash160 of the HD seed (only present when HD is enabled)
        ///   "private_keys_enabled" : true|false,    (boolean) false if privatekeys are disabled for this wallet (enforced watch-only wallet)
        ///   "avoid_reuse" : true|false,             (boolean) whether this wallet tracks clean/dirty coins in terms of reuse
        ///   "scanning" : {                          (json object) current scanning details, or false if no scan is in progress
        ///     "duration" : n,                       (numeric) elapsed seconds since scan start
        ///     "progress" : n                        (numeric) scanning progress percentage [0.0, 1.0]
        ///   },
        ///   "descriptors" : true|false,             (boolean) whether this wallet uses descriptors for output script management
        ///   "external_signer" : true|false,         (boolean) whether this wallet is configured to use an external signer such as a hardware wallet
        ///   "blank" : true|false,                   (boolean) Whether this wallet intentionally does not contain any keys, scripts, or descriptors
        ///   "birthtime" : xxx,                      (numeric, optional) The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
        ///   "lastprocessedblock" : {                (json object) hash and height of the block this information was generated on
        ///     "hash" : "hex",                       (string) hash of the block this information was generated on
        ///     "height" : n                          (numeric) height of the block this information was generated on
        ///   }
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli getwalletinfo 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getwalletinfo", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getwalletinfo(&self) -> RpcResult<GetwalletinfoResponse> {
            self.call("getwalletinfo", json!([]))
        }
    };
}


/// client impl for `getzmqnotifications` RPC (v29)
macro_rules! impl_client_v29__getzmqnotifications {
    () => {
        /// Returns information about the active ZeroMQ notifications.
        ///
        /// Result:
        /// [                         (json array)
        ///   {                       (json object)
        ///     "type" : "str",       (string) Type of notification
        ///     "address" : "str",    (string) Address of the publisher
        ///     "hwm" : n             (numeric) Outbound message high water mark
        ///   },
        ///   ...
        /// ]
        ///
        /// Examples:
        /// > bitcoin-cli getzmqnotifications 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "getzmqnotifications", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn getzmqnotifications(&self) -> RpcResult<GetzmqnotificationsResponse> {
            self.call("getzmqnotifications", json!([]))
        }
    };
}


/// client impl for `listaddressgroupings` RPC (v29)
macro_rules! impl_client_v29__listaddressgroupings {
    () => {
        /// Lists groups of addresses which have had their common ownership
        /// made public by common use as inputs or as the resulting change
        /// in past transactions
        ///
        /// Result:
        /// [               (json array)
        ///   [             (json array)
        ///     [           (json array)
        ///       "str",    (string) The bitcoin address
        ///       n,        (numeric) The amount in BTC
        ///       "str"     (string, optional) The label
        ///     ],
        ///     ...
        ///   ],
        ///   ...
        /// ]
        ///
        /// Examples:
        /// > bitcoin-cli listaddressgroupings 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "listaddressgroupings", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn listaddressgroupings(&self) -> RpcResult<ListaddressgroupingsResponse> {
            self.call("listaddressgroupings", json!([]))
        }
    };
}


/// client impl for `listbanned` RPC (v29)
macro_rules! impl_client_v29__listbanned {
    () => {
        /// List all manually banned IPs/Subnets.
        ///
        /// Result:
        /// [                              (json array)
        ///   {                            (json object)
        ///     "address" : "str",         (string) The IP/Subnet of the banned node
        ///     "ban_created" : xxx,       (numeric) The UNIX epoch time the ban was created
        ///     "banned_until" : xxx,      (numeric) The UNIX epoch time the ban expires
        ///     "ban_duration" : xxx,      (numeric) The ban duration, in seconds
        ///     "time_remaining" : xxx     (numeric) The time remaining until the ban expires, in seconds
        ///   },
        ///   ...
        /// ]
        ///
        /// Examples:
        /// > bitcoin-cli listbanned 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "listbanned", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn listbanned(&self) -> RpcResult<ListbannedResponse> {
            self.call("listbanned", json!([]))
        }
    };
}


/// client impl for `listlockunspent` RPC (v29)
macro_rules! impl_client_v29__listlockunspent {
    () => {
        /// Returns list of temporarily unspendable outputs.
        /// See the lockunspent call to lock and unlock transactions for spending.
        ///
        /// Result:
        /// [                      (json array)
        ///   {                    (json object)
        ///     "txid" : "hex",    (string) The transaction id locked
        ///     "vout" : n         (numeric) The vout value
        ///   },
        ///   ...
        /// ]
        ///
        /// Examples:
        ///
        /// List the unspent transactions
        /// > bitcoin-cli listunspent 
        ///
        /// Lock an unspent transaction
        /// > bitcoin-cli lockunspent false "[{\"txid\":\"a08e6907dbbd3d809776dbfc5d82e371b764ed838b5655e72f463568df1aadf0\",\"vout\":1}]"
        ///
        /// List the locked transactions
        /// > bitcoin-cli listlockunspent 
        ///
        /// Unlock the transaction again
        /// > bitcoin-cli lockunspent true "[{\"txid\":\"a08e6907dbbd3d809776dbfc5d82e371b764ed838b5655e72f463568df1aadf0\",\"vout\":1}]"
        ///
        /// As a JSON-RPC call
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "listlockunspent", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn listlockunspent(&self) -> RpcResult<ListlockunspentResponse> {
            self.call("listlockunspent", json!([]))
        }
    };
}


/// client impl for `listwalletdir` RPC (v29)
macro_rules! impl_client_v29__listwalletdir {
    () => {
        /// Returns a list of wallets in the wallet directory.
        ///
        /// Result:
        /// {                        (json object)
        ///   "wallets" : [          (json array)
        ///     {                    (json object)
        ///       "name" : "str"     (string) The wallet name
        ///     },
        ///     ...
        ///   ]
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli listwalletdir 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "listwalletdir", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn listwalletdir(&self) -> RpcResult<ListwalletdirResponse> {
            self.call("listwalletdir", json!([]))
        }
    };
}


/// client impl for `listwallets` RPC (v29)
macro_rules! impl_client_v29__listwallets {
    () => {
        /// Returns a list of currently loaded wallets.
        /// For full information on the wallet, use "getwalletinfo"
        ///
        /// Result:
        /// [           (json array)
        ///   "str",    (string) the wallet name
        ///   ...
        /// ]
        ///
        /// Examples:
        /// > bitcoin-cli listwallets 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "listwallets", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn listwallets(&self) -> RpcResult<ListwalletsResponse> {
            self.call("listwallets", json!([]))
        }
    };
}


/// client impl for `ping` RPC (v29)
macro_rules! impl_client_v29__ping {
    () => {
        /// Requests that a ping be sent to all other nodes, to measure ping time.
        /// Results provided in getpeerinfo, pingtime and pingwait fields are decimal seconds.
        /// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
        ///
        /// Result:
        /// null    (json null)
        ///
        /// Examples:
        /// > bitcoin-cli ping 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "ping", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn ping(&self) -> RpcResult<PingResponse> {
            self.call("ping", json!([]))
        }
    };
}


/// client impl for `savemempool` RPC (v29)
macro_rules! impl_client_v29__savemempool {
    () => {
        /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
        ///
        /// Result:
        /// {                        (json object)
        ///   "filename" : "str"     (string) the directory and file where the mempool was saved
        /// }
        ///
        /// Examples:
        /// > bitcoin-cli savemempool 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "savemempool", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn savemempool(&self) -> RpcResult<SavemempoolResponse> {
            self.call("savemempool", json!([]))
        }
    };
}


/// client impl for `stop` RPC (v29)
macro_rules! impl_client_v29__stop {
    () => {
        /// Request a graceful shutdown of Bitcoin Core.
        ///
        /// Result:
        /// "str"    (string) A string with the content 'Bitcoin Core stopping'
        pub fn stop(&self) -> RpcResult<StopResponse> {
            self.call("stop", json!([]))
        }
    };
}


/// client impl for `uptime` RPC (v29)
macro_rules! impl_client_v29__uptime {
    () => {
        /// Returns the total uptime of the server.
        ///
        /// Result:
        /// n    (numeric) The number of seconds that the server has been running
        ///
        /// Examples:
        /// > bitcoin-cli uptime 
        /// > curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "uptime", "params": []}' -H 'content-type: application/json' http://127.0.0.1:8332/
        pub fn uptime(&self) -> RpcResult<UptimeResponse> {
            self.call("uptime", json!([]))
        }
    };
}


