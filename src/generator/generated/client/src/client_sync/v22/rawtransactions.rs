/// Implements Bitcoin Core JSON-RPC API method `analyzepsbt` for version v22
///
/// Analyzes and provides information about the current status of a PSBT and its inputs
#[macro_export]
macro_rules! impl_client_v22__analyzepsbt {
    () => {
        impl Client {
            pub fn analyzepsbt(&self, psbt: String) -> Result<serde_json::Value> {
                self.call("analyzepsbt", &[into_json(psbt)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `combinepsbt` for version v22
///
/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
#[macro_export]
macro_rules! impl_client_v22__combinepsbt {
    () => {
        impl Client {
            pub fn combinepsbt(&self, txs: Vec<String>) -> Result<String> {
                self.call("combinepsbt", &[into_json(txs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `combinerawtransaction` for version v22
///
/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
#[macro_export]
macro_rules! impl_client_v22__combinerawtransaction {
    () => {
        impl Client {
            pub fn combinerawtransaction(&self, txs: Vec<String>) -> Result<String> {
                self.call("combinerawtransaction", &[into_json(txs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `converttopsbt` for version v22
///
/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
#[macro_export]
macro_rules! impl_client_v22__converttopsbt {
    () => {
        impl Client {
            pub fn converttopsbt(&self, hexstring: String, permitsigdata: Option<bool>, iswitness: Option<bool>) -> Result<String> {
                let mut params = vec![into_json(hexstring)?];
                if let Some(permitsigdata) = permitsigdata {
                    params.push(into_json(permitsigdata)?);
                }
                if let Some(iswitness) = iswitness {
                    params.push(into_json(iswitness)?);
                }
                self.call("converttopsbt", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createpsbt` for version v22
///
/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
#[macro_export]
macro_rules! impl_client_v22__createpsbt {
    () => {
        impl Client {
            pub fn createpsbt(&self, inputs: Vec<Input>, outputs: Vec<Output>, locktime: Option<i64>, replaceable: Option<bool>) -> Result<String> {
                let mut params = vec![into_json(inputs)?, into_json(outputs)?];
                if let Some(locktime) = locktime {
                    params.push(into_json(locktime)?);
                }
                if let Some(replaceable) = replaceable {
                    params.push(into_json(replaceable)?);
                }
                self.call("createpsbt", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createrawtransaction` for version v22
///
/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[macro_export]
macro_rules! impl_client_v22__createrawtransaction {
    () => {
        impl Client {
            pub fn createrawtransaction(&self, inputs: Vec<Input>, outputs: Vec<Output>, locktime: Option<i64>, replaceable: Option<bool>) -> Result<hex> {
                let mut params = vec![into_json(inputs)?, into_json(outputs)?];
                if let Some(locktime) = locktime {
                    params.push(into_json(locktime)?);
                }
                if let Some(replaceable) = replaceable {
                    params.push(into_json(replaceable)?);
                }
                self.call("createrawtransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `decodepsbt` for version v22
///
/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[macro_export]
macro_rules! impl_client_v22__decodepsbt {
    () => {
        impl Client {
            pub fn decodepsbt(&self, psbt: String) -> Result<serde_json::Value> {
                self.call("decodepsbt", &[into_json(psbt)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `decoderawtransaction` for version v22
///
/// Return a JSON object representing the serialized, hex-encoded transaction.
#[macro_export]
macro_rules! impl_client_v22__decoderawtransaction {
    () => {
        impl Client {
            pub fn decoderawtransaction(&self, hexstring: String, iswitness: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(hexstring)?];
                if let Some(iswitness) = iswitness {
                    params.push(into_json(iswitness)?);
                }
                self.call("decoderawtransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `decodescript` for version v22
///
/// Decode a hex-encoded script.
#[macro_export]
macro_rules! impl_client_v22__decodescript {
    () => {
        impl Client {
            pub fn decodescript(&self, hexstring: String) -> Result<serde_json::Value> {
                self.call("decodescript", &[into_json(hexstring)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `descriptorprocesspsbt` for version v22
///
/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[macro_export]
macro_rules! impl_client_v22__descriptorprocesspsbt {
    () => {
        impl Client {
            pub fn descriptorprocesspsbt(&self, psbt: String, descriptors: Vec<String>, sighashtype: Option<String>, bip32derivs: Option<bool>, finalize: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(psbt)?, into_json(descriptors)?];
                if let Some(sighashtype) = sighashtype {
                    params.push(into_json(sighashtype)?);
                }
                if let Some(bip32derivs) = bip32derivs {
                    params.push(into_json(bip32derivs)?);
                }
                if let Some(finalize) = finalize {
                    params.push(into_json(finalize)?);
                }
                self.call("descriptorprocesspsbt", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `finalizepsbt` for version v22
///
/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[macro_export]
macro_rules! impl_client_v22__finalizepsbt {
    () => {
        impl Client {
            pub fn finalizepsbt(&self, psbt: String, extract: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(psbt)?];
                if let Some(extract) = extract {
                    params.push(into_json(extract)?);
                }
                self.call("finalizepsbt", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `fundrawtransaction` for version v22
///
/// If the transaction has no inputs, they will be automatically selected to meet its out value.
/// It will add at most one change output to the outputs.
/// No existing outputs will be modified unless "subtractFeeFromOutputs" is specified.
/// Note that inputs which were signed may need to be resigned after completion since in/outputs have been added.
/// The inputs added will not be signed, use signrawtransactionwithkey
/// or signrawtransactionwithwallet for that.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
/// Note that all inputs selected must be of standard form and P2SH scripts must be
/// in the wallet using importaddress or addmultisigaddress (to calculate fees).
/// You can see whether this is the case by checking the "solvable" field in the listunspent output.
/// Only pay-to-pubkey, multisig, and P2SH versions thereof are currently supported for watch-only
#[macro_export]
macro_rules! impl_client_v22__fundrawtransaction {
    () => {
        impl Client {
            pub fn fundrawtransaction(&self, hexstring: String, options: Option<serde_json::Value>, iswitness: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(hexstring)?];
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                if let Some(iswitness) = iswitness {
                    params.push(into_json(iswitness)?);
                }
                self.call("fundrawtransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawtransaction` for version v22
///
/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// Hint: Use gettransaction for wallet transactions.
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[macro_export]
macro_rules! impl_client_v22__getrawtransaction {
    () => {
        impl Client {
            pub fn getrawtransaction(&self, txid: String, verbosity: Option<i64>, blockhash: Option<String>) -> Result<String> {
                let mut params = vec![into_json(txid)?];
                if let Some(verbosity) = verbosity {
                    params.push(into_json(verbosity)?);
                }
                if let Some(blockhash) = blockhash {
                    params.push(into_json(blockhash)?);
                }
                self.call("getrawtransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `joinpsbts` for version v22
///
/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
#[macro_export]
macro_rules! impl_client_v22__joinpsbts {
    () => {
        impl Client {
            pub fn joinpsbts(&self, txs: Vec<String>) -> Result<String> {
                self.call("joinpsbts", &[into_json(txs)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendrawtransaction` for version v22
///
/// Submit a raw transaction (serialized, hex-encoded) to local node and network.
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
#[macro_export]
macro_rules! impl_client_v22__sendrawtransaction {
    () => {
        impl Client {
            pub fn sendrawtransaction(&self, hexstring: String, maxfeerate: Option<amount>, maxburnamount: Option<amount>) -> Result<hex> {
                let mut params = vec![into_json(hexstring)?];
                if let Some(maxfeerate) = maxfeerate {
                    params.push(into_json(maxfeerate)?);
                }
                if let Some(maxburnamount) = maxburnamount {
                    params.push(into_json(maxburnamount)?);
                }
                self.call("sendrawtransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signrawtransactionwithkey` for version v22
///
/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[macro_export]
macro_rules! impl_client_v22__signrawtransactionwithkey {
    () => {
        impl Client {
            pub fn signrawtransactionwithkey(&self, hexstring: String, privkeys: Vec<String>, prevtxs: Option<Vec<String>>, sighashtype: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(hexstring)?, into_json(privkeys)?];
                if let Some(prevtxs) = prevtxs {
                    params.push(into_json(prevtxs)?);
                }
                if let Some(sighashtype) = sighashtype {
                    params.push(into_json(sighashtype)?);
                }
                self.call("signrawtransactionwithkey", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `submitpackage` for version v22
///
/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[macro_export]
macro_rules! impl_client_v22__submitpackage {
    () => {
        impl Client {
            pub fn submitpackage(&self, package: Vec<String>, maxfeerate: Option<amount>, maxburnamount: Option<amount>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(package)?];
                if let Some(maxfeerate) = maxfeerate {
                    params.push(into_json(maxfeerate)?);
                }
                if let Some(maxburnamount) = maxburnamount {
                    params.push(into_json(maxburnamount)?);
                }
                self.call("submitpackage", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `testmempoolaccept` for version v22
///
/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.
#[macro_export]
macro_rules! impl_client_v22__testmempoolaccept {
    () => {
        impl Client {
            pub fn testmempoolaccept(&self, rawtxs: Vec<String>, maxfeerate: Option<amount>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![into_json(rawtxs)?];
                if let Some(maxfeerate) = maxfeerate {
                    params.push(into_json(maxfeerate)?);
                }
                self.call("testmempoolaccept", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `utxoupdatepsbt` for version v22
///
/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[macro_export]
macro_rules! impl_client_v22__utxoupdatepsbt {
    () => {
        impl Client {
            pub fn utxoupdatepsbt(&self, psbt: String, descriptors: Option<Vec<String>>) -> Result<String> {
                let mut params = vec![into_json(psbt)?];
                if let Some(descriptors) = descriptors {
                    params.push(into_json(descriptors)?);
                }
                self.call("utxoupdatepsbt", &params)
            }
        }
    };
}

