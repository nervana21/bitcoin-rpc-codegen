/// Implements Bitcoin Core JSON-RPC API method `createmultisig` for version v27
///
/// Creates a multi-signature address with n signature of m keys required.
/// It returns a json object with the address and redeemScript.
#[macro_export]
macro_rules! impl_client_v27__createmultisig {
    () => {
        impl Client {
            pub fn createmultisig(&self, nrequired: i64, keys: Vec<String>, address_type: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(nrequired)?, into_json(keys)?];
                if let Some(address_type) = address_type {
                    params.push(into_json(address_type)?);
                }
                self.call("createmultisig", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `deriveaddresses` for version v27
///
/// Derives one or more addresses corresponding to an output descriptor.
/// Examples of output descriptors are:
/// pkh(<pubkey>)                                     P2PKH outputs for the given pubkey
/// wpkh(<pubkey>)                                    Native segwit P2PKH outputs for the given pubkey
/// sh(multi(<n>,<pubkey>,<pubkey>,...))              P2SH-multisig outputs for the given threshold and pubkeys
/// raw(<hex script>)                                 Outputs whose output script equals the specified hex-encoded bytes
/// tr(<pubkey>,multi_a(<n>,<pubkey>,<pubkey>,...))   P2TR-multisig outputs for the given threshold and pubkeys
/// In the above, <pubkey> either refers to a fixed public key in hexadecimal notation, or to an xpub/xprv optionally followed by one
/// or more path elements separated by "/", where "h" represents a hardened child key.
/// For more information on output descriptors, see the documentation in the doc/descriptors.md file.
#[macro_export]
macro_rules! impl_client_v27__deriveaddresses {
    () => {
        impl Client {
            pub fn deriveaddresses(&self, descriptor: String, range: Option<range>) -> Result<Vec<String>> {
                let mut params = vec![into_json(descriptor)?];
                if let Some(range) = range {
                    params.push(into_json(range)?);
                }
                self.call("deriveaddresses", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `estimatesmartfee` for version v27
///
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible and return the number of blocks
/// for which the estimate is valid. Uses virtual transaction size as defined
/// in BIP 141 (witness data is discounted).
#[macro_export]
macro_rules! impl_client_v27__estimatesmartfee {
    () => {
        impl Client {
            pub fn estimatesmartfee(&self, conf_target: i64, estimate_mode: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(conf_target)?];
                if let Some(estimate_mode) = estimate_mode {
                    params.push(into_json(estimate_mode)?);
                }
                self.call("estimatesmartfee", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getdescriptorinfo` for version v27
///
/// Analyses a descriptor.
#[macro_export]
macro_rules! impl_client_v27__getdescriptorinfo {
    () => {
        impl Client {
            pub fn getdescriptorinfo(&self, descriptor: String) -> Result<serde_json::Value> {
                self.call("getdescriptorinfo", &[into_json(descriptor)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getindexinfo` for version v27
///
/// Returns the status of one or all available indices currently running in the node.
#[macro_export]
macro_rules! impl_client_v27__getindexinfo {
    () => {
        impl Client {
            pub fn getindexinfo(&self, index_name: Option<String>) -> Result<object_dynamic> {
                let mut params = vec![];
                if let Some(index_name) = index_name {
                    params.push(into_json(index_name)?);
                }
                self.call("getindexinfo", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `signmessagewithprivkey` for version v27
///
/// Sign a message with the private key of an address
#[macro_export]
macro_rules! impl_client_v27__signmessagewithprivkey {
    () => {
        impl Client {
            pub fn signmessagewithprivkey(&self, privkey: String, message: String) -> Result<String> {
                self.call("signmessagewithprivkey", &[into_json(privkey)?, into_json(message)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `validateaddress` for version v27
///
/// Return information about the given bitcoin address.
#[macro_export]
macro_rules! impl_client_v27__validateaddress {
    () => {
        impl Client {
            pub fn validateaddress(&self, address: String) -> Result<serde_json::Value> {
                self.call("validateaddress", &[into_json(address)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `verifymessage` for version v27
///
/// Verify a signed message.
#[macro_export]
macro_rules! impl_client_v27__verifymessage {
    () => {
        impl Client {
            pub fn verifymessage(&self, address: String, signature: String, message: String) -> Result<bool> {
                self.call("verifymessage", &[into_json(address)?, into_json(signature)?, into_json(message)?])
            }
        }
    };
}

