/// /// Mark in-wallet transaction <txid> as abandoned
/// This will mark this transaction and all its in-wallet descendants as abandoned which will allow
/// for their inputs to be respent.  It can be used to replace "stuck" or evicted transactions.
/// It only works on transactions which are not included in a block and are not currently in the mempool.
/// It has no effect on transactions which are already abandoned.
macro_rules! impl_client_v29__abandontransaction {
    () => {
        pub fn abandontransaction(
            &self,
            txid: String
        ) -> RpcResult<AbandontransactionResponse> {
            self.call("abandontransaction", json!([txid: String]))
        }
    };
}


/// /// Stops current wallet rescan triggered by an RPC call, e.g. by an importprivkey call.
/// Note: Use "getwalletinfo" to query the scanning progress.
macro_rules! impl_client_v29__abortrescan {
    () => {
        pub fn abortrescan(
            &self,
            
        ) -> RpcResult<AbortrescanResponse> {
            self.call("abortrescan", json!([]))
        }
    };
}


/// /// Add an nrequired-to-sign multisignature address to the wallet. Requires a new wallet backup.
/// Each key is a Bitcoin address or hex-encoded public key.
/// This functionality is only intended for use with non-watchonly addresses.
/// See `importaddress` for watchonly p2sh address support.
/// If 'label' is specified, assign address to that label.
/// Note: This command is only compatible with legacy wallets.
macro_rules! impl_client_v29__addmultisigaddress {
    () => {
        pub fn addmultisigaddress(
            &self,
            nrequired: String, keys: String, label: String, address_type: String
        ) -> RpcResult<AddmultisigaddressResponse> {
            self.call("addmultisigaddress", json!([nrequired: String, keys: String, label: String, address_type: String]))
        }
    };
}


/// /// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
macro_rules! impl_client_v29__addnode {
    () => {
        pub fn addnode(
            &self,
            node: String, command: String, v2transport: String
        ) -> RpcResult<AddnodeResponse> {
            self.call("addnode", json!([node: String, command: String, v2transport: String]))
        }
    };
}


/// /// Analyzes and provides information about the current status of a PSBT and its inputs
macro_rules! impl_client_v29__analyzepsbt {
    () => {
        pub fn analyzepsbt(
            &self,
            psbt: String
        ) -> RpcResult<AnalyzepsbtResponse> {
            self.call("analyzepsbt", json!([psbt: String]))
        }
    };
}


/// /// Safely copies the current wallet file to the specified destination, which can either be a directory or a path with a filename.
macro_rules! impl_client_v29__backupwallet {
    () => {
        pub fn backupwallet(
            &self,
            destination: String
        ) -> RpcResult<BackupwalletResponse> {
            self.call("backupwallet", json!([destination: String]))
        }
    };
}


/// /// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
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
macro_rules! impl_client_v29__bumpfee {
    () => {
        pub fn bumpfee(
            &self,
            txid: String, options: String
        ) -> RpcResult<BumpfeeResponse> {
            self.call("bumpfee", json!([txid: String, options: String]))
        }
    };
}


/// /// Clear all banned IPs.
macro_rules! impl_client_v29__clearbanned {
    () => {
        pub fn clearbanned(
            &self,
            
        ) -> RpcResult<ClearbannedResponse> {
            self.call("clearbanned", json!([]))
        }
    };
}


/// /// Combine multiple partially signed Bitcoin transactions into one transaction.
/// Implements the Combiner role.
macro_rules! impl_client_v29__combinepsbt {
    () => {
        pub fn combinepsbt(
            &self,
            txs: String
        ) -> RpcResult<CombinepsbtResponse> {
            self.call("combinepsbt", json!([txs: String]))
        }
    };
}


/// /// Combine multiple partially signed transactions into one transaction.
/// The combined transaction may be another partially signed transaction or a
/// fully signed transaction.
macro_rules! impl_client_v29__combinerawtransaction {
    () => {
        pub fn combinerawtransaction(
            &self,
            txs: String
        ) -> RpcResult<CombinerawtransactionResponse> {
            self.call("combinerawtransaction", json!([txs: String]))
        }
    };
}


/// /// Converts a network serialized transaction to a PSBT. This should be used only with createrawtransaction and fundrawtransaction
/// createpsbt and walletcreatefundedpsbt should be used for new applications.
macro_rules! impl_client_v29__converttopsbt {
    () => {
        pub fn converttopsbt(
            &self,
            hexstring: String, permitsigdata: String, iswitness: String
        ) -> RpcResult<ConverttopsbtResponse> {
            self.call("converttopsbt", json!([hexstring: String, permitsigdata: String, iswitness: String]))
        }
    };
}


/// /// Creates a multi-signature address with n signature of m keys required.
/// It returns a json object with the address and redeemScript.
macro_rules! impl_client_v29__createmultisig {
    () => {
        pub fn createmultisig(
            &self,
            nrequired: String, keys: String, address_type: String
        ) -> RpcResult<CreatemultisigResponse> {
            self.call("createmultisig", json!([nrequired: String, keys: String, address_type: String]))
        }
    };
}


/// /// Creates a transaction in the Partially Signed Transaction format.
/// Implements the Creator role.
macro_rules! impl_client_v29__createpsbt {
    () => {
        pub fn createpsbt(
            &self,
            inputs: String, outputs: String, locktime: String, replaceable: String
        ) -> RpcResult<CreatepsbtResponse> {
            self.call("createpsbt", json!([inputs: String, outputs: String, locktime: String, replaceable: String]))
        }
    };
}


/// /// Create a transaction spending the given inputs and creating new outputs.
/// Outputs can be addresses or data.
/// Returns hex-encoded raw transaction.
/// Note that the transaction's inputs are not signed, and
/// it is not stored in the wallet or transmitted to the network.
macro_rules! impl_client_v29__createrawtransaction {
    () => {
        pub fn createrawtransaction(
            &self,
            inputs: String, outputs: String, locktime: String, replaceable: String
        ) -> RpcResult<CreaterawtransactionResponse> {
            self.call("createrawtransaction", json!([inputs: String, outputs: String, locktime: String, replaceable: String]))
        }
    };
}


/// /// Creates and loads a new wallet.
macro_rules! impl_client_v29__createwallet {
    () => {
        pub fn createwallet(
            &self,
            wallet_name: String, disable_private_keys: String, blank: String, passphrase: String, avoid_reuse: String, descriptors: String, load_on_startup: String, external_signer: String
        ) -> RpcResult<CreatewalletResponse> {
            self.call("createwallet", json!([wallet_name: String, disable_private_keys: String, blank: String, passphrase: String, avoid_reuse: String, descriptors: String, load_on_startup: String, external_signer: String]))
        }
    };
}


/// /// Creates the wallet's descriptor for the given address type. The address type must be one that the wallet does not already have a descriptor for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__createwalletdescriptor {
    () => {
        pub fn createwalletdescriptor(
            &self,
            type: String, options: String
        ) -> RpcResult<CreatewalletdescriptorResponse> {
            self.call("createwalletdescriptor", json!([type: String, options: String]))
        }
    };
}


/// /// Return a JSON object representing the serialized, base64-encoded partially signed Bitcoin transaction.
macro_rules! impl_client_v29__decodepsbt {
    () => {
        pub fn decodepsbt(
            &self,
            psbt: String
        ) -> RpcResult<DecodepsbtResponse> {
            self.call("decodepsbt", json!([psbt: String]))
        }
    };
}


/// /// Return a JSON object representing the serialized, hex-encoded transaction.
macro_rules! impl_client_v29__decoderawtransaction {
    () => {
        pub fn decoderawtransaction(
            &self,
            hexstring: String, iswitness: String
        ) -> RpcResult<DecoderawtransactionResponse> {
            self.call("decoderawtransaction", json!([hexstring: String, iswitness: String]))
        }
    };
}


/// /// Decode a hex-encoded script.
macro_rules! impl_client_v29__decodescript {
    () => {
        pub fn decodescript(
            &self,
            hexstring: String
        ) -> RpcResult<DecodescriptResponse> {
            self.call("decodescript", json!([hexstring: String]))
        }
    };
}


/// /// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
/// pkh(<pubkey>)                                     P2PKH outputs for the given pubkey
/// wpkh(<pubkey>)                                    Native segwit P2PKH outputs for the given pubkey
/// sh(multi(<n>,<pubkey>,<pubkey>,...))              P2SH-multisig outputs for the given threshold and pubkeys
/// raw(<hex script>)                                 Outputs whose output script equals the specified hex-encoded bytes
/// tr(<pubkey>,multi_a(<n>,<pubkey>,<pubkey>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
macro_rules! impl_client_v29__deriveaddresses {
    () => {
        pub fn deriveaddresses(
            &self,
            descriptor: String, range: String
        ) -> RpcResult<DeriveaddressesResponse> {
            self.call("deriveaddresses", json!([descriptor: String, range: String]))
        }
    };
}


