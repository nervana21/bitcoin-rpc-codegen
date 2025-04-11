/// Implements Bitcoin Core JSON-RPC API method `addconnection` for version v17
///
/// Open an outbound connection to a specified node. This RPC is for testing only.
#[macro_export]
macro_rules! impl_client_v17__addconnection {
    () => {
        impl Client {
            pub fn addconnection(&self, address: String, connection_type: String, v2transport: bool) -> Result<serde_json::Value> {
                self.call("addconnection", &[into_json(address)?, into_json(connection_type)?, into_json(v2transport)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `addpeeraddress` for version v17
///
/// Add the address of a potential peer to an address manager table. This RPC is for testing only.
#[macro_export]
macro_rules! impl_client_v17__addpeeraddress {
    () => {
        impl Client {
            pub fn addpeeraddress(&self, address: String, port: i64, tried: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(address)?, into_json(port)?];
                if let Some(tried) = tried {
                    params.push(into_json(tried)?);
                }
                self.call("addpeeraddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `echo` for version v17
///
/// Simply echo back the input arguments. This command is for testing.
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[macro_export]
macro_rules! impl_client_v17__echo {
    () => {
        impl Client {
            pub fn echo(&self, arg0: Option<String>, arg1: Option<String>, arg2: Option<String>, arg3: Option<String>, arg4: Option<String>, arg5: Option<String>, arg6: Option<String>, arg7: Option<String>, arg8: Option<String>, arg9: Option<String>) -> Result<any> {
                let mut params = vec![];
                if let Some(arg0) = arg0 {
                    params.push(into_json(arg0)?);
                }
                if let Some(arg1) = arg1 {
                    params.push(into_json(arg1)?);
                }
                if let Some(arg2) = arg2 {
                    params.push(into_json(arg2)?);
                }
                if let Some(arg3) = arg3 {
                    params.push(into_json(arg3)?);
                }
                if let Some(arg4) = arg4 {
                    params.push(into_json(arg4)?);
                }
                if let Some(arg5) = arg5 {
                    params.push(into_json(arg5)?);
                }
                if let Some(arg6) = arg6 {
                    params.push(into_json(arg6)?);
                }
                if let Some(arg7) = arg7 {
                    params.push(into_json(arg7)?);
                }
                if let Some(arg8) = arg8 {
                    params.push(into_json(arg8)?);
                }
                if let Some(arg9) = arg9 {
                    params.push(into_json(arg9)?);
                }
                self.call("echo", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `echoipc` for version v17
///
/// Echo back the input argument, passing it through a spawned process in a multiprocess build.
/// This command is for testing.
#[macro_export]
macro_rules! impl_client_v17__echoipc {
    () => {
        impl Client {
            pub fn echoipc(&self, arg: String) -> Result<String> {
                self.call("echoipc", &[into_json(arg)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `echojson` for version v17
///
/// Simply echo back the input arguments. This command is for testing.
/// It will return an internal bug report when arg9='trigger_internal_bug' is passed.
/// The difference between echo and echojson is that echojson has argument conversion enabled in the client-side table in bitcoin-cli and the GUI. There is no server-side difference.
#[macro_export]
macro_rules! impl_client_v17__echojson {
    () => {
        impl Client {
            pub fn echojson(&self, arg0: Option<String>, arg1: Option<String>, arg2: Option<String>, arg3: Option<String>, arg4: Option<String>, arg5: Option<String>, arg6: Option<String>, arg7: Option<String>, arg8: Option<String>, arg9: Option<String>) -> Result<any> {
                let mut params = vec![];
                if let Some(arg0) = arg0 {
                    params.push(into_json(arg0)?);
                }
                if let Some(arg1) = arg1 {
                    params.push(into_json(arg1)?);
                }
                if let Some(arg2) = arg2 {
                    params.push(into_json(arg2)?);
                }
                if let Some(arg3) = arg3 {
                    params.push(into_json(arg3)?);
                }
                if let Some(arg4) = arg4 {
                    params.push(into_json(arg4)?);
                }
                if let Some(arg5) = arg5 {
                    params.push(into_json(arg5)?);
                }
                if let Some(arg6) = arg6 {
                    params.push(into_json(arg6)?);
                }
                if let Some(arg7) = arg7 {
                    params.push(into_json(arg7)?);
                }
                if let Some(arg8) = arg8 {
                    params.push(into_json(arg8)?);
                }
                if let Some(arg9) = arg9 {
                    params.push(into_json(arg9)?);
                }
                self.call("echojson", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `estimaterawfee` for version v17
///
/// WARNING: This interface is unstable and may disappear or change!
/// WARNING: This is an advanced API call that is tightly coupled to the specific
/// implementation of fee estimation. The parameters it can be called with
/// and the results it returns will change if the internal implementation changes.
/// Estimates the approximate fee per kilobyte needed for a transaction to begin
/// confirmation within conf_target blocks if possible. Uses virtual transaction size as
/// defined in BIP 141 (witness data is discounted).
#[macro_export]
macro_rules! impl_client_v17__estimaterawfee {
    () => {
        impl Client {
            pub fn estimaterawfee(&self, conf_target: i64, threshold: Option<i64>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(conf_target)?];
                if let Some(threshold) = threshold {
                    params.push(into_json(threshold)?);
                }
                self.call("estimaterawfee", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `generate` for version v17
///
/// has been replaced by the -generate cli option. Refer to -help for more information.
#[macro_export]
macro_rules! impl_client_v17__generate {
    () => {
        impl Client {
            pub fn generate(&self) -> Result<()> {
                self.call("generate", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `generateblock` for version v17
///
/// Mine a set of ordered transactions to a specified address or descriptor and return the block hash.
#[macro_export]
macro_rules! impl_client_v17__generateblock {
    () => {
        impl Client {
            pub fn generateblock(&self, output: String, transactions: Vec<String>, submit: Option<bool>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(output)?, into_json(transactions)?];
                if let Some(submit) = submit {
                    params.push(into_json(submit)?);
                }
                self.call("generateblock", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `generatetoaddress` for version v17
///
/// Mine to a specified address and return the block hashes.
#[macro_export]
macro_rules! impl_client_v17__generatetoaddress {
    () => {
        impl Client {
            pub fn generatetoaddress(&self, nblocks: i64, address: String, maxtries: Option<i64>) -> Result<Vec<Hex>> {
                let mut params = vec![into_json(nblocks)?, into_json(address)?];
                if let Some(maxtries) = maxtries {
                    params.push(into_json(maxtries)?);
                }
                self.call("generatetoaddress", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `generatetodescriptor` for version v17
///
/// Mine to a specified descriptor and return the block hashes.
#[macro_export]
macro_rules! impl_client_v17__generatetodescriptor {
    () => {
        impl Client {
            pub fn generatetodescriptor(&self, num_blocks: i64, descriptor: String, maxtries: Option<i64>) -> Result<Vec<Hex>> {
                let mut params = vec![into_json(num_blocks)?, into_json(descriptor)?];
                if let Some(maxtries) = maxtries {
                    params.push(into_json(maxtries)?);
                }
                self.call("generatetodescriptor", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getorphantxs` for version v17
///
/// Shows transactions in the tx orphanage.
/// EXPERIMENTAL warning: this call may be changed in future releases.
#[macro_export]
macro_rules! impl_client_v17__getorphantxs {
    () => {
        impl Client {
            pub fn getorphantxs(&self, verbosity: Option<i64>) -> Result<Vec<Hex>> {
                let mut params = vec![];
                if let Some(verbosity) = verbosity {
                    params.push(into_json(verbosity)?);
                }
                self.call("getorphantxs", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrawaddrman` for version v17
///
/// EXPERIMENTAL warning: this call may be changed in future releases.
/// Returns information on all address manager entries for the new and tried tables.
#[macro_export]
macro_rules! impl_client_v17__getrawaddrman {
    () => {
        impl Client {
            pub fn getrawaddrman(&self) -> Result<object_dynamic> {
                self.call("getrawaddrman", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `invalidateblock` for version v17
///
/// Permanently marks a block as invalid, as if it violated a consensus rule.
#[macro_export]
macro_rules! impl_client_v17__invalidateblock {
    () => {
        impl Client {
            pub fn invalidateblock(&self, blockhash: String) -> Result<()> {
                self.call("invalidateblock", &[into_json(blockhash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `mockscheduler` for version v17
///
/// Bump the scheduler into the future (-regtest only)
#[macro_export]
macro_rules! impl_client_v17__mockscheduler {
    () => {
        impl Client {
            pub fn mockscheduler(&self, delta_time: i64) -> Result<()> {
                self.call("mockscheduler", &[into_json(delta_time)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `reconsiderblock` for version v17
///
/// Removes invalidity status of a block, its ancestors and its descendants, reconsider them for activation.
/// This can be used to undo the effects of invalidateblock.
#[macro_export]
macro_rules! impl_client_v17__reconsiderblock {
    () => {
        impl Client {
            pub fn reconsiderblock(&self, blockhash: String) -> Result<()> {
                self.call("reconsiderblock", &[into_json(blockhash)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `sendmsgtopeer` for version v17
///
/// Send a p2p message to a peer specified by id.
/// The message type and body must be provided, the message header will be generated.
/// This RPC is for testing only.
#[macro_export]
macro_rules! impl_client_v17__sendmsgtopeer {
    () => {
        impl Client {
            pub fn sendmsgtopeer(&self, peer_id: i64, msg_type: String, msg: String) -> Result<serde_json::Value> {
                self.call("sendmsgtopeer", &[into_json(peer_id)?, into_json(msg_type)?, into_json(msg)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setmocktime` for version v17
///
/// Set the local time to given timestamp (-regtest only)
#[macro_export]
macro_rules! impl_client_v17__setmocktime {
    () => {
        impl Client {
            pub fn setmocktime(&self, timestamp: i64) -> Result<()> {
                self.call("setmocktime", &[into_json(timestamp)?])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `syncwithvalidationinterfacequeue` for version v17
///
/// Waits for the validation interface queue to catch up on everything that was there when we entered this function.
#[macro_export]
macro_rules! impl_client_v17__syncwithvalidationinterfacequeue {
    () => {
        impl Client {
            pub fn syncwithvalidationinterfacequeue(&self) -> Result<()> {
                self.call("syncwithvalidationinterfacequeue", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `waitforblock` for version v17
///
/// Waits for a specific new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[macro_export]
macro_rules! impl_client_v17__waitforblock {
    () => {
        impl Client {
            pub fn waitforblock(&self, blockhash: String, timeout: Option<i64>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(blockhash)?];
                if let Some(timeout) = timeout {
                    params.push(into_json(timeout)?);
                }
                self.call("waitforblock", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `waitforblockheight` for version v17
///
/// Waits for (at least) block height and returns the height and hash
/// of the current tip.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[macro_export]
macro_rules! impl_client_v17__waitforblockheight {
    () => {
        impl Client {
            pub fn waitforblockheight(&self, height: i64, timeout: Option<i64>) -> Result<serde_json::Value> {
                let mut params = vec![into_json(height)?];
                if let Some(timeout) = timeout {
                    params.push(into_json(timeout)?);
                }
                self.call("waitforblockheight", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `waitfornewblock` for version v17
///
/// Waits for any new block and returns useful info about it.
/// Returns the current block on timeout or exit.
/// Make sure to use no RPC timeout (bitcoin-cli -rpcclienttimeout=0)
#[macro_export]
macro_rules! impl_client_v17__waitfornewblock {
    () => {
        impl Client {
            pub fn waitfornewblock(&self, timeout: Option<i64>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(timeout) = timeout {
                    params.push(into_json(timeout)?);
                }
                self.call("waitfornewblock", &params)
            }
        }
    };
}

