use bitcoin::{amount::Amount, hex::Hex, time::Time};
use serde::{Deserialize, Serialize};
use serde_json;

/// Response for the abortrescan RPC call.
///
/// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

/// Response for the addmultisigaddress RPC call.
///
/// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// Each key is a Bitcoin address or hex-encoded public key.
/// This functionality is only intended for use with non-watchonly addresses.
/// See `importaddress` for watchonly p2sh address support.
/// If 'label' is specified, assign address to that label.
/// Note: This command is only compatible with legacy wallets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddmultisigaddressResponse {
    /// The value of the new multisig address
    pub address: String,
    /// The string value of the hex-encoded redemption script
    pub redeemscript: Hex,
    /// The descriptor for this multisig
    pub descriptor: String,
    /// Any warnings resulting from the creation of this multisig
    pub warnings: Vec<String>,

}

/// Response for the bumpfee RPC call.
///
/// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
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
pub struct BumpfeeResponse {
    /// The id of the new transaction.
    pub txid: Hex,
    /// The fee of the replaced transaction.
    pub origfee: Amount,
    /// The fee of the new transaction.
    pub fee: Amount,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,

}

/// Response for the createwallet RPC call.
///
/// Creates and loads a new wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatewalletResponse {
    /// The wallet name if created successfully. If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Vec<String>,

}

/// Response for the createwalletdescriptor RPC call.
///
/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CreatewalletdescriptorResponse {
    /// The public descriptors that were added to the wallet
    pub descs: Vec<String>,

}

/// Response for the dumpprivkey RPC call.
///
/// Reveals the private key corresponding to 'address'.
/// Then the importprivkey can be used with this output
/// Note: This command is only compatible with legacy wallets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the dumpwallet RPC call.
///
/// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// Note: This command is only compatible with legacy wallets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DumpwalletResponse {
    /// The filename with full absolute path
    pub filename: String,

}

/// Response for the encryptwallet RPC call.
///
/// Encrypts the wallet with 'passphrase'. This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior the making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the getaddressesbylabel RPC call.
///
/// Returns the list of addresses assigned the specified label.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddressesbylabelResponse {
    /// json object with information about address
    pub address: serde_json::Address,

}

/// Response for the getaddressinfo RPC call.
///
/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddressinfoResponse {
    /// The bitcoin address validated.
    pub address: String,
    /// The hex-encoded output script generated by the address.
    pub scriptpubkey: Hex,
    /// If the address is yours.
    pub ismine: bool,
    /// If the address is watchonly.
    pub iswatchonly: bool,
    /// If we know how to spend coins sent to this address, ignoring the possible lack of private keys.
    pub solvable: bool,
    /// A descriptor for spending coins sent to this address (only when solvable).
    pub desc: String,
    /// The descriptor used to derive this address if this is a descriptor wallet
    pub parent_desc: String,
    /// If the key is a script.
    pub isscript: bool,
    /// If the address was used for change output.
    pub ischange: bool,
    /// If the address is a witness address.
    pub iswitness: bool,
    /// The version number of the witness program.
    pub witness_version: f64,
    /// The hex value of the witness program.
    pub witness_program: Hex,
    /// The output script type. Only if isscript is true and the redeemscript is known. Possible
    /// types: nonstandard, pubkey, pubkeyhash, scripthash, multisig, nulldata, witness_v0_keyhash,
    /// witness_v0_scripthash, witness_unknown.
    pub script: String,
    /// The redeemscript for the p2sh address.
    pub hex: Hex,
    /// Array of pubkeys associated with the known redeemscript (only if script is multisig).
    pub pubkeys: Vec<String>,
    /// The number of signatures required to spend multisig output (only if script is multisig).
    pub sigsrequired: f64,
    /// The hex value of the raw public key for single-key addresses (possibly embedded in P2SH or P2WSH).
    pub pubkey: Hex,
    /// Information about the address embedded in P2SH or P2WSH, if relevant and known.
    pub embedded: serde_json::Embedded,
    /// If the pubkey is compressed.
    pub iscompressed: bool,
    /// The creation time of the key, if available, expressed in UNIX epoch time.
    pub timestamp: Time,
    /// The HD keypath, if the key is HD and available.
    pub hdkeypath: String,
    /// The Hash160 of the HD seed.
    pub hdseedid: Hex,
    /// The fingerprint of the master key.
    pub hdmasterfingerprint: Hex,
    /// Array of labels associated with the address. Currently limited to one label but returned
    /// as an array to keep the API stable if multiple labels are enabled in the future.
    pub labels: Vec<String>,

}

/// Response for the getbalance RPC call.
///
/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalanceResponse {
    pub result: amount,
}