/// /// Update all segwit inputs in a PSBT with information from output descriptors, the UTXO set or the mempool.
/// Then, sign the inputs we are able to with information from the output descriptors.
macro_rules! impl_client_v29__descriptorprocesspsbt {
    () => {
        pub fn descriptorprocesspsbt(
            &self,
            psbt: String, descriptors: String, sighashtype: String, bip32derivs: String, finalize: String
        ) -> RpcResult<DescriptorprocesspsbtResponse> {
            self.call("descriptorprocesspsbt", json!([psbt: String, descriptors: String, sighashtype: String, bip32derivs: String, finalize: String]))
        }
    };
}


/// /// Immediately disconnects from the specified peer node.
/// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
/// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
macro_rules! impl_client_v29__disconnectnode {
    () => {
        pub fn disconnectnode(
            &self,
            address: String, nodeid: String
        ) -> RpcResult<DisconnectnodeResponse> {
            self.call("disconnectnode", json!([address: String, nodeid: String]))
        }
    };
}


/// /// Reveals the private key corresponding to 'address'.
/// Then the importprivkey can be used with this output
/// Note: This command is only compatible with legacy wallets.
macro_rules! impl_client_v29__dumpprivkey {
    () => {
        pub fn dumpprivkey(
            &self,
            address: String
        ) -> RpcResult<DumpprivkeyResponse> {
            self.call("dumpprivkey", json!([address: String]))
        }
    };
}


/// /// Write the serialized UTXO set to a file. This can be used in loadtxoutset afterwards if this snapshot height is supported in the chainparams as well.
/// Unless the "latest" type is requested, the node will roll back to the requested height and network activity will be suspended during this process. Because of this it is discouraged to interact with the node in any other way during the execution of this call to avoid inconsistent results and race conditions, particularly RPCs that interact with blockstorage.
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
macro_rules! impl_client_v29__dumptxoutset {
    () => {
        pub fn dumptxoutset(
            &self,
            path: String, type: String, options: String
        ) -> RpcResult<DumptxoutsetResponse> {
            self.call("dumptxoutset", json!([path: String, type: String, options: String]))
        }
    };
}


/// /// Dumps all wallet keys in a human-readable format to a server-side file. This does not allow overwriting existing files.
/// Imported scripts are included in the dumpfile, but corresponding BIP173 addresses, etc. may not be added automatically by importwallet.
/// Note that if your wallet contains keys which are not derived from your HD seed (e.g. imported keys), these are not covered by
/// only backing up the seed itself, and must be backed up too (e.g. ensure you back up the whole dumpfile).
/// Note: This command is only compatible with legacy wallets.
macro_rules! impl_client_v29__dumpwallet {
    () => {
        pub fn dumpwallet(
            &self,
            filename: String
        ) -> RpcResult<DumpwalletResponse> {
            self.call("dumpwallet", json!([filename: String]))
        }
    };
}


/// /// Encrypts the wallet with 'passphrase'. This is for first time encryption.
/// After this, any calls that interact with private keys such as sending or signing
/// will require the passphrase to be set prior the making these calls.
/// Use the walletpassphrase call for this, and then walletlock call.
/// If the wallet is already encrypted, use the walletpassphrasechange call.
/// ** IMPORTANT **
/// For security reasons, the encryption process will generate a new HD seed, resulting
/// in the creation of a fresh set of active descriptors. Therefore, it is crucial to
/// securely back up the newly generated wallet file using the backupwallet RPC.
macro_rules! impl_client_v29__encryptwallet {
    () => {
        pub fn encryptwallet(
            &self,
            passphrase: String
        ) -> RpcResult<EncryptwalletResponse> {
            self.call("encryptwallet", json!([passphrase: String]))
        }
    };
}


/// /// Returns a list of external signers from -signer.
macro_rules! impl_client_v29__enumeratesigners {
    () => {
        pub fn enumeratesigners(
            &self,
            
        ) -> RpcResult<EnumeratesignersResponse> {
            self.call("enumeratesigners", json!([]))
        }
    };
}


/// /// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
macro_rules! impl_client_v29__estimatesmartfee {
    () => {
        pub fn estimatesmartfee(
            &self,
            conf_target: String, estimate_mode: String
        ) -> RpcResult<EstimatesmartfeeResponse> {
            self.call("estimatesmartfee", json!([conf_target: String, estimate_mode: String]))
        }
    };
}


/// /// Finalize the inputs of a PSBT. If the transaction is fully signed, it will produce a
/// network serialized transaction which can be broadcast with sendrawtransaction. Otherwise a PSBT will be
/// created which has the final_scriptSig and final_scriptWitness fields filled for inputs that are complete.
/// Implements the Finalizer and Extractor roles.
macro_rules! impl_client_v29__finalizepsbt {
    () => {
        pub fn finalizepsbt(
            &self,
            psbt: String, extract: String
        ) -> RpcResult<FinalizepsbtResponse> {
            self.call("finalizepsbt", json!([psbt: String, extract: String]))
        }
    };
}


/// /// If the transaction has no inputs, they will be automatically selected to meet its out value.
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
macro_rules! impl_client_v29__fundrawtransaction {
    () => {
        pub fn fundrawtransaction(
            &self,
            hexstring: String, options: String, iswitness: String
        ) -> RpcResult<FundrawtransactionResponse> {
            self.call("fundrawtransaction", json!([hexstring: String, options: String, iswitness: String]))
        }
    };
}


/// /// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
macro_rules! impl_client_v29__getaddednodeinfo {
    () => {
        pub fn getaddednodeinfo(
            &self,
            node: String
        ) -> RpcResult<GetaddednodeinfoResponse> {
            self.call("getaddednodeinfo", json!([node: String]))
        }
    };
}


/// /// Returns the list of addresses assigned the specified label.
macro_rules! impl_client_v29__getaddressesbylabel {
    () => {
        pub fn getaddressesbylabel(
            &self,
            label: String
        ) -> RpcResult<GetaddressesbylabelResponse> {
            self.call("getaddressesbylabel", json!([label: String]))
        }
    };
}


/// /// Return information about the given bitcoin address.
/// Some of the information will only be present if the address is in the active wallet.
macro_rules! impl_client_v29__getaddressinfo {
    () => {
        pub fn getaddressinfo(
            &self,
            address: String
        ) -> RpcResult<GetaddressinfoResponse> {
            self.call("getaddressinfo", json!([address: String]))
        }
    };
}


/// /// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
macro_rules! impl_client_v29__getaddrmaninfo {
    () => {
        pub fn getaddrmaninfo(
            &self,
            
        ) -> RpcResult<GetaddrmaninfoResponse> {
            self.call("getaddrmaninfo", json!([]))
        }
    };
}


/// /// Returns the total available balance.
/// The available balance is what the wallet considers currently spendable, and is
/// thus affected by options which limit spendability such as -spendzeroconfchange.
macro_rules! impl_client_v29__getbalance {
    () => {
        pub fn getbalance(
            &self,
            dummy: String, minconf: String, include_watchonly: String, avoid_reuse: String
        ) -> RpcResult<GetbalanceResponse> {
            self.call("getbalance", json!([dummy: String, minconf: String, include_watchonly: String, avoid_reuse: String]))
        }
    };
}


/// /// Returns an object with all balances in BTC.
macro_rules! impl_client_v29__getbalances {
    () => {
        pub fn getbalances(
            &self,
            
        ) -> RpcResult<GetbalancesResponse> {
            self.call("getbalances", json!([]))
        }
    };
}


/// /// Returns the hash of the best (tip) block in the most-work fully-validated chain.
macro_rules! impl_client_v29__getbestblockhash {
    () => {
        pub fn getbestblockhash(
            &self,
            
        ) -> RpcResult<GetbestblockhashResponse> {
            self.call("getbestblockhash", json!([]))
        }
    };
}


/// /// If verbosity is 0, returns a string that is serialized, hex-encoded data for block 'hash'.
/// If verbosity is 1, returns an Object with information about block <hash>.
/// If verbosity is 2, returns an Object with information about block <hash> and information about each transaction.
/// If verbosity is 3, returns an Object with information about block <hash> and information about each transaction, including prevout information for inputs (only for unpruned blocks in the current best chain).
macro_rules! impl_client_v29__getblock {
    () => {
        pub fn getblock(
            &self,
            blockhash: String, verbosity: String
        ) -> RpcResult<GetblockResponse> {
            self.call("getblock", json!([blockhash: String, verbosity: String]))
        }
    };
}


/// /// Returns an object containing various state info regarding blockchain processing.
macro_rules! impl_client_v29__getblockchaininfo {
    () => {
        pub fn getblockchaininfo(
            &self,
            
        ) -> RpcResult<GetblockchaininfoResponse> {
            self.call("getblockchaininfo", json!([]))
        }
    };
}


