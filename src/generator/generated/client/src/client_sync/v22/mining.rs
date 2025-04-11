/// Implements Bitcoin Core JSON-RPC API method `getblocktemplate` for version v22
///
/// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
#[macro_export]
macro_rules! impl_client_v22__getblocktemplate {
    () => {
        impl Client {
            pub fn getblocktemplate(&self, template_request: serde_json::Value) -> Result<()> {
                self.call("getblocktemplate", &[into_json(template_request)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmininginfo` for version v22
///
/// Returns a json object containing mining-related information.
#[macro_export]
macro_rules! impl_client_v22__getmininginfo {
    () => {
        impl Client {
            pub fn getmininginfo(&self) -> Result<serde_json::Value> {
                self.call("getmininginfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnetworkhashps` for version v22
///
/// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
#[macro_export]
macro_rules! impl_client_v22__getnetworkhashps {
    () => {
        impl Client {
            pub fn getnetworkhashps(&self, nblocks: Option<i64>, height: Option<i64>) -> Result<number> {
                let mut params = vec![];
                if let Some(nblocks) = nblocks {
                    params.push(into_json(nblocks)?);
                }
                if let Some(height) = height {
                    params.push(into_json(height)?);
                }
                self.call("getnetworkhashps", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getprioritisedtransactions` for version v22
///
/// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
#[macro_export]
macro_rules! impl_client_v22__getprioritisedtransactions {
    () => {
        impl Client {
            pub fn getprioritisedtransactions(&self) -> Result<object_dynamic> {
                self.call("getprioritisedtransactions", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `prioritisetransaction` for version v22
///
/// Accepts the transaction into mined blocks at a higher (or lower) priority
#[macro_export]
macro_rules! impl_client_v22__prioritisetransaction {
    () => {
        impl Client {
            pub fn prioritisetransaction(&self, txid: String, dummy: Option<i64>, fee_delta: i64) -> Result<bool> {
                let mut params = vec![into_json(txid)?, into_json(fee_delta)?];
                if let Some(dummy) = dummy {
                    params.push(into_json(dummy)?);
                }
                self.call("prioritisetransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `submitblock` for version v22
///
/// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
#[macro_export]
macro_rules! impl_client_v22__submitblock {
    () => {
        impl Client {
            pub fn submitblock(&self, hexdata: String, dummy: Option<String>) -> Result<()> {
                let mut params = vec![into_json(hexdata)?];
                if let Some(dummy) = dummy {
                    params.push(into_json(dummy)?);
                }
                self.call("submitblock", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `submitheader` for version v22
///
/// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.
#[macro_export]
macro_rules! impl_client_v22__submitheader {
    () => {
        impl Client {
            pub fn submitheader(&self, hexdata: String) -> Result<()> {
                self.call("submitheader", &[into_json(hexdata)?])
            }
        }
    };
}