/// Response for the getbalances RPC call.
///
/// Returns an object with all balances in BTC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    /// balances from outputs that the wallet can sign
    pub mine: serde_json::Mine,
    /// watchonly balances (not present if wallet does not watch anything)
    pub watchonly: serde_json::Watchonly,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: serde_json::Lastprocessedblock,

}

/// Response for the gethdkeys RPC call.
///
/// List all BIP 32 HD keys in the wallet and which descriptors use them.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GethdkeysResponse {

    pub result: serde_json::Value,

}

/// Response for the getnewaddress RPC call.
///
/// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the getrawchangeaddress RPC call.
///
/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the getreceivedbyaddress RPC call.
///
/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetreceivedbyaddressResponse {
    pub result: amount,
}

/// Response for the getreceivedbylabel RPC call.
///
/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetreceivedbylabelResponse {
    pub result: amount,
}

/// Response for the gettransaction RPC call.
///
/// Get detailed information about in-wallet transaction <txid>
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GettransactionResponse {
    /// The amount in BTC
    pub amount: Amount,
    /// The amount of the fee in BTC. This is negative and only available for the
    /// 'send' category of transactions.
    pub fee: Amount,
    /// The number of confirmations for the transaction. Negative confirmations means the
    /// transaction conflicted that many blocks ago.
    pub confirmations: f64,
    /// Only present if the transaction's only input is a coinbase one.
    pub generated: bool,
    /// Whether we consider the transaction to be trusted and safe to spend from.
    /// Only present when the transaction has 0 confirmations (or negative confirmations, if conflicted).
    pub trusted: bool,
    /// The block hash containing the transaction.
    pub blockhash: Hex,
    /// The block height containing the transaction.
    pub blockheight: f64,
    /// The index of the transaction in the block that includes it.
    pub blockindex: f64,
    /// The block time expressed in UNIX epoch time.
    pub blocktime: Time,
    /// The transaction id.
    pub txid: Hex,
    /// The hash of serialized transaction, including witness data.
    pub wtxid: Hex,
    /// Confirmed transactions that have been detected by the wallet to conflict with this transaction.
    pub walletconflicts: Vec<Hex>,
    /// Only if 'category' is 'send'. The txid if this tx was replaced.
    pub replaced_by_txid: Hex,
    /// Only if 'category' is 'send'. The txid if this tx replaces another.
    pub replaces_txid: Hex,
    /// Transactions in the mempool that directly conflict with either this transaction or an ancestor transaction
    pub mempoolconflicts: Vec<Hex>,
    /// If a comment to is associated with the transaction.
    pub to: String,
    /// The transaction time expressed in UNIX epoch time.
    pub time: Time,
    /// The time received expressed in UNIX epoch time.
    pub timereceived: Time,
    /// If a comment is associated with the transaction, only present if not empty.
    pub comment: String,
    /// ("yes|no|unknown") Whether this transaction signals BIP125 replaceability or has an unconfirmed ancestor signaling BIP125 replaceability.
    /// May be unknown for unconfirmed transactions not in the mempool because their unconfirmed ancestors are unknown.
    pub bip125_replaceable: String,
    /// Only if 'category' is 'received'. List of parent descriptors for the output script of this coin.
    pub parent_descs: Vec<String>,

    pub details: Vec<serde_json::Value>,
    /// Raw data for transaction
    pub hex: Hex,
    /// The decoded transaction (only present when `verbose` is passed)
    pub decoded: serde_json::Decoded,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: serde_json::Lastprocessedblock,

}

/// Response for the getunconfirmedbalance RPC call.
///
/// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetunconfirmedbalanceResponse {
    pub result: number,
}