/// /// Returns the height of the most-work fully-validated chain.
/// The genesis block has height 0.
macro_rules! impl_client_v29__getblockcount {
    () => {
        pub fn getblockcount(
            &self,
            
        ) -> RpcResult<GetblockcountResponse> {
            self.call("getblockcount", json!([]))
        }
    };
}


/// /// Retrieve a BIP 157 content filter for a particular block.
macro_rules! impl_client_v29__getblockfilter {
    () => {
        pub fn getblockfilter(
            &self,
            blockhash: String, filtertype: String
        ) -> RpcResult<GetblockfilterResponse> {
            self.call("getblockfilter", json!([blockhash: String, filtertype: String]))
        }
    };
}


/// /// Attempt to fetch block from a given peer.
/// We must have the header for this block, e.g. using submitheader.
/// The block will not have any undo data which can limit the usage of the block data in a context where the undo data is needed.
/// Subsequent calls for the same block may cause the response from the previous peer to be ignored.
/// Peers generally ignore requests for a stale block that they never fully verified, or one that is more than a month old.
/// When a peer does not respond with a block, we will disconnect.
/// Note: The block could be re-pruned as soon as it is received.
/// Returns an empty JSON object if the request was successfully scheduled.
macro_rules! impl_client_v29__getblockfrompeer {
    () => {
        pub fn getblockfrompeer(
            &self,
            blockhash: String, peer_id: String
        ) -> RpcResult<GetblockfrompeerResponse> {
            self.call("getblockfrompeer", json!([blockhash: String, peer_id: String]))
        }
    };
}


/// /// Returns hash of block in best-block-chain at height provided.
macro_rules! impl_client_v29__getblockhash {
    () => {
        pub fn getblockhash(
            &self,
            height: String
        ) -> RpcResult<GetblockhashResponse> {
            self.call("getblockhash", json!([height: String]))
        }
    };
}


/// /// If verbose is false, returns a string that is serialized, hex-encoded data for blockheader 'hash'.
/// If verbose is true, returns an Object with information about blockheader <hash>.
macro_rules! impl_client_v29__getblockheader {
    () => {
        pub fn getblockheader(
            &self,
            blockhash: String, verbose: String
        ) -> RpcResult<GetblockheaderResponse> {
            self.call("getblockheader", json!([blockhash: String, verbose: String]))
        }
    };
}


/// /// Compute per block statistics for a given window. All amounts are in satoshis.
/// It won't work for some heights with pruning.
macro_rules! impl_client_v29__getblockstats {
    () => {
        pub fn getblockstats(
            &self,
            hash_or_height: String, stats: String
        ) -> RpcResult<GetblockstatsResponse> {
            self.call("getblockstats", json!([hash_or_height: String, stats: String]))
        }
    };
}


/// /// If the request parameters include a 'mode' key, that is used to explicitly select between the default 'template' request or a 'proposal'.
/// It returns data needed to construct a block to work on.
/// For full specification, see BIPs 22, 23, 9, and 145:
/// https://github.com/bitcoin/bips/blob/master/bip-0022.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0023.mediawiki
/// https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki#getblocktemplate_changes
/// https://github.com/bitcoin/bips/blob/master/bip-0145.mediawiki
macro_rules! impl_client_v29__getblocktemplate {
    () => {
        pub fn getblocktemplate(
            &self,
            template_request: String
        ) -> RpcResult<GetblocktemplateResponse> {
            self.call("getblocktemplate", json!([template_request: String]))
        }
    };
}


/// /// Return information about chainstates.
macro_rules! impl_client_v29__getchainstates {
    () => {
        pub fn getchainstates(
            &self,
            
        ) -> RpcResult<GetchainstatesResponse> {
            self.call("getchainstates", json!([]))
        }
    };
}


/// /// Return information about all known tips in the block tree, including the main chain as well as orphaned branches.
macro_rules! impl_client_v29__getchaintips {
    () => {
        pub fn getchaintips(
            &self,
            
        ) -> RpcResult<GetchaintipsResponse> {
            self.call("getchaintips", json!([]))
        }
    };
}


/// /// Compute statistics about the total number and rate of transactions in the chain.
macro_rules! impl_client_v29__getchaintxstats {
    () => {
        pub fn getchaintxstats(
            &self,
            nblocks: String, blockhash: String
        ) -> RpcResult<GetchaintxstatsResponse> {
            self.call("getchaintxstats", json!([nblocks: String, blockhash: String]))
        }
    };
}


/// /// Returns the number of connections to other nodes.
macro_rules! impl_client_v29__getconnectioncount {
    () => {
        pub fn getconnectioncount(
            &self,
            
        ) -> RpcResult<GetconnectioncountResponse> {
            self.call("getconnectioncount", json!([]))
        }
    };
}


/// /// Returns an object containing various state info regarding deployments of consensus changes.
macro_rules! impl_client_v29__getdeploymentinfo {
    () => {
        pub fn getdeploymentinfo(
            &self,
            blockhash: String
        ) -> RpcResult<GetdeploymentinfoResponse> {
            self.call("getdeploymentinfo", json!([blockhash: String]))
        }
    };
}


/// /// Get spend and receive activity associated with a set of descriptors for a set of blocks. This command pairs well with the `relevant_blocks` output of `scanblocks()`.
/// This call may take several minutes. If you encounter timeouts, try specifying no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
macro_rules! impl_client_v29__getdescriptoractivity {
    () => {
        pub fn getdescriptoractivity(
            &self,
            blockhashes: String, scanobjects: String, include_mempool: String
        ) -> RpcResult<GetdescriptoractivityResponse> {
            self.call("getdescriptoractivity", json!([blockhashes: String, scanobjects: String, include_mempool: String]))
        }
    };
}


/// /// Analyses a descriptor.
macro_rules! impl_client_v29__getdescriptorinfo {
    () => {
        pub fn getdescriptorinfo(
            &self,
            descriptor: String
        ) -> RpcResult<GetdescriptorinfoResponse> {
            self.call("getdescriptorinfo", json!([descriptor: String]))
        }
    };
}


/// /// Returns the proof-of-work difficulty as a multiple of the minimum difficulty.
macro_rules! impl_client_v29__getdifficulty {
    () => {
        pub fn getdifficulty(
            &self,
            
        ) -> RpcResult<GetdifficultyResponse> {
            self.call("getdifficulty", json!([]))
        }
    };
}


/// /// List all BIP 32 HD keys in the wallet and which descriptors use them.
macro_rules! impl_client_v29__gethdkeys {
    () => {
        pub fn gethdkeys(
            &self,
            options: String
        ) -> RpcResult<GethdkeysResponse> {
            self.call("gethdkeys", json!([options: String]))
        }
    };
}


/// /// Returns the status of one or all available indices currently running in the node.
macro_rules! impl_client_v29__getindexinfo {
    () => {
        pub fn getindexinfo(
            &self,
            index_name: String
        ) -> RpcResult<GetindexinfoResponse> {
            self.call("getindexinfo", json!([index_name: String]))
        }
    };
}


/// /// Returns an object containing information about memory usage.
macro_rules! impl_client_v29__getmemoryinfo {
    () => {
        pub fn getmemoryinfo(
            &self,
            mode: String
        ) -> RpcResult<GetmemoryinfoResponse> {
            self.call("getmemoryinfo", json!([mode: String]))
        }
    };
}


/// /// If txid is in the mempool, returns all in-mempool ancestors.
macro_rules! impl_client_v29__getmempoolancestors {
    () => {
        pub fn getmempoolancestors(
            &self,
            txid: String, verbose: String
        ) -> RpcResult<GetmempoolancestorsResponse> {
            self.call("getmempoolancestors", json!([txid: String, verbose: String]))
        }
    };
}


/// /// If txid is in the mempool, returns all in-mempool descendants.
macro_rules! impl_client_v29__getmempooldescendants {
    () => {
        pub fn getmempooldescendants(
            &self,
            txid: String, verbose: String
        ) -> RpcResult<GetmempooldescendantsResponse> {
            self.call("getmempooldescendants", json!([txid: String, verbose: String]))
        }
    };
}


/// /// Returns mempool data for given transaction
macro_rules! impl_client_v29__getmempoolentry {
    () => {
        pub fn getmempoolentry(
            &self,
            txid: String
        ) -> RpcResult<GetmempoolentryResponse> {
            self.call("getmempoolentry", json!([txid: String]))
        }
    };
}


/// /// Returns details on the active state of the TX memory pool.
macro_rules! impl_client_v29__getmempoolinfo {
    () => {
        pub fn getmempoolinfo(
            &self,
            
        ) -> RpcResult<GetmempoolinfoResponse> {
            self.call("getmempoolinfo", json!([]))
        }
    };
}


/// /// Returns a json object containing mining-related information.
macro_rules! impl_client_v29__getmininginfo {
    () => {
        pub fn getmininginfo(
            &self,
            
        ) -> RpcResult<GetmininginfoResponse> {
            self.call("getmininginfo", json!([]))
        }
    };
}


