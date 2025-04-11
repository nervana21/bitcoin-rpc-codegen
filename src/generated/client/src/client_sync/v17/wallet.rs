/// Implements Bitcoin Core JSON-RPC API method `abandontransaction` for version v17
///
/// Mark in-wallet transaction <txid> as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.
#[macro_export]
macro_rules! impl_client_v17__abandontransaction {
    () => {
        impl Client {
            pub fn abandontransaction(&self, txid: String) -> Result<()> {
                self.call("abandontransaction", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `abortrescan` for version v17
///
/// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.
#[macro_export]
macro_rules! impl_client_v17__abortrescan {
    () => {
        impl Client {
            pub fn abortrescan(&self) -> Result<bool> {
                self.call("abortrescan", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `addmultisigaddress` for version v17
///
/// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// Each key is a Bitcoin address or hex-encoded public key.
/// This functionality is only intended for use with non-watchonly addresses.
/// See `importaddress` for watchonly p2sh address support.
/// If 'label' is specified, assign address to that label.
/// Note: This command is only compatible with legacy wallets.
#[macro_export]
macro_rules! impl_client_v17__addmultisigaddress {
    () => {
        impl Client {
            pub fn addmultisigaddress(&self, nrequired: i64, keys: Vec<String>, label: Option<String>, address_type: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(nrequired)?, into_json(keys)?];
                if let Some(label) = label {
                    params.push(into_json(label)?);
                }
                if let Some(address_type) = address_type {
                    params.push(into_json(address_type)?);
                }
                self.call("addmultisigaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `backupwallet` for version v17
///
/// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
#[macro_export]
macro_rules! impl_client_v17__backupwallet {
    () => {
        impl Client {
            pub fn backupwallet(&self, destination: String) -> Result<()> {
                self.call("backupwallet", &[into_json(destination)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `bumpfee` for version v17
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
#[macro_export]
macro_rules! impl_client_v17__bumpfee {
    () => {
        impl Client {
            pub fn bumpfee(&self, txid: String, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(txid)?];
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("bumpfee", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createwallet` for version v17
///
/// Creates and loads a new wallet.
#[macro_export]
macro_rules! impl_client_v17__createwallet {
    () => {
        impl Client {
            pub fn createwallet(&self, wallet_name: String, disable_private_keys: Option<bool>, blank: Option<bool>, passphrase: Option<String>, avoid_reuse: Option<bool>, descriptors: Option<bool>, load_on_startup: Option<bool>, external_signer: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(wallet_name)?];
                if let Some(disable_private_keys) = disable_private_keys {
                    params.push(into_json(disable_private_keys)?);
                }
                if let Some(blank) = blank {
                    params.push(into_json(blank)?);
                }
                if let Some(passphrase) = passphrase {
                    params.push(into_json(passphrase)?);
                }
                if let Some(avoid_reuse) = avoid_reuse {
                    params.push(into_json(avoid_reuse)?);
                }
                if let Some(descriptors) = descriptors {
                    params.push(into_json(descriptors)?);
                }
                if let Some(load_on_startup) = load_on_startup {
                    params.push(into_json(load_on_startup)?);
                }
                if let Some(external_signer) = external_signer {
                    params.push(into_json(external_signer)?);
                }
                self.call("createwallet", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `createwalletdescriptor` for version v17
///
/// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__createwalletdescriptor {
    () => {
        impl Client {
            pub fn createwalletdescriptor(&self, type: String, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(type)?];
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("createwalletdescriptor", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `dumpprivkey` for version v17
///
/// Reveals the private key corresponding to 'address'.
/// Then the importprivkey can be used with this output
/// Note: This command is only compatible with legacy wallets.
#[macro_export]
macro_rules! impl_client_v17__dumpprivkey {
    () => {
        impl Client {
            pub fn dumpprivkey(&self, address: String) -> Result<String> {
                self.call("dumpprivkey", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `dumpwallet` for version v17
///
/// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// Note: This command is only compatible with legacy wallets.
#[macro_export]
macro_rules! impl_client_v17__dumpwallet {
    () => {
        impl Client {
            pub fn dumpwallet(&self, filename: String) -> Result<serde_json::Value> {
                self.call("dumpwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `encryptwallet` for version v17
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
#[macro_export]
macro_rules! impl_client_v17__encryptwallet {
    () => {
        impl Client {
            pub fn encryptwallet(&self, passphrase: String) -> Result<String> {
                self.call("encryptwallet", &[into_json(passphrase)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddressesbylabel` for version v17
///
/// Returns the list of addresses assigned the specified label.
#[macro_export]
macro_rules! impl_client_v17__getaddressesbylabel {
    () => {
        impl Client {
            pub fn getaddressesbylabel(&self, label: String) -> Result<object_dynamic> {
                self.call("getaddressesbylabel", &[into_json(label)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddressinfo` for version v17
///
/// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
#[macro_export]
macro_rules! impl_client_v17__getaddressinfo {
    () => {
        impl Client {
            pub fn getaddressinfo(&self, address: String) -> Result<serde_json::Value> {
                self.call("getaddressinfo", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getbalance` for version v17
///
/// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
#[macro_export]
macro_rules! impl_client_v17__getbalance {
    () => {
        impl Client {
            pub fn getbalance(&self, dummy: Option<String>, minconf: Option<i64>, include_watchonly: Option<bool>, avoid_reuse: Option<bool>) -> Result<amount> {
                let mut params = vec![];
                if let Some(dummy) = dummy {
                    params.push(into_json(dummy)?);
                }
                if let Some(minconf) = minconf {
                    params.push(into_json(minconf)?);
                }
                if let Some(include_watchonly) = include_watchonly {
                    params.push(into_json(include_watchonly)?);
                }
                if let Some(avoid_reuse) = avoid_reuse {
                    params.push(into_json(avoid_reuse)?);
                }
                self.call("getbalance", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getbalances` for version v17
///
/// Returns an object with all balances in BTC.
#[macro_export]
macro_rules! impl_client_v17__getbalances {
    () => {
        impl Client {
            pub fn getbalances(&self) -> Result<serde_json::Value> {
                self.call("getbalances", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gethdkeys` for version v17
///
/// List all BIP 32 HD keys in the wallet and which descriptors use them.
#[macro_export]
macro_rules! impl_client_v17__gethdkeys {
    () => {
        impl Client {
            pub fn gethdkeys(&self, options: Option<serde_json::Value>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![];
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("gethdkeys", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnewaddress` for version v17
///
/// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.
#[macro_export]
macro_rules! impl_client_v17__getnewaddress {
    () => {
        impl Client {
            pub fn getnewaddress(&self, label: Option<String>, address_type: Option<String>) -> Result<String> {
                let mut params = vec![];
                if let Some(label) = label {
                    params.push(into_json(label)?);
                }
                if let Some(address_type) = address_type {
                    params.push(into_json(address_type)?);
                }
                self.call("getnewaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawchangeaddress` for version v17
///
/// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
#[macro_export]
macro_rules! impl_client_v17__getrawchangeaddress {
    () => {
        impl Client {
            pub fn getrawchangeaddress(&self, address_type: Option<String>) -> Result<String> {
                let mut params = vec![];
                if let Some(address_type) = address_type {
                    params.push(into_json(address_type)?);
                }
                self.call("getrawchangeaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getreceivedbyaddress` for version v17
///
/// Returns the total amount received by the given address in transactions with at least minconf confirmations.
#[macro_export]
macro_rules! impl_client_v17__getreceivedbyaddress {
    () => {
        impl Client {
            pub fn getreceivedbyaddress(&self, address: String, minconf: Option<i64>, include_immature_coinbase: Option<bool>) -> Result<amount> {
                let mut params = vec![into_json(address)?];
                if let Some(minconf) = minconf {
                    params.push(into_json(minconf)?);
                }
                if let Some(include_immature_coinbase) = include_immature_coinbase {
                    params.push(into_json(include_immature_coinbase)?);
                }
                self.call("getreceivedbyaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getreceivedbylabel` for version v17
///
/// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
#[macro_export]
macro_rules! impl_client_v17__getreceivedbylabel {
    () => {
        impl Client {
            pub fn getreceivedbylabel(&self, label: String, minconf: Option<i64>, include_immature_coinbase: Option<bool>) -> Result<amount> {
                let mut params = vec![into_json(label)?];
                if let Some(minconf) = minconf {
                    params.push(into_json(minconf)?);
                }
                if let Some(include_immature_coinbase) = include_immature_coinbase {
                    params.push(into_json(include_immature_coinbase)?);
                }
                self.call("getreceivedbylabel", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `gettransaction` for version v17
///
/// Get detailed information about in-wallet transaction <txid>
#[macro_export]
macro_rules! impl_client_v17__gettransaction {
    () => {
        impl Client {
            pub fn gettransaction(&self, txid: String, include_watchonly: Option<bool>, verbose: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(txid)?];
                if let Some(include_watchonly) = include_watchonly {
                    params.push(into_json(include_watchonly)?);
                }
                if let Some(verbose) = verbose {
                    params.push(into_json(verbose)?);
                }
                self.call("gettransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getunconfirmedbalance` for version v17
///
/// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending
#[macro_export]
macro_rules! impl_client_v17__getunconfirmedbalance {
    () => {
        impl Client {
            pub fn getunconfirmedbalance(&self) -> Result<number> {
                self.call("getunconfirmedbalance", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getwalletinfo` for version v17
///
/// Returns an object containing various wallet state info.
#[macro_export]
macro_rules! impl_client_v17__getwalletinfo {
    () => {
        impl Client {
            pub fn getwalletinfo(&self) -> Result<serde_json::Value> {
                self.call("getwalletinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importaddress` for version v17
///
/// Adds an address or script (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported address exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// If you have the full public key, you should call importpubkey instead of this.
/// Hint: use importmulti to import more than one address.
/// Note: If you import a non-standard raw script in hex form, outputs sending to it will be treated
/// as change, and not show up in many RPCs.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" for descriptor wallets.
#[macro_export]
macro_rules! impl_client_v17__importaddress {
    () => {
        impl Client {
            pub fn importaddress(&self, address: String, label: Option<String>, rescan: Option<bool>, p2sh: Option<bool>) -> Result<()> {
                let mut params = vec![into_json(address)?];
                if let Some(label) = label {
                    params.push(into_json(label)?);
                }
                if let Some(rescan) = rescan {
                    params.push(into_json(rescan)?);
                }
                if let Some(p2sh) = p2sh {
                    params.push(into_json(p2sh)?);
                }
                self.call("importaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importdescriptors` for version v17
///
/// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second elements will be imported as an internal descriptor.
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
#[macro_export]
macro_rules! impl_client_v17__importdescriptors {
    () => {
        impl Client {
            pub fn importdescriptors(&self, requests: Vec<String>) -> Result<Vec<serde_json::Value>> {
                self.call("importdescriptors", &[into_json(requests)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importmulti` for version v17
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
#[macro_export]
macro_rules! impl_client_v17__importmulti {
    () => {
        impl Client {
            pub fn importmulti(&self, requests: Vec<String>, options: Option<serde_json::Value>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![into_json(requests)?];
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("importmulti", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importprivkey` for version v17
///
/// Adds a private key (as returned by dumpprivkey) to your wallet. Requires a new wallet backup.
/// Hint: use importmulti to import more than one private key.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported key exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" with "combo(X)" for descriptor wallets.
#[macro_export]
macro_rules! impl_client_v17__importprivkey {
    () => {
        impl Client {
            pub fn importprivkey(&self, privkey: String, label: Option<String>, rescan: Option<bool>) -> Result<()> {
                let mut params = vec![into_json(privkey)?];
                if let Some(label) = label {
                    params.push(into_json(label)?);
                }
                if let Some(rescan) = rescan {
                    params.push(into_json(rescan)?);
                }
                self.call("importprivkey", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importprunedfunds` for version v17
///
/// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
#[macro_export]
macro_rules! impl_client_v17__importprunedfunds {
    () => {
        impl Client {
            pub fn importprunedfunds(&self, rawtransaction: String, txoutproof: String) -> Result<()> {
                self.call("importprunedfunds", &[into_json(rawtransaction)?, into_json(txoutproof)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importpubkey` for version v17
///
/// Adds a public key (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
/// Hint: use importmulti to import more than one public key.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported pubkey exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" with "combo(X)" for descriptor wallets.
#[macro_export]
macro_rules! impl_client_v17__importpubkey {
    () => {
        impl Client {
            pub fn importpubkey(&self, pubkey: String, label: Option<String>, rescan: Option<bool>) -> Result<()> {
                let mut params = vec![into_json(pubkey)?];
                if let Some(label) = label {
                    params.push(into_json(label)?);
                }
                if let Some(rescan) = rescan {
                    params.push(into_json(rescan)?);
                }
                self.call("importpubkey", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `importwallet` for version v17
///
/// Imports keys from a wallet dump file (see dumpwallet). Requires a new wallet backup to include imported keys.
/// Note: Blockchain and Mempool will be rescanned after a successful import. Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets.
#[macro_export]
macro_rules! impl_client_v17__importwallet {
    () => {
        impl Client {
            pub fn importwallet(&self, filename: String) -> Result<()> {
                self.call("importwallet", &[into_json(filename)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `keypoolrefill` for version v17
///
/// Fills the keypool.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__keypoolrefill {
    () => {
        impl Client {
            pub fn keypoolrefill(&self, newsize: Option<i64>) -> Result<()> {
                let mut params = vec![];
                if let Some(newsize) = newsize {
                    params.push(into_json(newsize)?);
                }
                self.call("keypoolrefill", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listaddressgroupings` for version v17
///
/// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
#[macro_export]
macro_rules! impl_client_v17__listaddressgroupings {
    () => {
        impl Client {
            pub fn listaddressgroupings(&self) -> Result<Vec<array>> {
                self.call("listaddressgroupings", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listdescriptors` for version v17
///
/// List descriptors imported into a descriptor-enabled wallet.
#[macro_export]
macro_rules! impl_client_v17__listdescriptors {
    () => {
        impl Client {
            pub fn listdescriptors(&self, private: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(private) = private {
                    params.push(into_json(private)?);
                }
                self.call("listdescriptors", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listlabels` for version v17
///
/// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
#[macro_export]
macro_rules! impl_client_v17__listlabels {
    () => {
        impl Client {
            pub fn listlabels(&self, purpose: Option<String>) -> Result<Vec<String>> {
                let mut params = vec![];
                if let Some(purpose) = purpose {
                    params.push(into_json(purpose)?);
                }
                self.call("listlabels", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listlockunspent` for version v17
///
/// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
#[macro_export]
macro_rules! impl_client_v17__listlockunspent {
    () => {
        impl Client {
            pub fn listlockunspent(&self) -> Result<Vec<serde_json::Value>> {
                self.call("listlockunspent", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listreceivedbyaddress` for version v17
///
/// List balances by receiving address.
#[macro_export]
macro_rules! impl_client_v17__listreceivedbyaddress {
    () => {
        impl Client {
            pub fn listreceivedbyaddress(&self, minconf: Option<i64>, include_empty: Option<bool>, include_watchonly: Option<bool>, address_filter: Option<String>, include_immature_coinbase: Option<bool>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![];
                if let Some(minconf) = minconf {
                    params.push(into_json(minconf)?);
                }
                if let Some(include_empty) = include_empty {
                    params.push(into_json(include_empty)?);
                }
                if let Some(include_watchonly) = include_watchonly {
                    params.push(into_json(include_watchonly)?);
                }
                if let Some(address_filter) = address_filter {
                    params.push(into_json(address_filter)?);
                }
                if let Some(include_immature_coinbase) = include_immature_coinbase {
                    params.push(into_json(include_immature_coinbase)?);
                }
                self.call("listreceivedbyaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listreceivedbylabel` for version v17
///
/// List received transactions by label.
#[macro_export]
macro_rules! impl_client_v17__listreceivedbylabel {
    () => {
        impl Client {
            pub fn listreceivedbylabel(&self, minconf: Option<i64>, include_empty: Option<bool>, include_watchonly: Option<bool>, include_immature_coinbase: Option<bool>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![];
                if let Some(minconf) = minconf {
                    params.push(into_json(minconf)?);
                }
                if let Some(include_empty) = include_empty {
                    params.push(into_json(include_empty)?);
                }
                if let Some(include_watchonly) = include_watchonly {
                    params.push(into_json(include_watchonly)?);
                }
                if let Some(include_immature_coinbase) = include_immature_coinbase {
                    params.push(into_json(include_immature_coinbase)?);
                }
                self.call("listreceivedbylabel", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listsinceblock` for version v17
///
/// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
#[macro_export]
macro_rules! impl_client_v17__listsinceblock {
    () => {
        impl Client {
            pub fn listsinceblock(&self, blockhash: Option<String>, target_confirmations: Option<i64>, include_watchonly: Option<bool>, include_removed: Option<bool>, include_change: Option<bool>, label: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(blockhash) = blockhash {
                    params.push(into_json(blockhash)?);
                }
                if let Some(target_confirmations) = target_confirmations {
                    params.push(into_json(target_confirmations)?);
                }
                if let Some(include_watchonly) = include_watchonly {
                    params.push(into_json(include_watchonly)?);
                }
                if let Some(include_removed) = include_removed {
                    params.push(into_json(include_removed)?);
                }
                if let Some(include_change) = include_change {
                    params.push(into_json(include_change)?);
                }
                if let Some(label) = label {
                    params.push(into_json(label)?);
                }
                self.call("listsinceblock", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listtransactions` for version v17
///
/// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
#[macro_export]
macro_rules! impl_client_v17__listtransactions {
    () => {
        impl Client {
            pub fn listtransactions(&self, label: Option<String>, count: Option<i64>, skip: Option<i64>, include_watchonly: Option<bool>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![];
                if let Some(label) = label {
                    params.push(into_json(label)?);
                }
                if let Some(count) = count {
                    params.push(into_json(count)?);
                }
                if let Some(skip) = skip {
                    params.push(into_json(skip)?);
                }
                if let Some(include_watchonly) = include_watchonly {
                    params.push(into_json(include_watchonly)?);
                }
                self.call("listtransactions", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listunspent` for version v17
///
/// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
#[macro_export]
macro_rules! impl_client_v17__listunspent {
    () => {
        impl Client {
            pub fn listunspent(&self, minconf: Option<i64>, maxconf: Option<i64>, addresses: Option<Vec<String>>, include_unsafe: Option<bool>, query_options: Option<serde_json::Value>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![];
                if let Some(minconf) = minconf {
                    params.push(into_json(minconf)?);
                }
                if let Some(maxconf) = maxconf {
                    params.push(into_json(maxconf)?);
                }
                if let Some(addresses) = addresses {
                    params.push(into_json(addresses)?);
                }
                if let Some(include_unsafe) = include_unsafe {
                    params.push(into_json(include_unsafe)?);
                }
                if let Some(query_options) = query_options {
                    params.push(into_json(query_options)?);
                }
                self.call("listunspent", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listwalletdir` for version v17
///
/// Returns a list of wallets in the wallet directory.
#[macro_export]
macro_rules! impl_client_v17__listwalletdir {
    () => {
        impl Client {
            pub fn listwalletdir(&self) -> Result<serde_json::Value> {
                self.call("listwalletdir", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listwallets` for version v17
///
/// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
#[macro_export]
macro_rules! impl_client_v17__listwallets {
    () => {
        impl Client {
            pub fn listwallets(&self) -> Result<Vec<String>> {
                self.call("listwallets", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `loadwallet` for version v17
///
/// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
#[macro_export]
macro_rules! impl_client_v17__loadwallet {
    () => {
        impl Client {
            pub fn loadwallet(&self, filename: String, load_on_startup: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(filename)?];
                if let Some(load_on_startup) = load_on_startup {
                    params.push(into_json(load_on_startup)?);
                }
                self.call("loadwallet", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `lockunspent` for version v17
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
#[macro_export]
macro_rules! impl_client_v17__lockunspent {
    () => {
        impl Client {
            pub fn lockunspent(&self, unlock: bool, transactions: Option<Vec<String>>, persistent: Option<bool>) -> Result<bool> {
                let mut params = vec![into_json(unlock)?];
                if let Some(transactions) = transactions {
                    params.push(into_json(transactions)?);
                }
                if let Some(persistent) = persistent {
                    params.push(into_json(persistent)?);
                }
                self.call("lockunspent", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `migratewallet` for version v17
///
/// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
#[macro_export]
macro_rules! impl_client_v17__migratewallet {
    () => {
        impl Client {
            pub fn migratewallet(&self, wallet_name: Option<String>, passphrase: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(wallet_name) = wallet_name {
                    params.push(into_json(wallet_name)?);
                }
                if let Some(passphrase) = passphrase {
                    params.push(into_json(passphrase)?);
                }
                self.call("migratewallet", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `newkeypool` for version v17
///
/// Entirely clears and refills the keypool.
/// WARNING: On non-HD wallets, this will require a new backup immediately, to include the new keys.
/// When restoring a backup of an HD wallet created before the newkeypool command is run, funds received to
/// new addresses may not appear automatically. They have not been lost, but the wallet may not find them.
/// This can be fixed by running the newkeypool command on the backup and then rescanning, so the wallet
/// re-generates the required keys.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__newkeypool {
    () => {
        impl Client {
            pub fn newkeypool(&self) -> Result<()> {
                self.call("newkeypool", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `psbtbumpfee` for version v17
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
#[macro_export]
macro_rules! impl_client_v17__psbtbumpfee {
    () => {
        impl Client {
            pub fn psbtbumpfee(&self, txid: String, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(txid)?];
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("psbtbumpfee", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `removeprunedfunds` for version v17
///
/// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
#[macro_export]
macro_rules! impl_client_v17__removeprunedfunds {
    () => {
        impl Client {
            pub fn removeprunedfunds(&self, txid: String) -> Result<()> {
                self.call("removeprunedfunds", &[into_json(txid)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `rescanblockchain` for version v17
///
/// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster when used on a descriptor wallet
/// and block filters are available (using startup option "-blockfilterindex=1").
#[macro_export]
macro_rules! impl_client_v17__rescanblockchain {
    () => {
        impl Client {
            pub fn rescanblockchain(&self, start_height: Option<i64>, stop_height: Option<i64>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(start_height) = start_height {
                    params.push(into_json(start_height)?);
                }
                if let Some(stop_height) = stop_height {
                    params.push(into_json(stop_height)?);
                }
                self.call("rescanblockchain", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `restorewallet` for version v17
///
/// Restores and loads a wallet from backup.
/// The rescan is significantly faster if a descriptor wallet is restored
/// and block filters are available (using startup option "-blockfilterindex=1").
#[macro_export]
macro_rules! impl_client_v17__restorewallet {
    () => {
        impl Client {
            pub fn restorewallet(&self, wallet_name: String, backup_file: String, load_on_startup: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(wallet_name)?, into_json(backup_file)?];
                if let Some(load_on_startup) = load_on_startup {
                    params.push(into_json(load_on_startup)?);
                }
                self.call("restorewallet", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `send` for version v17
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Send a transaction.
#[macro_export]
macro_rules! impl_client_v17__send {
    () => {
        impl Client {
            pub fn send(&self, outputs: Vec<Output>, conf_target: Option<i64>, estimate_mode: Option<String>, fee_rate: Option<amount>, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(outputs)?];
                if let Some(conf_target) = conf_target {
                    params.push(into_json(conf_target)?);
                }
                if let Some(estimate_mode) = estimate_mode {
                    params.push(into_json(estimate_mode)?);
                }
                if let Some(fee_rate) = fee_rate {
                    params.push(into_json(fee_rate)?);
                }
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("send", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendall` for version v17
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
#[macro_export]
macro_rules! impl_client_v17__sendall {
    () => {
        impl Client {
            pub fn sendall(&self, recipients: Vec<String>, conf_target: Option<i64>, estimate_mode: Option<String>, fee_rate: Option<amount>, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(recipients)?];
                if let Some(conf_target) = conf_target {
                    params.push(into_json(conf_target)?);
                }
                if let Some(estimate_mode) = estimate_mode {
                    params.push(into_json(estimate_mode)?);
                }
                if let Some(fee_rate) = fee_rate {
                    params.push(into_json(fee_rate)?);
                }
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("sendall", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendmany` for version v17
///
/// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__sendmany {
    () => {
        impl Client {
            pub fn sendmany(&self, dummy: Option<String>, amounts: object-user-keys, minconf: Option<i64>, comment: Option<String>, subtractfeefrom: Option<Vec<String>>, replaceable: Option<bool>, conf_target: Option<i64>, estimate_mode: Option<String>, fee_rate: Option<amount>, verbose: Option<bool>) -> Result<hex> {
                let mut params = vec![into_json(amounts)?];
                if let Some(dummy) = dummy {
                    params.push(into_json(dummy)?);
                }
                if let Some(minconf) = minconf {
                    params.push(into_json(minconf)?);
                }
                if let Some(comment) = comment {
                    params.push(into_json(comment)?);
                }
                if let Some(subtractfeefrom) = subtractfeefrom {
                    params.push(into_json(subtractfeefrom)?);
                }
                if let Some(replaceable) = replaceable {
                    params.push(into_json(replaceable)?);
                }
                if let Some(conf_target) = conf_target {
                    params.push(into_json(conf_target)?);
                }
                if let Some(estimate_mode) = estimate_mode {
                    params.push(into_json(estimate_mode)?);
                }
                if let Some(fee_rate) = fee_rate {
                    params.push(into_json(fee_rate)?);
                }
                if let Some(verbose) = verbose {
                    params.push(into_json(verbose)?);
                }
                self.call("sendmany", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendtoaddress` for version v17
///
/// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__sendtoaddress {
    () => {
        impl Client {
            pub fn sendtoaddress(&self, address: String, amount: amount, comment: Option<String>, comment_to: Option<String>, subtractfeefromamount: Option<bool>, replaceable: Option<bool>, conf_target: Option<i64>, estimate_mode: Option<String>, avoid_reuse: Option<bool>, fee_rate: Option<amount>, verbose: Option<bool>) -> Result<hex> {
                let mut params = vec![into_json(address)?, into_json(amount)?];
                if let Some(comment) = comment {
                    params.push(into_json(comment)?);
                }
                if let Some(comment_to) = comment_to {
                    params.push(into_json(comment_to)?);
                }
                if let Some(subtractfeefromamount) = subtractfeefromamount {
                    params.push(into_json(subtractfeefromamount)?);
                }
                if let Some(replaceable) = replaceable {
                    params.push(into_json(replaceable)?);
                }
                if let Some(conf_target) = conf_target {
                    params.push(into_json(conf_target)?);
                }
                if let Some(estimate_mode) = estimate_mode {
                    params.push(into_json(estimate_mode)?);
                }
                if let Some(avoid_reuse) = avoid_reuse {
                    params.push(into_json(avoid_reuse)?);
                }
                if let Some(fee_rate) = fee_rate {
                    params.push(into_json(fee_rate)?);
                }
                if let Some(verbose) = verbose {
                    params.push(into_json(verbose)?);
                }
                self.call("sendtoaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sethdseed` for version v17
///
/// Set or generate a new HD wallet seed. Non-HD wallets will not be upgraded to being a HD wallet. Wallets that are already
/// HD will have a new HD seed set so that new keys added to the keypool will be derived from this new seed.
/// Note that you will need to MAKE A NEW BACKUP of your wallet after setting the HD wallet seed.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// Note: This command is only compatible with legacy wallets.
#[macro_export]
macro_rules! impl_client_v17__sethdseed {
    () => {
        impl Client {
            pub fn sethdseed(&self, newkeypool: Option<bool>, seed: Option<String>) -> Result<()> {
                let mut params = vec![];
                if let Some(newkeypool) = newkeypool {
                    params.push(into_json(newkeypool)?);
                }
                if let Some(seed) = seed {
                    params.push(into_json(seed)?);
                }
                self.call("sethdseed", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setlabel` for version v17
///
/// Sets the label associated with the given address.
#[macro_export]
macro_rules! impl_client_v17__setlabel {
    () => {
        impl Client {
            pub fn setlabel(&self, address: String, label: String) -> Result<()> {
                self.call("setlabel", &[into_json(address)?, into_json(label)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `settxfee` for version v17
///
/// Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
#[macro_export]
macro_rules! impl_client_v17__settxfee {
    () => {
        impl Client {
            pub fn settxfee(&self, amount: amount) -> Result<bool> {
                self.call("settxfee", &[into_json(amount)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setwalletflag` for version v17
///
/// Change the state of the given wallet flag for a wallet.
#[macro_export]
macro_rules! impl_client_v17__setwalletflag {
    () => {
        impl Client {
            pub fn setwalletflag(&self, flag: String, value: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(flag)?];
                if let Some(value) = value {
                    params.push(into_json(value)?);
                }
                self.call("setwalletflag", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signmessage` for version v17
///
/// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__signmessage {
    () => {
        impl Client {
            pub fn signmessage(&self, address: String, message: String) -> Result<String> {
                self.call("signmessage", &[into_json(address)?, into_json(message)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signrawtransactionwithwallet` for version v17
///
/// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__signrawtransactionwithwallet {
    () => {
        impl Client {
            pub fn signrawtransactionwithwallet(&self, hexstring: String, prevtxs: Option<Vec<String>>, sighashtype: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(hexstring)?];
                if let Some(prevtxs) = prevtxs {
                    params.push(into_json(prevtxs)?);
                }
                if let Some(sighashtype) = sighashtype {
                    params.push(into_json(sighashtype)?);
                }
                self.call("signrawtransactionwithwallet", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `simulaterawtransaction` for version v17
///
/// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
#[macro_export]
macro_rules! impl_client_v17__simulaterawtransaction {
    () => {
        impl Client {
            pub fn simulaterawtransaction(&self, rawtxs: Option<Vec<String>>, options: Option<serde_json::Value>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(rawtxs) = rawtxs {
                    params.push(into_json(rawtxs)?);
                }
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                self.call("simulaterawtransaction", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `unloadwallet` for version v17
///
/// Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// Specifying the wallet name on a wallet endpoint is invalid.
#[macro_export]
macro_rules! impl_client_v17__unloadwallet {
    () => {
        impl Client {
            pub fn unloadwallet(&self, wallet_name: Option<String>, load_on_startup: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(wallet_name) = wallet_name {
                    params.push(into_json(wallet_name)?);
                }
                if let Some(load_on_startup) = load_on_startup {
                    params.push(into_json(load_on_startup)?);
                }
                self.call("unloadwallet", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `upgradewallet` for version v17
///
/// Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// New keys may be generated and a new wallet backup will need to be made.
#[macro_export]
macro_rules! impl_client_v17__upgradewallet {
    () => {
        impl Client {
            pub fn upgradewallet(&self, version: Option<i64>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(version) = version {
                    params.push(into_json(version)?);
                }
                self.call("upgradewallet", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletcreatefundedpsbt` for version v17
///
/// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
#[macro_export]
macro_rules! impl_client_v17__walletcreatefundedpsbt {
    () => {
        impl Client {
            pub fn walletcreatefundedpsbt(&self, inputs: Option<Vec<Input>>, outputs: Vec<Output>, locktime: Option<i64>, options: Option<serde_json::Value>, bip32derivs: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(outputs)?];
                if let Some(inputs) = inputs {
                    params.push(into_json(inputs)?);
                }
                if let Some(locktime) = locktime {
                    params.push(into_json(locktime)?);
                }
                if let Some(options) = options {
                    params.push(into_json(options)?);
                }
                if let Some(bip32derivs) = bip32derivs {
                    params.push(into_json(bip32derivs)?);
                }
                self.call("walletcreatefundedpsbt", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletdisplayaddress` for version v17
///
/// Display address on an external signer for verification.
#[macro_export]
macro_rules! impl_client_v17__walletdisplayaddress {
    () => {
        impl Client {
            pub fn walletdisplayaddress(&self, address: String) -> Result<serde_json::Value> {
                self.call("walletdisplayaddress", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletlock` for version v17
///
/// Removes the wallet encryption key from memory, locking the wallet.
/// After calling this method, you will need to call walletpassphrase again
/// before being able to call any methods which require the wallet to be unlocked.
#[macro_export]
macro_rules! impl_client_v17__walletlock {
    () => {
        impl Client {
            pub fn walletlock(&self) -> Result<()> {
                self.call("walletlock", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletpassphrase` for version v17
///
/// Stores the wallet decryption key in memory for 'timeout' seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
#[macro_export]
macro_rules! impl_client_v17__walletpassphrase {
    () => {
        impl Client {
            pub fn walletpassphrase(&self, passphrase: String, timeout: i64) -> Result<()> {
                self.call("walletpassphrase", &[into_json(passphrase)?, into_json(timeout)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletpassphrasechange` for version v17
///
/// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
#[macro_export]
macro_rules! impl_client_v17__walletpassphrasechange {
    () => {
        impl Client {
            pub fn walletpassphrasechange(&self, oldpassphrase: String, newpassphrase: String) -> Result<()> {
                self.call("walletpassphrasechange", &[into_json(oldpassphrase)?, into_json(newpassphrase)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `walletprocesspsbt` for version v17
///
/// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
#[macro_export]
macro_rules! impl_client_v17__walletprocesspsbt {
    () => {
        impl Client {
            pub fn walletprocesspsbt(&self, psbt: String, sign: Option<bool>, sighashtype: Option<String>, bip32derivs: Option<bool>, finalize: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(psbt)?];
                if let Some(sign) = sign {
                    params.push(into_json(sign)?);
                }
                if let Some(sighashtype) = sighashtype {
                    params.push(into_json(sighashtype)?);
                }
                if let Some(bip32derivs) = bip32derivs {
                    params.push(into_json(bip32derivs)?);
                }
                if let Some(finalize) = finalize {
                    params.push(into_json(finalize)?);
                }
                self.call("walletprocesspsbt", &params)
            }
        }
    };
}