/// Response for the getwalletinfo RPC call.
///
/// Returns an object containing various wallet state info.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetwalletinfoResponse {
    /// the wallet name
    pub walletname: String,
    /// the wallet version
    pub walletversion: f64,
    /// the database format (bdb or sqlite)
    pub format: String,
    /// DEPRECATED. Identical to getbalances().mine.trusted
    pub balance: Amount,
    /// DEPRECATED. Identical to getbalances().mine.untrusted_pending
    pub unconfirmed_balance: Amount,
    /// DEPRECATED. Identical to getbalances().mine.immature
    pub immature_balance: Amount,
    /// the total number of transactions in the wallet
    pub txcount: f64,
    /// the UNIX epoch time of the oldest pre-generated key in the key pool. Legacy wallets only.
    pub keypoololdest: Time,
    /// how many new keys are pre-generated (only counts external keys)
    pub keypoolsize: f64,
    /// how many new keys are pre-generated for internal use (used for change outputs, only appears if the wallet is using this feature, otherwise external keys are used)
    pub keypoolsize_hd_internal: f64,
    /// the UNIX epoch time until which the wallet is unlocked for transfers, or 0 if the wallet is locked (only present for passphrase-encrypted wallets)
    pub unlocked_until: Time,
    /// the transaction fee configuration, set in BTC/kvB
    pub paytxfee: Amount,
    /// the Hash160 of the HD seed (only present when HD is enabled)
    pub hdseedid: Hex,
    /// false if privatekeys are disabled for this wallet (enforced watch-only wallet)
    pub private_keys_enabled: bool,
    /// whether this wallet tracks clean/dirty coins in terms of reuse
    pub avoid_reuse: bool,
    /// current scanning details, or false if no scan is in progress
    pub scanning: serde_json::Scanning,
    /// whether this wallet uses descriptors for output script management
    pub descriptors: bool,
    /// whether this wallet is configured to use an external signer such as a hardware wallet
    pub external_signer: bool,
    /// Whether this wallet intentionally does not contain any keys, scripts, or descriptors
    pub blank: bool,
    /// The start time for blocks scanning. It could be modified by (re)importing any descriptor with an earlier timestamp.
    pub birthtime: Time,
    /// hash and height of the block this information was generated on
    pub lastprocessedblock: serde_json::Lastprocessedblock,

}

/// Response for the importdescriptors RPC call.
///
/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second elements will be imported as an internal descriptor.
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ImportdescriptorsResponse {

    pub result: serde_json::Value,

}

/// Response for the importmulti RPC call.
///
/// Import addresses/scripts (with private or public keys, redeem script (P2SH)), optionally rescanning the blockchain from the earliest creation time of the imported scripts. Requires a new wallet backup.
/// If an address/script is imported without all of the private keys required to spend from that address, it will be watchonly. The 'watchonly' option must be set to true in this case or a warning will be returned.
/// Conversely, if all the private keys are provided and the address/script is spendable, the watchonly option must be set to false, or a warning will be returned.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" for descriptor wallets.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ImportmultiResponse {

    pub result: serde_json::Value,

}

/// Response for the listaddressgroupings RPC call.
///
/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListaddressgroupingsResponse {

    pub result: Vec<array-fixed>,

}

/// Response for the listdescriptors RPC call.
///
/// List descriptors imported into a descriptor-enabled wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListdescriptorsResponse {
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
    /// Array of descriptor objects (sorted by descriptor string representation)
    pub descriptors: Vec<serde_json::Value>,

}

/// Response for the listlabels RPC call.
///
/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListlabelsResponse {
    /// Label name
    pub label: String,

}

/// Response for the listlockunspent RPC call.
///
/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListlockunspentResponse {

    pub result: serde_json::Value,

}

/// Response for the listreceivedbyaddress RPC call.
///
/// List balances by receiving address.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListreceivedbyaddressResponse {

    pub result: serde_json::Value,

}

/// Response for the listreceivedbylabel RPC call.
///
/// List received transactions by label.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListreceivedbylabelResponse {

    pub result: serde_json::Value,

}

/// Response for the listsinceblock RPC call.
///
/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListsinceblockResponse {

    pub transactions: Vec<serde_json::Value>,
    /// <structure is the same as "transactions" above, only present if include_removed=true>
    /// Note: transactions that were re-added in the active chain will appear as-is in this array, and may thus have a positive confirmation count.
    pub removed: Vec<serde_json::Value>,
    /// The hash of the block (target_confirmations-1) from the best block on the main chain, or the genesis hash if the referenced block does not exist yet. This is typically used to feed back into listsinceblock the next time you call it. So you would generally use a target_confirmations of say 6, so you will be continually re-notified of transactions until they've reached 6 confirmations plus any new ones
    pub lastblock: Hex,

}

/// Response for the listtransactions RPC call.
///
/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListtransactionsResponse {

    pub result: serde_json::Value,

}

/// Response for the listunspent RPC call.
///
/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListunspentResponse {

    pub result: serde_json::Value,

}

/// Response for the listwalletdir RPC call.
///
/// Returns a list of wallets in the wallet directory.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletdirResponse {

    pub wallets: Vec<serde_json::Value>,

}

/// Response for the listwallets RPC call.
///
/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletsResponse {
    /// the wallet name
    pub walletname: String,

}

/// Response for the loadwallet RPC call.
///
/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LoadwalletResponse {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Vec<String>,

}

/// Response for the lockunspent RPC call.
///
/// Updates list of temporarily unspendable outputs.
/// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
/// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
/// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
/// Manually selected coins are automatically unlocked.
/// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
/// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
/// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
/// Also see the listunspent call
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