/// /// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
macro_rules! impl_client_v29__getnettotals {
    () => {
        pub fn getnettotals(
            &self,
            
        ) -> RpcResult<GetnettotalsResponse> {
            self.call("getnettotals", json!([]))
        }
    };
}


/// /// Returns the estimated network hashes per second based on the last n blocks.
/// Pass in [blocks] to override # of blocks, -1 specifies since last difficulty change.
/// Pass in [height] to estimate the network speed at the time when a certain block was found.
macro_rules! impl_client_v29__getnetworkhashps {
    () => {
        pub fn getnetworkhashps(
            &self,
            nblocks: String, height: String
        ) -> RpcResult<GetnetworkhashpsResponse> {
            self.call("getnetworkhashps", json!([nblocks: String, height: String]))
        }
    };
}


/// /// Returns an object containing various state info regarding P2P networking.
macro_rules! impl_client_v29__getnetworkinfo {
    () => {
        pub fn getnetworkinfo(
            &self,
            
        ) -> RpcResult<GetnetworkinfoResponse> {
            self.call("getnetworkinfo", json!([]))
        }
    };
}


/// /// Returns a new Bitcoin address for receiving payments.
/// If 'label' is specified, it is added to the address book
/// so payments received with the address will be associated with 'label'.
macro_rules! impl_client_v29__getnewaddress {
    () => {
        pub fn getnewaddress(
            &self,
            label: String, address_type: String
        ) -> RpcResult<GetnewaddressResponse> {
            self.call("getnewaddress", json!([label: String, address_type: String]))
        }
    };
}


/// /// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
macro_rules! impl_client_v29__getnodeaddresses {
    () => {
        pub fn getnodeaddresses(
            &self,
            count: String, network: String
        ) -> RpcResult<GetnodeaddressesResponse> {
            self.call("getnodeaddresses", json!([count: String, network: String]))
        }
    };
}


/// /// Returns data about each connected network peer as a json array of objects.
macro_rules! impl_client_v29__getpeerinfo {
    () => {
        pub fn getpeerinfo(
            &self,
            
        ) -> RpcResult<GetpeerinfoResponse> {
            self.call("getpeerinfo", json!([]))
        }
    };
}


/// /// Returns a map of all user-created (see prioritisetransaction) fee deltas by txid, and whether the tx is present in mempool.
macro_rules! impl_client_v29__getprioritisedtransactions {
    () => {
        pub fn getprioritisedtransactions(
            &self,
            
        ) -> RpcResult<GetprioritisedtransactionsResponse> {
            self.call("getprioritisedtransactions", json!([]))
        }
    };
}


/// /// Returns a new Bitcoin address, for receiving change.
/// This is for use with raw transactions, NOT normal use.
macro_rules! impl_client_v29__getrawchangeaddress {
    () => {
        pub fn getrawchangeaddress(
            &self,
            address_type: String
        ) -> RpcResult<GetrawchangeaddressResponse> {
            self.call("getrawchangeaddress", json!([address_type: String]))
        }
    };
}


/// /// Returns all transaction ids in memory pool as a json array of string transaction ids.
/// Hint: use getmempoolentry to fetch a specific transaction from the mempool.
macro_rules! impl_client_v29__getrawmempool {
    () => {
        pub fn getrawmempool(
            &self,
            verbose: String, mempool_sequence: String
        ) -> RpcResult<GetrawmempoolResponse> {
            self.call("getrawmempool", json!([verbose: String, mempool_sequence: String]))
        }
    };
}


/// /// By default, this call only returns a transaction if it is in the mempool. If -txindex is enabled
/// and no blockhash argument is passed, it will return the transaction if it is in the mempool or any block.
/// If a blockhash argument is passed, it will return the transaction if
/// the specified block is available and the transaction is in that block.
/// Hint: Use gettransaction for wallet transactions.
/// If verbosity is 0 or omitted, returns the serialized transaction as a hex-encoded string.
/// If verbosity is 1, returns a JSON Object with information about the transaction.
/// If verbosity is 2, returns a JSON Object with information about the transaction, including fee and prevout information.
macro_rules! impl_client_v29__getrawtransaction {
    () => {
        pub fn getrawtransaction(
            &self,
            txid: String, verbosity: String, blockhash: String
        ) -> RpcResult<GetrawtransactionResponse> {
            self.call("getrawtransaction", json!([txid: String, verbosity: String, blockhash: String]))
        }
    };
}


/// /// Returns the total amount received by the given address in transactions with at least minconf confirmations.
macro_rules! impl_client_v29__getreceivedbyaddress {
    () => {
        pub fn getreceivedbyaddress(
            &self,
            address: String, minconf: String, include_immature_coinbase: String
        ) -> RpcResult<GetreceivedbyaddressResponse> {
            self.call("getreceivedbyaddress", json!([address: String, minconf: String, include_immature_coinbase: String]))
        }
    };
}


/// /// Returns the total amount received by addresses with <label> in transactions with at least [minconf] confirmations.
macro_rules! impl_client_v29__getreceivedbylabel {
    () => {
        pub fn getreceivedbylabel(
            &self,
            label: String, minconf: String, include_immature_coinbase: String
        ) -> RpcResult<GetreceivedbylabelResponse> {
            self.call("getreceivedbylabel", json!([label: String, minconf: String, include_immature_coinbase: String]))
        }
    };
}


/// /// Returns details of the RPC server.
macro_rules! impl_client_v29__getrpcinfo {
    () => {
        pub fn getrpcinfo(
            &self,
            
        ) -> RpcResult<GetrpcinfoResponse> {
            self.call("getrpcinfo", json!([]))
        }
    };
}


/// /// Get detailed information about in-wallet transaction <txid>
macro_rules! impl_client_v29__gettransaction {
    () => {
        pub fn gettransaction(
            &self,
            txid: String, include_watchonly: String, verbose: String
        ) -> RpcResult<GettransactionResponse> {
            self.call("gettransaction", json!([txid: String, include_watchonly: String, verbose: String]))
        }
    };
}


/// /// Returns details about an unspent transaction output.
macro_rules! impl_client_v29__gettxout {
    () => {
        pub fn gettxout(
            &self,
            txid: String, n: String, include_mempool: String
        ) -> RpcResult<GettxoutResponse> {
            self.call("gettxout", json!([txid: String, n: String, include_mempool: String]))
        }
    };
}


/// /// Returns a hex-encoded proof that "txid" was included in a block.
/// NOTE: By default this function only works sometimes. This is when there is an
/// unspent output in the utxo for this transaction. To make it always work,
/// you need to maintain a transaction index, using the -txindex command line option or
/// specify the block in which the transaction is included manually (by blockhash).
macro_rules! impl_client_v29__gettxoutproof {
    () => {
        pub fn gettxoutproof(
            &self,
            txids: String, blockhash: String
        ) -> RpcResult<GettxoutproofResponse> {
            self.call("gettxoutproof", json!([txids: String, blockhash: String]))
        }
    };
}


/// /// Returns statistics about the unspent transaction output set.
/// Note this call may take some time if you are not using coinstatsindex.
macro_rules! impl_client_v29__gettxoutsetinfo {
    () => {
        pub fn gettxoutsetinfo(
            &self,
            hash_type: String, hash_or_height: String, use_index: String
        ) -> RpcResult<GettxoutsetinfoResponse> {
            self.call("gettxoutsetinfo", json!([hash_type: String, hash_or_height: String, use_index: String]))
        }
    };
}


/// /// Scans the mempool to find transactions spending any of the given outputs
macro_rules! impl_client_v29__gettxspendingprevout {
    () => {
        pub fn gettxspendingprevout(
            &self,
            outputs: String
        ) -> RpcResult<GettxspendingprevoutResponse> {
            self.call("gettxspendingprevout", json!([outputs: String]))
        }
    };
}


/// /// DEPRECATED
/// Identical to getbalances().mine.untrusted_pending
macro_rules! impl_client_v29__getunconfirmedbalance {
    () => {
        pub fn getunconfirmedbalance(
            &self,
            
        ) -> RpcResult<GetunconfirmedbalanceResponse> {
            self.call("getunconfirmedbalance", json!([]))
        }
    };
}


/// /// Returns an object containing various wallet state info.
macro_rules! impl_client_v29__getwalletinfo {
    () => {
        pub fn getwalletinfo(
            &self,
            
        ) -> RpcResult<GetwalletinfoResponse> {
            self.call("getwalletinfo", json!([]))
        }
    };
}


/// /// Returns information about the active ZeroMQ notifications.
macro_rules! impl_client_v29__getzmqnotifications {
    () => {
        pub fn getzmqnotifications(
            &self,
            
        ) -> RpcResult<GetzmqnotificationsResponse> {
            self.call("getzmqnotifications", json!([]))
        }
    };
}


/// /// List all commands, or get help for a specified command.
macro_rules! impl_client_v29__help {
    () => {
        pub fn help(
            &self,
            command: String
        ) -> RpcResult<HelpResponse> {
            self.call("help", json!([command: String]))
        }
    };
}


