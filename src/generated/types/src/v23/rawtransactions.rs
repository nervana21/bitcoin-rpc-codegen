use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the analyzepsbt RPC call.
///
/// Analyzes and provides information about the current status of a PSBT and its inputs
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AnalyzepsbtResponse {

    pub inputs: Vec<serde_json::Value>,
    /// Estimated vsize of the final signed transaction
    pub estimated_vsize: f64,
    /// Estimated feerate of the final signed transaction in BTC/kvB. Shown only if all UTXO slots in the PSBT have been filled
    pub estimated_feerate: Amount,
    /// The transaction fee paid. Shown only if all UTXO slots in the PSBT have been filled
    pub fee: Amount,
    /// Role of the next person that this psbt needs to go to
    pub next: String,
    /// Error message (if there is one)
    pub error: String,

}

/// Response for the combinepsbt RPC call.
///
/// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the combinerawtransaction RPC call.
///
/// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the converttopsbt RPC call.
///
/// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the createpsbt RPC call.
///
/// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the createrawtransaction RPC call.
///
/// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreaterawtransactionResponse {
    pub result: hex,
}

/// Response for the decodepsbt RPC call.
///
/// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodepsbtResponse {
    /// The decoded network-serialized unsigned transaction.
    pub tx: serde_json::Tx,

    pub global_xpubs: Vec<serde_json::Value>,
    /// The PSBT version number. Not to be confused with the unsigned transaction version
    pub psbt_version: f64,
    /// The global proprietary map
    pub proprietary: Vec<serde_json::Value>,
    /// The unknown global fields
    pub unknown: object_dynamic,

    pub inputs: Vec<serde_json::Value>,

    pub outputs: Vec<serde_json::Value>,
    /// The transaction fee paid if all UTXOs slots in the PSBT have been filled.
    pub fee: Amount,

}

/// Response for the decoderawtransaction RPC call.
///
/// Return a JSON object representing the serialized, hex-encoded transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecoderawtransactionResponse {
    /// The transaction id
    pub txid: Hex,
    /// The transaction hash (differs from txid for witness transactions)
    pub hash: Hex,
    /// The serialized transaction size
    pub size: f64,
    /// The virtual transaction size (differs from size for witness transactions)
    pub vsize: f64,
    /// The transaction's weight (between vsize*4-3 and vsize*4)
    pub weight: f64,
    /// The version
    pub version: f64,
    /// The lock time
    pub locktime: Time,

    pub vin: Vec<serde_json::Value>,

    pub vout: Vec<serde_json::Value>,

}

/// Response for the decodescript RPC call.
///
/// Decode a hex-encoded script.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecodescriptResponse {
    /// Disassembly of the script
    pub asm: String,
    /// Inferred descriptor for the script
    pub desc: String,
    /// The output type (e.g. nonstandard, anchor, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_scripthash, witness_v0_keyhash, witness_v1_taproot, witness_unknown)
    pub type: String,
    /// The Bitcoin address (only if a well-defined address exists)
    pub address: String,
    /// address of P2SH script wrapping this redeem script (not returned for types that should not be wrapped)
    pub p2sh: String,
    /// Result of a witness output script wrapping this redeem script (not returned for types that should not be wrapped)
    pub segwit: serde_json::Segwit,

}

/// Response for the descriptorprocesspsbt RPC call.
///
/// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DescriptorprocesspsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Hex,

}

/// Response for the finalizepsbt RPC call.
///
/// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FinalizepsbtResponse {
    /// The base64-encoded partially signed transaction if not extracted
    pub psbt: String,
    /// The hex-encoded network transaction if extracted
    pub hex: Hex,
    /// If the transaction has a complete set of signatures
    pub complete: bool,

}

/// Response for the fundrawtransaction RPC call.
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
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct FundrawtransactionResponse {
    /// The resulting raw transaction (hex-encoded string)
    pub hex: Hex,
    /// Fee in BTC the resulting transaction pays
    pub fee: Amount,
    /// The position of the added change output, or -1
    pub changepos: f64,

}

/// Response for the getrawtransaction RPC call.
///
/// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// Hint: Use gettransaction for wallet transactions.
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the joinpsbts RPC call.
///
/// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the sendrawtransaction RPC call.
///
/// Submit a raw transaction (serialized, hex-encoded) to local node and network.
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendrawtransactionResponse {
    pub result: hex,
}

/// Response for the signrawtransactionwithkey RPC call.
///
/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignrawtransactionwithkeyResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub hex: Hex,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Vec<serde_json::Value>,

}

/// Response for the submitpackage RPC call.
///
/// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitpackageResponse {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// transaction results keyed by wtxid
    pub tx_results: object_dynamic,
    /// List of txids of replaced transactions
    pub replaced_transactions: Vec<Hex>,

}

/// Response for the testmempoolaccept RPC call.
///
/// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TestmempoolacceptResponse {

    pub result: serde_json::Value,

}

/// Response for the utxoupdatepsbt RPC call.
///
/// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