/// Response for the migratewallet RPC call.
///
/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MigratewalletResponse {
    /// The name of the primary migrated wallet
    pub wallet_name: String,
    /// The name of the migrated wallet containing the watchonly scripts
    pub watchonly_name: String,
    /// The name of the migrated wallet containing solvable but not watched scripts
    pub solvables_name: String,
    /// The location of the backup of the original wallet
    pub backup_path: String,

}

/// Response for the psbtbumpfee RPC call.
///
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
    /// The base64-encoded unsigned PSBT of the new transaction.
    pub psbt: String,
    /// The fee of the replaced transaction.
    pub origfee: Amount,
    /// The fee of the new transaction.
    pub fee: Amount,
    /// Errors encountered during processing (may be empty).
    pub errors: Vec<String>,

}

/// Response for the rescanblockchain RPC call.
///
/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster when used on a descriptor wallet
/// and block filters are available (using startup option "-blockfilterindex=1").
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RescanblockchainResponse {
    /// The block height where the rescan started (the requested height or 0)
    pub start_height: f64,
    /// The height of the last rescanned block. May be null in rare cases if there was a reorg and the call didn't scan any blocks because they were already scanned in the background.
    pub stop_height: f64,

}

/// Response for the restorewallet RPC call.
///
/// Restores and loads a wallet from backup.
/// The rescan is significantly faster if a descriptor wallet is restored
/// and block filters are available (using startup option "-blockfilterindex=1").
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RestorewalletResponse {
    /// The wallet name if restored successfully.
    pub name: String,
    /// Warning messages, if any, related to restoring and loading the wallet.
    pub warnings: Vec<String>,

}

/// Response for the send RPC call.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Send a transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Hex,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Hex,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: String,

}

/// Response for the sendall RPC call.
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendallResponse {
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The transaction id for the send. Only 1 transaction is created regardless of the number of addresses.
    pub txid: Hex,
    /// If add_to_wallet is false, the hex-encoded raw transaction with signature(s)
    pub hex: Hex,
    /// If more signatures are needed, or if add_to_wallet is false, the base64-encoded (partially) signed transaction
    pub psbt: String,

}

/// Response for the sendmany RPC call.
///
/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendmanyResponse {
    pub result: hex,
}

/// Response for the sendtoaddress RPC call.
///
/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendtoaddressResponse {
    pub result: hex,
}

/// Response for the settxfee RPC call.
///
/// Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct BooleanResponse {
    pub result: bool,
}

/// Response for the setwalletflag RPC call.
///
/// Change the state of the given wallet flag for a wallet.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SetwalletflagResponse {
    /// The name of the flag that was modified
    pub flag_name: String,
    /// The new state of the flag
    pub flag_state: bool,
    /// Any warnings associated with the change
    pub warnings: String,

}

/// Response for the signmessage RPC call.
///
/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StringResponse {
    pub result: String,
}

/// Response for the signrawtransactionwithwallet RPC call.
///
/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SignrawtransactionwithwalletResponse {
    /// The hex-encoded raw transaction with signature(s)
    pub hex: Hex,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// Script verification errors (if there are any)
    pub errors: Vec<serde_json::Value>,

}

/// Response for the simulaterawtransaction RPC call.
///
/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SimulaterawtransactionResponse {
    /// The wallet balance change (negative means decrease).
    pub balance_change: Amount,

}

/// Response for the unloadwallet RPC call.
///
/// Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// Specifying the wallet name on a wallet endpoint is invalid.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnloadwalletResponse {
    /// Warning messages, if any, related to unloading the wallet.
    pub warnings: Vec<String>,

}

/// Response for the upgradewallet RPC call.
///
/// Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// New keys may be generated and a new wallet backup will need to be made.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UpgradewalletResponse {
    /// Name of wallet this operation was performed on
    pub wallet_name: String,
    /// Version of wallet before this operation
    pub previous_version: f64,
    /// Version of wallet after this operation
    pub current_version: f64,
    /// Description of result, if no error
    pub result: String,
    /// Error message (if there is one)
    pub error: String,

}

/// Response for the walletcreatefundedpsbt RPC call.
///
/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletcreatefundedpsbtResponse {
    /// The resulting raw transaction (base64-encoded string)
    pub psbt: String,
    /// Fee in BTC the resulting transaction pays
    pub fee: Amount,
    /// The position of the added change output, or -1
    pub changepos: f64,

}

/// Response for the walletdisplayaddress RPC call.
///
/// Display address on an external signer for verification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletdisplayaddressResponse {
    /// The address as confirmed by the signer
    pub address: String,

}

/// Response for the walletprocesspsbt RPC call.
///
/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct WalletprocesspsbtResponse {
    /// The base64-encoded partially signed transaction
    pub psbt: String,
    /// If the transaction has a complete set of signatures
    pub complete: bool,
    /// The hex-encoded network transaction if complete
    pub hex: Hex,

}