/// /// Adds an address or script (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
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
macro_rules! impl_client_v29__importaddress {
    () => {
        pub fn importaddress(
            &self,
            address: String, label: String, rescan: String, p2sh: String
        ) -> RpcResult<ImportaddressResponse> {
            self.call("importaddress", json!([address: String, label: String, rescan: String, p2sh: String]))
        }
    };
}


/// /// Import descriptors. This will trigger a rescan of the blockchain based on the earliest timestamp of all descriptors being imported. Requires a new wallet backup.
/// When importing descriptors with multipath key expressions, if the multipath specifier contains exactly two elements, the descriptor produced from the second elements will be imported as an internal descriptor.
/// Note: This call can take over an hour to complete if using an early timestamp; during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan is significantly faster if block filters are available (using startup option "-blockfilterindex=1").
macro_rules! impl_client_v29__importdescriptors {
    () => {
        pub fn importdescriptors(
            &self,
            requests: String
        ) -> RpcResult<ImportdescriptorsResponse> {
            self.call("importdescriptors", json!([requests: String]))
        }
    };
}


/// /// Import a mempool.dat file and attempt to add its contents to the mempool.
/// Warning: Importing untrusted files is dangerous, especially if metadata from the file is taken over.
macro_rules! impl_client_v29__importmempool {
    () => {
        pub fn importmempool(
            &self,
            filepath: String, options: String
        ) -> RpcResult<ImportmempoolResponse> {
            self.call("importmempool", json!([filepath: String, options: String]))
        }
    };
}


/// /// Import addresses/scripts (with private or public keys, redeem script (P2SH)), optionally rescanning the blockchain from the earliest creation time of the imported scripts. Requires a new wallet backup.
/// If an address/script is imported without all of the private keys required to spend from that address, it will be watchonly. The 'watchonly' option must be set to true in this case or a warning will be returned.
/// Conversely, if all the private keys are provided and the address/script is spendable, the watchonly option must be set to false, or a warning will be returned.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported keys, addresses or scripts exist but related transactions are still missing.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" for descriptor wallets.
macro_rules! impl_client_v29__importmulti {
    () => {
        pub fn importmulti(
            &self,
            requests: String, options: String
        ) -> RpcResult<ImportmultiResponse> {
            self.call("importmulti", json!([requests: String, options: String]))
        }
    };
}


/// /// Adds a private key (as returned by dumpprivkey) to your wallet. Requires a new wallet backup.
/// Hint: use importmulti to import more than one private key.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported key exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" with "combo(X)" for descriptor wallets.
macro_rules! impl_client_v29__importprivkey {
    () => {
        pub fn importprivkey(
            &self,
            privkey: String, label: String, rescan: String
        ) -> RpcResult<ImportprivkeyResponse> {
            self.call("importprivkey", json!([privkey: String, label: String, rescan: String]))
        }
    };
}


/// /// Imports funds without rescan. Corresponding address or script must previously be included in wallet. Aimed towards pruned wallets. The end-user is responsible to import additional transactions that subsequently spend the imported outputs or rescan after the point in the blockchain the transaction is included.
macro_rules! impl_client_v29__importprunedfunds {
    () => {
        pub fn importprunedfunds(
            &self,
            rawtransaction: String, txoutproof: String
        ) -> RpcResult<ImportprunedfundsResponse> {
            self.call("importprunedfunds", json!([rawtransaction: String, txoutproof: String]))
        }
    };
}


/// /// Adds a public key (in hex) that can be watched as if it were in your wallet but cannot be used to spend. Requires a new wallet backup.
/// Hint: use importmulti to import more than one public key.
/// Note: This call can take over an hour to complete if rescan is true, during that time, other rpc calls
/// may report that the imported pubkey exists but related transactions are still missing, leading to temporarily incorrect/bogus balances and unspent outputs until rescan completes.
/// The rescan parameter can be set to false if the key was never used to create transactions. If it is set to false,
/// but the key was used to create transactions, rescanblockchain needs to be called with the appropriate block range.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets. Use "importdescriptors" with "combo(X)" for descriptor wallets.
macro_rules! impl_client_v29__importpubkey {
    () => {
        pub fn importpubkey(
            &self,
            pubkey: String, label: String, rescan: String
        ) -> RpcResult<ImportpubkeyResponse> {
            self.call("importpubkey", json!([pubkey: String, label: String, rescan: String]))
        }
    };
}


/// /// Imports keys from a wallet dump file (see dumpwallet). Requires a new wallet backup to include imported keys.
/// Note: Blockchain and Mempool will be rescanned after a successful import. Use "getwalletinfo" to query the scanning progress.
/// Note: This command is only compatible with legacy wallets.
macro_rules! impl_client_v29__importwallet {
    () => {
        pub fn importwallet(
            &self,
            filename: String
        ) -> RpcResult<ImportwalletResponse> {
            self.call("importwallet", json!([filename: String]))
        }
    };
}


/// /// Joins multiple distinct PSBTs with different inputs and outputs into one PSBT with inputs and outputs from all of the PSBTs
/// No input in any of the PSBTs can be in more than one of the PSBTs.
macro_rules! impl_client_v29__joinpsbts {
    () => {
        pub fn joinpsbts(
            &self,
            txs: String
        ) -> RpcResult<JoinpsbtsResponse> {
            self.call("joinpsbts", json!([txs: String]))
        }
    };
}


/// /// Fills the keypool.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__keypoolrefill {
    () => {
        pub fn keypoolrefill(
            &self,
            newsize: String
        ) -> RpcResult<KeypoolrefillResponse> {
            self.call("keypoolrefill", json!([newsize: String]))
        }
    };
}


/// /// Lists groups of addresses which have had their common ownership
/// made public by common use as inputs or as the resulting change
/// in past transactions
macro_rules! impl_client_v29__listaddressgroupings {
    () => {
        pub fn listaddressgroupings(
            &self,
            
        ) -> RpcResult<ListaddressgroupingsResponse> {
            self.call("listaddressgroupings", json!([]))
        }
    };
}


/// /// List all manually banned IPs/Subnets.
macro_rules! impl_client_v29__listbanned {
    () => {
        pub fn listbanned(
            &self,
            
        ) -> RpcResult<ListbannedResponse> {
            self.call("listbanned", json!([]))
        }
    };
}


/// /// List descriptors imported into a descriptor-enabled wallet.
macro_rules! impl_client_v29__listdescriptors {
    () => {
        pub fn listdescriptors(
            &self,
            private: String
        ) -> RpcResult<ListdescriptorsResponse> {
            self.call("listdescriptors", json!([private: String]))
        }
    };
}


/// /// Returns the list of all labels, or labels that are assigned to addresses with a specific purpose.
macro_rules! impl_client_v29__listlabels {
    () => {
        pub fn listlabels(
            &self,
            purpose: String
        ) -> RpcResult<ListlabelsResponse> {
            self.call("listlabels", json!([purpose: String]))
        }
    };
}


/// /// Returns list of temporarily unspendable outputs.
/// See the lockunspent call to lock and unlock transactions for spending.
macro_rules! impl_client_v29__listlockunspent {
    () => {
        pub fn listlockunspent(
            &self,
            
        ) -> RpcResult<ListlockunspentResponse> {
            self.call("listlockunspent", json!([]))
        }
    };
}


/// /// List balances by receiving address.
macro_rules! impl_client_v29__listreceivedbyaddress {
    () => {
        pub fn listreceivedbyaddress(
            &self,
            minconf: String, include_empty: String, include_watchonly: String, address_filter: String, include_immature_coinbase: String
        ) -> RpcResult<ListreceivedbyaddressResponse> {
            self.call("listreceivedbyaddress", json!([minconf: String, include_empty: String, include_watchonly: String, address_filter: String, include_immature_coinbase: String]))
        }
    };
}


/// /// List received transactions by label.
macro_rules! impl_client_v29__listreceivedbylabel {
    () => {
        pub fn listreceivedbylabel(
            &self,
            minconf: String, include_empty: String, include_watchonly: String, include_immature_coinbase: String
        ) -> RpcResult<ListreceivedbylabelResponse> {
            self.call("listreceivedbylabel", json!([minconf: String, include_empty: String, include_watchonly: String, include_immature_coinbase: String]))
        }
    };
}


/// /// Get all transactions in blocks since block [blockhash], or all transactions if omitted.
/// If "blockhash" is no longer a part of the main chain, transactions from the fork point onward are included.
/// Additionally, if include_removed is set, transactions affecting the wallet which were removed are returned in the "removed" array.
macro_rules! impl_client_v29__listsinceblock {
    () => {
        pub fn listsinceblock(
            &self,
            blockhash: String, target_confirmations: String, include_watchonly: String, include_removed: String, include_change: String, label: String
        ) -> RpcResult<ListsinceblockResponse> {
            self.call("listsinceblock", json!([blockhash: String, target_confirmations: String, include_watchonly: String, include_removed: String, include_change: String, label: String]))
        }
    };
}


