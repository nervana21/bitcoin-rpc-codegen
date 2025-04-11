/// Implements Bitcoin Core JSON-RPC API method `api` for version v18
///
/// Return JSON description of RPC API.
#[macro_export]
macro_rules! impl_client_v18__api {
    () => {
        impl Client {
            pub fn api(&self) -> Result<serde_json::Value> {
                self.call("api", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getmemoryinfo` for version v18
///
/// Returns an object containing information about memory usage.
#[macro_export]
macro_rules! impl_client_v18__getmemoryinfo {
    () => {
        impl Client {
            pub fn getmemoryinfo(&self, mode: Option<String>) -> Result<serde_json::Value> {
                let mut params = vec![];
                if let Some(mode) = mode {
                    params.push(into_json(mode)?);
                }
                self.call("getmemoryinfo", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getrpcinfo` for version v18
///
/// Returns details of the RPC server.
#[macro_export]
macro_rules! impl_client_v18__getrpcinfo {
    () => {
        impl Client {
            pub fn getrpcinfo(&self) -> Result<serde_json::Value> {
                self.call("getrpcinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `help` for version v18
///
/// List all commands, or get help for a specified command.
#[macro_export]
macro_rules! impl_client_v18__help {
    () => {
        impl Client {
            pub fn help(&self, command: Option<String>) -> Result<String> {
                let mut params = vec![];
                if let Some(command) = command {
                    params.push(into_json(command)?);
                }
                self.call("help", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `logging` for version v18
///
/// Gets and sets the logging configuration.
/// When called without an argument, returns the list of categories with status that are currently being debug logged or not.
/// When called with arguments, adds or removes categories from debug logging and return the lists above.
/// The arguments are evaluated in order "include", "exclude".
/// If an item is both included and excluded, it will thus end up being excluded.
/// The valid logging categories are: addrman, bench, blockstorage, cmpctblock, coindb, estimatefee, http, i2p, ipc, leveldb, libevent, mempool, mempoolrej, net, proxy, prune, qt, rand, reindex, rpc, scan, selectcoins, tor, txpackages, txreconciliation, validation, walletdb, zmq
/// In addition, the following are available as category names with special meanings:
/// - "all",  "1" : represent all logging categories.
#[macro_export]
macro_rules! impl_client_v18__logging {
    () => {
        impl Client {
            pub fn logging(&self, include: Option<Vec<String>>, exclude: Option<Vec<String>>) -> Result<object_dynamic> {
                let mut params = vec![];
                if let Some(include) = include {
                    params.push(into_json(include)?);
                }
                if let Some(exclude) = exclude {
                    params.push(into_json(exclude)?);
                }
                self.call("logging", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `stop` for version v18
///
/// Request a graceful shutdown of Bitcoin Core.
#[macro_export]
macro_rules! impl_client_v18__stop {
    () => {
        impl Client {
            pub fn stop(&self, wait: Option<i64>) -> Result<String> {
                let mut params = vec![];
                if let Some(wait) = wait {
                    params.push(into_json(wait)?);
                }
                self.call("stop", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `uptime` for version v18
///
/// Returns the total uptime of the server.
#[macro_export]
macro_rules! impl_client_v18__uptime {
    () => {
        impl Client {
            pub fn uptime(&self) -> Result<number> {
                self.call("uptime", &[])
            }
        }
    };
}