/// /// If a label name is provided, this will return only incoming transactions paying to addresses with the specified label.
/// Returns up to 'count' most recent transactions skipping the first 'from' transactions.
macro_rules! impl_client_v29__listtransactions {
    () => {
        pub fn listtransactions(
            &self,
            label: String, count: String, skip: String, include_watchonly: String
        ) -> RpcResult<ListtransactionsResponse> {
            self.call("listtransactions", json!([label: String, count: String, skip: String, include_watchonly: String]))
        }
    };
}


/// /// Returns array of unspent transaction outputs
/// with between minconf and maxconf (inclusive) confirmations.
/// Optionally filter to only include txouts paid to specified addresses.
macro_rules! impl_client_v29__listunspent {
    () => {
        pub fn listunspent(
            &self,
            minconf: String, maxconf: String, addresses: String, include_unsafe: String, query_options: String
        ) -> RpcResult<ListunspentResponse> {
            self.call("listunspent", json!([minconf: String, maxconf: String, addresses: String, include_unsafe: String, query_options: String]))
        }
    };
}


/// /// Returns a list of wallets in the wallet directory.
macro_rules! impl_client_v29__listwalletdir {
    () => {
        pub fn listwalletdir(
            &self,
            
        ) -> RpcResult<ListwalletdirResponse> {
            self.call("listwalletdir", json!([]))
        }
    };
}


/// /// Returns a list of currently loaded wallets.
/// For full information on the wallet, use "getwalletinfo"
macro_rules! impl_client_v29__listwallets {
    () => {
        pub fn listwallets(
            &self,
            
        ) -> RpcResult<ListwalletsResponse> {
            self.call("listwallets", json!([]))
        }
    };
}


/// /// Load the serialized UTXO set from a file.
/// Once this snapshot is loaded, its contents will be deserialized into a second chainstate data structure, which is then used to sync to the network's tip. Meanwhile, the original chainstate will complete the initial block download process in the background, eventually validating up to the block that the snapshot is based upon.
/// The result is a usable bitcoind instance that is current with the network tip in a matter of minutes rather than hours. UTXO snapshot are typically obtained from third-party sources (HTTP, torrent, etc.) which is reasonable since their contents are always checked by hash.
/// You can find more information on this process in the `assumeutxo` design document (<https://github.com/bitcoin/bitcoin/blob/master/doc/design/assumeutxo.md>).
macro_rules! impl_client_v29__loadtxoutset {
    () => {
        pub fn loadtxoutset(
            &self,
            path: String
        ) -> RpcResult<LoadtxoutsetResponse> {
            self.call("loadtxoutset", json!([path: String]))
        }
    };
}


/// /// Loads a wallet from a wallet file or directory.
/// Note that all wallet command-line options used when starting bitcoind will be
/// applied to the new wallet.
macro_rules! impl_client_v29__loadwallet {
    () => {
        pub fn loadwallet(
            &self,
            filename: String, load_on_startup: String
        ) -> RpcResult<LoadwalletResponse> {
            self.call("loadwallet", json!([filename: String, load_on_startup: String]))
        }
    };
}


/// /// Updates list of temporarily unspendable outputs.
/// Temporarily lock (unlock=false) or unlock (unlock=true) specified transaction outputs.
/// If no transaction outputs are specified when unlocking then all current locked transaction outputs are unlocked.
/// A locked transaction output will not be chosen by automatic coin selection, when spending bitcoins.
/// Manually selected coins are automatically unlocked.
/// Locks are stored in memory only, unless persistent=true, in which case they will be written to the
/// wallet database and loaded on node start. Unwritten (persistent=false) locks are always cleared
/// (by virtue of process exit) when a node stops or fails. Unlocking will clear both persistent and not.
/// Also see the listunspent call
macro_rules! impl_client_v29__lockunspent {
    () => {
        pub fn lockunspent(
            &self,
            unlock: String, transactions: String, persistent: String
        ) -> RpcResult<LockunspentResponse> {
            self.call("lockunspent", json!([unlock: String, transactions: String, persistent: String]))
        }
    };
}


/// /// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.
macro_rules! impl_client_v29__logging {
    () => {
        pub fn logging(
            &self,
            include: String, exclude: String
        ) -> RpcResult<LoggingResponse> {
            self.call("logging", json!([include: String, exclude: String]))
        }
    };
}


/// /// Migrate the wallet to a descriptor wallet.
/// A new wallet backup will need to be made.
/// The migration process will create a backup of the wallet before migrating. This backup
/// file will be named <wallet name>-<timestamp>.legacy.bak and can be found in the directory
/// for this wallet. In the event of an incorrect migration, the backup can be restored using restorewallet.
/// Encrypted wallets must have the passphrase provided as an argument to this call.
/// This RPC may take a long time to complete. Increasing the RPC client timeout is recommended.
macro_rules! impl_client_v29__migratewallet {
    () => {
        pub fn migratewallet(
            &self,
            wallet_name: String, passphrase: String
        ) -> RpcResult<MigratewalletResponse> {
            self.call("migratewallet", json!([wallet_name: String, passphrase: String]))
        }
    };
}


/// /// Entirely clears and refills the keypool.
/// WARNING: On non-HD wallets, this will require a new backup immediately, to include the new keys.
/// When restoring a backup of an HD wallet created before the newkeypool command is run, funds received to
/// new addresses may not appear automatically. They have not been lost, but the wallet may not find them.
/// This can be fixed by running the newkeypool command on the backup and then rescanning, so the wallet
/// re-generates the required keys.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__newkeypool {
    () => {
        pub fn newkeypool(
            &self,
            
        ) -> RpcResult<NewkeypoolResponse> {
            self.call("newkeypool", json!([]))
        }
    };
}


/// /// Requests that a ping be sent to all other nodes, to measure ping time.
/// Results provided in getpeerinfo, pingtime and pingwait fields are decimal seconds.
/// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
macro_rules! impl_client_v29__ping {
    () => {
        pub fn ping(
            &self,
            
        ) -> RpcResult<PingResponse> {
            self.call("ping", json!([]))
        }
    };
}


/// /// Treats a block as if it were received before others with the same work.
/// A later preciousblock call can override the effect of an earlier one.
/// The effects of preciousblock are not retained across restarts.
macro_rules! impl_client_v29__preciousblock {
    () => {
        pub fn preciousblock(
            &self,
            blockhash: String
        ) -> RpcResult<PreciousblockResponse> {
            self.call("preciousblock", json!([blockhash: String]))
        }
    };
}


/// /// Accepts the transaction into mined blocks at a higher (or lower) priority
macro_rules! impl_client_v29__prioritisetransaction {
    () => {
        pub fn prioritisetransaction(
            &self,
            txid: String, dummy: String, fee_delta: String
        ) -> RpcResult<PrioritisetransactionResponse> {
            self.call("prioritisetransaction", json!([txid: String, dummy: String, fee_delta: String]))
        }
    };
}


/// /// <no description available>
macro_rules! impl_client_v29__pruneblockchain {
    () => {
        pub fn pruneblockchain(
            &self,
            height: String
        ) -> RpcResult<PruneblockchainResponse> {
            self.call("pruneblockchain", json!([height: String]))
        }
    };
}


/// /// Bumps the fee of an opt-in-RBF transaction T, replacing it with a new transaction B.
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
macro_rules! impl_client_v29__psbtbumpfee {
    () => {
        pub fn psbtbumpfee(
            &self,
            txid: String, options: String
        ) -> RpcResult<PsbtbumpfeeResponse> {
            self.call("psbtbumpfee", json!([txid: String, options: String]))
        }
    };
}


/// /// Deletes the specified transaction from the wallet. Meant for use with pruned wallets and as a companion to importprunedfunds. This will affect wallet balances.
macro_rules! impl_client_v29__removeprunedfunds {
    () => {
        pub fn removeprunedfunds(
            &self,
            txid: String
        ) -> RpcResult<RemoveprunedfundsResponse> {
            self.call("removeprunedfunds", json!([txid: String]))
        }
    };
}


/// /// Rescan the local blockchain for wallet related transactions.
/// Note: Use "getwalletinfo" to query the scanning progress.
/// The rescan is significantly faster when used on a descriptor wallet
/// and block filters are available (using startup option "-blockfilterindex=1").
macro_rules! impl_client_v29__rescanblockchain {
    () => {
        pub fn rescanblockchain(
            &self,
            start_height: String, stop_height: String
        ) -> RpcResult<RescanblockchainResponse> {
            self.call("rescanblockchain", json!([start_height: String, stop_height: String]))
        }
    };
}


/// /// Restores and loads a wallet from backup.
/// The rescan is significantly faster if a descriptor wallet is restored
/// and block filters are available (using startup option "-blockfilterindex=1").
macro_rules! impl_client_v29__restorewallet {
    () => {
        pub fn restorewallet(
            &self,
            wallet_name: String, backup_file: String, load_on_startup: String
        ) -> RpcResult<RestorewalletResponse> {
            self.call("restorewallet", json!([wallet_name: String, backup_file: String, load_on_startup: String]))
        }
    };
}


/// /// Dumps the mempool to disk. It will fail until the previous dump is fully loaded.
macro_rules! impl_client_v29__savemempool {
    () => {
        pub fn savemempool(
            &self,
            
        ) -> RpcResult<SavemempoolResponse> {
            self.call("savemempool", json!([]))
        }
    };
}


/// /// Return relevant blockhashes for given descriptors (requires blockfilterindex).
/// This call may take several minutes. Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
macro_rules! impl_client_v29__scanblocks {
    () => {
        pub fn scanblocks(
            &self,
            action: String, scanobjects: String, start_height: String, stop_height: String, filtertype: String, options: String
        ) -> RpcResult<ScanblocksResponse> {
            self.call("scanblocks", json!([action: String, scanobjects: String, start_height: String, stop_height: String, filtertype: String, options: String]))
        }
    };
}


/// /// Scans the unspent transaction output set for entries that match certain output descriptors.
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
macro_rules! impl_client_v29__scantxoutset {
    () => {
        pub fn scantxoutset(
            &self,
            action: String, scanobjects: String
        ) -> RpcResult<ScantxoutsetResponse> {
            self.call("scantxoutset", json!([action: String, scanobjects: String]))
        }
    };
}


/// /// EXPERIMENTAL warning: this call may be changed in future releases.
/// Send a transaction.
macro_rules! impl_client_v29__send {
    () => {
        pub fn send(
            &self,
            outputs: String, conf_target: String, estimate_mode: String, fee_rate: String, options: String
        ) -> RpcResult<SendResponse> {
            self.call("send", json!([outputs: String, conf_target: String, estimate_mode: String, fee_rate: String, options: String]))
        }
    };
}


/// /// EXPERIMENTAL warning: this call may be changed in future releases.
/// Spend the value of all (or specific) confirmed UTXOs and unconfirmed change in the wallet to one or more recipients.
/// Unconfirmed inbound UTXOs and locked UTXOs will not be spent. Sendall will respect the avoid_reuse wallet flag.
/// If your wallet contains many small inputs, either because it received tiny payments or as a result of accumulating change, consider using `send_max` to exclude inputs that are worth less than the fees needed to spend them.
macro_rules! impl_client_v29__sendall {
    () => {
        pub fn sendall(
            &self,
            recipients: String, conf_target: String, estimate_mode: String, fee_rate: String, options: String
        ) -> RpcResult<SendallResponse> {
            self.call("sendall", json!([recipients: String, conf_target: String, estimate_mode: String, fee_rate: String, options: String]))
        }
    };
}


/// /// Send multiple times. Amounts are double-precision floating point numbers.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__sendmany {
    () => {
        pub fn sendmany(
            &self,
            dummy: String, amounts: String, minconf: String, comment: String, subtractfeefrom: String, replaceable: String, conf_target: String, estimate_mode: String, fee_rate: String, verbose: String
        ) -> RpcResult<SendmanyResponse> {
            self.call("sendmany", json!([dummy: String, amounts: String, minconf: String, comment: String, subtractfeefrom: String, replaceable: String, conf_target: String, estimate_mode: String, fee_rate: String, verbose: String]))
        }
    };
}


/// /// Submit a raw transaction (serialized, hex-encoded) to local node and network.
/// The transaction will be sent unconditionally to all peers, so using sendrawtransaction
/// for manual rebroadcast may degrade privacy by leaking the transaction's origin, as
/// nodes will normally not rebroadcast non-wallet transactions already in their mempool.
/// A specific exception, RPC_TRANSACTION_ALREADY_IN_UTXO_SET, may throw if the transaction cannot be added to the mempool.
/// Related RPCs: createrawtransaction, signrawtransactionwithkey
macro_rules! impl_client_v29__sendrawtransaction {
    () => {
        pub fn sendrawtransaction(
            &self,
            hexstring: String, maxfeerate: String, maxburnamount: String
        ) -> RpcResult<SendrawtransactionResponse> {
            self.call("sendrawtransaction", json!([hexstring: String, maxfeerate: String, maxburnamount: String]))
        }
    };
}


/// /// Send an amount to a given address.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__sendtoaddress {
    () => {
        pub fn sendtoaddress(
            &self,
            address: String, amount: String, comment: String, comment_to: String, subtractfeefromamount: String, replaceable: String, conf_target: String, estimate_mode: String, avoid_reuse: String, fee_rate: String, verbose: String
        ) -> RpcResult<SendtoaddressResponse> {
            self.call("sendtoaddress", json!([address: String, amount: String, comment: String, comment_to: String, subtractfeefromamount: String, replaceable: String, conf_target: String, estimate_mode: String, avoid_reuse: String, fee_rate: String, verbose: String]))
        }
    };
}


/// /// Attempts to add or remove an IP/Subnet from the banned list.
macro_rules! impl_client_v29__setban {
    () => {
        pub fn setban(
            &self,
            subnet: String, command: String, bantime: String, absolute: String
        ) -> RpcResult<SetbanResponse> {
            self.call("setban", json!([subnet: String, command: String, bantime: String, absolute: String]))
        }
    };
}


/// /// Set or generate a new HD wallet seed. Non-HD wallets will not be upgraded to being a HD wallet. Wallets that are already
/// HD will have a new HD seed set so that new keys added to the keypool will be derived from this new seed.
/// Note that you will need to MAKE A NEW BACKUP of your wallet after setting the HD wallet seed.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// Note: This command is only compatible with legacy wallets.
macro_rules! impl_client_v29__sethdseed {
    () => {
        pub fn sethdseed(
            &self,
            newkeypool: String, seed: String
        ) -> RpcResult<SethdseedResponse> {
            self.call("sethdseed", json!([newkeypool: String, seed: String]))
        }
    };
}


/// /// Sets the label associated with the given address.
macro_rules! impl_client_v29__setlabel {
    () => {
        pub fn setlabel(
            &self,
            address: String, label: String
        ) -> RpcResult<SetlabelResponse> {
            self.call("setlabel", json!([address: String, label: String]))
        }
    };
}


/// /// Disable/enable all p2p network activity.
macro_rules! impl_client_v29__setnetworkactive {
    () => {
        pub fn setnetworkactive(
            &self,
            state: String
        ) -> RpcResult<SetnetworkactiveResponse> {
            self.call("setnetworkactive", json!([state: String]))
        }
    };
}


/// /// Set the transaction fee rate in BTC/kvB for this wallet. Overrides the global -paytxfee command line parameter.
/// Can be deactivated by passing 0 as the fee. In that case automatic fee selection will be used by default.
macro_rules! impl_client_v29__settxfee {
    () => {
        pub fn settxfee(
            &self,
            amount: String
        ) -> RpcResult<SettxfeeResponse> {
            self.call("settxfee", json!([amount: String]))
        }
    };
}


/// /// Change the state of the given wallet flag for a wallet.
macro_rules! impl_client_v29__setwalletflag {
    () => {
        pub fn setwalletflag(
            &self,
            flag: String, value: String
        ) -> RpcResult<SetwalletflagResponse> {
            self.call("setwalletflag", json!([flag: String, value: String]))
        }
    };
}


/// /// Sign a message with the private key of an address
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__signmessage {
    () => {
        pub fn signmessage(
            &self,
            address: String, message: String
        ) -> RpcResult<SignmessageResponse> {
            self.call("signmessage", json!([address: String, message: String]))
        }
    };
}


/// /// Sign a message with the private key of an address
macro_rules! impl_client_v29__signmessagewithprivkey {
    () => {
        pub fn signmessagewithprivkey(
            &self,
            privkey: String, message: String
        ) -> RpcResult<SignmessagewithprivkeyResponse> {
            self.call("signmessagewithprivkey", json!([privkey: String, message: String]))
        }
    };
}


/// /// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second argument is an array of base58-encoded private
/// keys that will be the only keys used to sign the transaction.
/// The third optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
macro_rules! impl_client_v29__signrawtransactionwithkey {
    () => {
        pub fn signrawtransactionwithkey(
            &self,
            hexstring: String, privkeys: String, prevtxs: String, sighashtype: String
        ) -> RpcResult<SignrawtransactionwithkeyResponse> {
            self.call("signrawtransactionwithkey", json!([hexstring: String, privkeys: String, prevtxs: String, sighashtype: String]))
        }
    };
}


/// /// Sign inputs for raw transaction (serialized, hex-encoded).
/// The second optional argument (may be null) is an array of previous transaction outputs that
/// this transaction depends on but may not yet be in the block chain.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__signrawtransactionwithwallet {
    () => {
        pub fn signrawtransactionwithwallet(
            &self,
            hexstring: String, prevtxs: String, sighashtype: String
        ) -> RpcResult<SignrawtransactionwithwalletResponse> {
            self.call("signrawtransactionwithwallet", json!([hexstring: String, prevtxs: String, sighashtype: String]))
        }
    };
}


/// /// Calculate the balance change resulting in the signing and broadcasting of the given transaction(s).
macro_rules! impl_client_v29__simulaterawtransaction {
    () => {
        pub fn simulaterawtransaction(
            &self,
            rawtxs: String, options: String
        ) -> RpcResult<SimulaterawtransactionResponse> {
            self.call("simulaterawtransaction", json!([rawtxs: String, options: String]))
        }
    };
}


/// /// Request a graceful shutdown of Bitcoin Core.
macro_rules! impl_client_v29__stop {
    () => {
        pub fn stop(
            &self,
            
        ) -> RpcResult<StopResponse> {
            self.call("stop", json!([]))
        }
    };
}


/// /// Attempts to submit new block to network.
/// See https://en.bitcoin.it/wiki/BIP_0022 for full specification.
macro_rules! impl_client_v29__submitblock {
    () => {
        pub fn submitblock(
            &self,
            hexdata: String, dummy: String
        ) -> RpcResult<SubmitblockResponse> {
            self.call("submitblock", json!([hexdata: String, dummy: String]))
        }
    };
}


/// /// Decode the given hexdata as a header and submit it as a candidate chain tip if valid.
/// Throws when the header is invalid.
macro_rules! impl_client_v29__submitheader {
    () => {
        pub fn submitheader(
            &self,
            hexdata: String
        ) -> RpcResult<SubmitheaderResponse> {
            self.call("submitheader", json!([hexdata: String]))
        }
    };
}


/// /// Submit a package of raw transactions (serialized, hex-encoded) to local node.
/// The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
/// This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
/// Warning: successful submission does not mean the transactions will propagate throughout the network.
macro_rules! impl_client_v29__submitpackage {
    () => {
        pub fn submitpackage(
            &self,
            package: String, maxfeerate: String, maxburnamount: String
        ) -> RpcResult<SubmitpackageResponse> {
            self.call("submitpackage", json!([package: String, maxfeerate: String, maxburnamount: String]))
        }
    };
}


/// /// Returns result of mempool acceptance tests indicating if raw transaction(s) (serialized, hex-encoded) would be accepted by mempool.
/// If multiple transactions are passed in, parents must come before children and package policies apply: the transactions cannot conflict with any mempool transactions or each other.
/// If one transaction fails, other transactions may not be fully validated (the 'allowed' key will be blank).
/// The maximum number of transactions allowed is 25.
/// This checks if transactions violate the consensus or policy rules.
/// See sendrawtransaction call.
macro_rules! impl_client_v29__testmempoolaccept {
    () => {
        pub fn testmempoolaccept(
            &self,
            rawtxs: String, maxfeerate: String
        ) -> RpcResult<TestmempoolacceptResponse> {
            self.call("testmempoolaccept", json!([rawtxs: String, maxfeerate: String]))
        }
    };
}


/// /// Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// Specifying the wallet name on a wallet endpoint is invalid.
macro_rules! impl_client_v29__unloadwallet {
    () => {
        pub fn unloadwallet(
            &self,
            wallet_name: String, load_on_startup: String
        ) -> RpcResult<UnloadwalletResponse> {
            self.call("unloadwallet", json!([wallet_name: String, load_on_startup: String]))
        }
    };
}


/// /// Upgrade the wallet. Upgrades to the latest version if no version number is specified.
/// New keys may be generated and a new wallet backup will need to be made.
macro_rules! impl_client_v29__upgradewallet {
    () => {
        pub fn upgradewallet(
            &self,
            version: String
        ) -> RpcResult<UpgradewalletResponse> {
            self.call("upgradewallet", json!([version: String]))
        }
    };
}


/// /// Returns the total uptime of the server.
macro_rules! impl_client_v29__uptime {
    () => {
        pub fn uptime(
            &self,
            
        ) -> RpcResult<UptimeResponse> {
            self.call("uptime", json!([]))
        }
    };
}


/// /// Updates all segwit inputs and outputs in a PSBT with data from output descriptors, the UTXO set, txindex, or the mempool.
macro_rules! impl_client_v29__utxoupdatepsbt {
    () => {
        pub fn utxoupdatepsbt(
            &self,
            psbt: String, descriptors: String
        ) -> RpcResult<UtxoupdatepsbtResponse> {
            self.call("utxoupdatepsbt", json!([psbt: String, descriptors: String]))
        }
    };
}


/// /// Return information about the given bitcoin address.
macro_rules! impl_client_v29__validateaddress {
    () => {
        pub fn validateaddress(
            &self,
            address: String
        ) -> RpcResult<ValidateaddressResponse> {
            self.call("validateaddress", json!([address: String]))
        }
    };
}


/// /// Verifies blockchain database.
macro_rules! impl_client_v29__verifychain {
    () => {
        pub fn verifychain(
            &self,
            checklevel: String, nblocks: String
        ) -> RpcResult<VerifychainResponse> {
            self.call("verifychain", json!([checklevel: String, nblocks: String]))
        }
    };
}


/// /// Verify a signed message.
macro_rules! impl_client_v29__verifymessage {
    () => {
        pub fn verifymessage(
            &self,
            address: String, signature: String, message: String
        ) -> RpcResult<VerifymessageResponse> {
            self.call("verifymessage", json!([address: String, signature: String, message: String]))
        }
    };
}


/// /// Verifies that a proof points to a transaction in a block, returning the transaction it commits to
/// and throwing an RPC error if the block is not in our best chain
macro_rules! impl_client_v29__verifytxoutproof {
    () => {
        pub fn verifytxoutproof(
            &self,
            proof: String
        ) -> RpcResult<VerifytxoutproofResponse> {
            self.call("verifytxoutproof", json!([proof: String]))
        }
    };
}


/// /// Creates and funds a transaction in the Partially Signed Transaction format.
/// Implements the Creator and Updater roles.
/// All existing inputs must either have their previous output transaction be in the wallet
/// or be in the UTXO set. Solving data must be provided for non-wallet inputs.
macro_rules! impl_client_v29__walletcreatefundedpsbt {
    () => {
        pub fn walletcreatefundedpsbt(
            &self,
            inputs: String, outputs: String, locktime: String, options: String, bip32derivs: String
        ) -> RpcResult<WalletcreatefundedpsbtResponse> {
            self.call("walletcreatefundedpsbt", json!([inputs: String, outputs: String, locktime: String, options: String, bip32derivs: String]))
        }
    };
}


/// /// Display address on an external signer for verification.
macro_rules! impl_client_v29__walletdisplayaddress {
    () => {
        pub fn walletdisplayaddress(
            &self,
            address: String
        ) -> RpcResult<WalletdisplayaddressResponse> {
            self.call("walletdisplayaddress", json!([address: String]))
        }
    };
}


/// /// Removes the wallet encryption key from memory, locking the wallet.
/// After calling this method, you will need to call walletpassphrase again
/// before being able to call any methods which require the wallet to be unlocked.
macro_rules! impl_client_v29__walletlock {
    () => {
        pub fn walletlock(
            &self,
            
        ) -> RpcResult<WalletlockResponse> {
            self.call("walletlock", json!([]))
        }
    };
}


/// /// Stores the wallet decryption key in memory for 'timeout' seconds.
/// This is needed prior to performing transactions related to private keys such as sending bitcoins
/// Note:
/// Issuing the walletpassphrase command while the wallet is already unlocked will set a new unlock
/// time that overrides the old one.
macro_rules! impl_client_v29__walletpassphrase {
    () => {
        pub fn walletpassphrase(
            &self,
            passphrase: String, timeout: String
        ) -> RpcResult<WalletpassphraseResponse> {
            self.call("walletpassphrase", json!([passphrase: String, timeout: String]))
        }
    };
}


/// /// Changes the wallet passphrase from 'oldpassphrase' to 'newpassphrase'.
macro_rules! impl_client_v29__walletpassphrasechange {
    () => {
        pub fn walletpassphrasechange(
            &self,
            oldpassphrase: String, newpassphrase: String
        ) -> RpcResult<WalletpassphrasechangeResponse> {
            self.call("walletpassphrasechange", json!([oldpassphrase: String, newpassphrase: String]))
        }
    };
}


/// /// Update a PSBT with input information from our wallet and then sign inputs
/// that we can sign for.
/// Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
macro_rules! impl_client_v29__walletprocesspsbt {
    () => {
        pub fn walletprocesspsbt(
            &self,
            psbt: String, sign: String, sighashtype: String, bip32derivs: String, finalize: String
        ) -> RpcResult<WalletprocesspsbtResponse> {
            self.call("walletprocesspsbt", json!([psbt: String, sign: String, sighashtype: String, bip32derivs: String, finalize: String]))
        }
    };
}


