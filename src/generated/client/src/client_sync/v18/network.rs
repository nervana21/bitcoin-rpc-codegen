/// Implements Bitcoin Core JSON-RPC API method `addnode` for version v18
///
/// Attempts to add or remove a node from the addnode list.
/// Or try a connection to a node once.
/// Nodes added using addnode (or -connect) are protected from DoS disconnection and are not required to be
/// full nodes/support SegWit as other outbound peers are (though such peers will not be synced from).
/// Addnode connections are limited to 8 at a time and are counted separately from the -maxconnections limit.
#[macro_export]
macro_rules! impl_client_v18__addnode {
    () => {
        impl Client {
            pub fn addnode(&self, node: String, command: String, v2transport: Option<bool>) -> Result<()> {
                let mut params = vec![into_json(node)?, into_json(command)?];
                if let Some(v2transport) = v2transport {
                    params.push(into_json(v2transport)?);
                }
                self.call("addnode", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `clearbanned` for version v18
///
/// Clear all banned IPs.
#[macro_export]
macro_rules! impl_client_v18__clearbanned {
    () => {
        impl Client {
            pub fn clearbanned(&self) -> Result<()> {
                self.call("clearbanned", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `disconnectnode` for version v18
///
/// Immediately disconnects from the specified peer node.
/// Strictly one out of 'address' and 'nodeid' can be provided to identify the node.
/// To disconnect by nodeid, either set 'address' to the empty string, or call using the named 'nodeid' argument only.
#[macro_export]
macro_rules! impl_client_v18__disconnectnode {
    () => {
        impl Client {
            pub fn disconnectnode(&self, address: Option<String>, nodeid: Option<i64>) -> Result<()> {
                let mut params = vec![];
                if let Some(address) = address {
                    params.push(into_json(address)?);
                }
                if let Some(nodeid) = nodeid {
                    params.push(into_json(nodeid)?);
                }
                self.call("disconnectnode", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddednodeinfo` for version v18
///
/// Returns information about the given added node, or all added nodes
/// (note that onetry addnodes are not listed here)
#[macro_export]
macro_rules! impl_client_v18__getaddednodeinfo {
    () => {
        impl Client {
            pub fn getaddednodeinfo(&self, node: Option<String>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![];
                if let Some(node) = node {
                    params.push(into_json(node)?);
                }
                self.call("getaddednodeinfo", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getaddrmaninfo` for version v18
///
/// Provides information about the node's address manager by returning the number of addresses in the `new` and `tried` tables and their sum for all networks.
#[macro_export]
macro_rules! impl_client_v18__getaddrmaninfo {
    () => {
        impl Client {
            pub fn getaddrmaninfo(&self) -> Result<object_dynamic> {
                self.call("getaddrmaninfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getconnectioncount` for version v18
///
/// Returns the number of connections to other nodes.
#[macro_export]
macro_rules! impl_client_v18__getconnectioncount {
    () => {
        impl Client {
            pub fn getconnectioncount(&self) -> Result<number> {
                self.call("getconnectioncount", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnettotals` for version v18
///
/// Returns information about network traffic, including bytes in, bytes out,
/// and current system time.
#[macro_export]
macro_rules! impl_client_v18__getnettotals {
    () => {
        impl Client {
            pub fn getnettotals(&self) -> Result<serde_json::Value> {
                self.call("getnettotals", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnetworkinfo` for version v18
///
/// Returns an object containing various state info regarding P2P networking.
#[macro_export]
macro_rules! impl_client_v18__getnetworkinfo {
    () => {
        impl Client {
            pub fn getnetworkinfo(&self) -> Result<serde_json::Value> {
                self.call("getnetworkinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getnodeaddresses` for version v18
///
/// Return known addresses, after filtering for quality and recency.
/// These can potentially be used to find new peers in the network.
/// The total number of addresses known to the node may be higher.
#[macro_export]
macro_rules! impl_client_v18__getnodeaddresses {
    () => {
        impl Client {
            pub fn getnodeaddresses(&self, count: Option<i64>, network: Option<String>) -> Result<Vec<serde_json::Value>> {
                let mut params = vec![];
                if let Some(count) = count {
                    params.push(into_json(count)?);
                }
                if let Some(network) = network {
                    params.push(into_json(network)?);
                }
                self.call("getnodeaddresses", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `getpeerinfo` for version v18
///
/// Returns data about each connected network peer as a json array of objects.
#[macro_export]
macro_rules! impl_client_v18__getpeerinfo {
    () => {
        impl Client {
            pub fn getpeerinfo(&self) -> Result<Vec<serde_json::Value>> {
                self.call("getpeerinfo", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `listbanned` for version v18
///
/// List all manually banned IPs/Subnets.
#[macro_export]
macro_rules! impl_client_v18__listbanned {
    () => {
        impl Client {
            pub fn listbanned(&self) -> Result<Vec<serde_json::Value>> {
                self.call("listbanned", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `ping` for version v18
///
/// Requests that a ping be sent to all other nodes, to measure ping time.
/// Results provided in getpeerinfo, pingtime and pingwait fields are decimal seconds.
/// Ping command is handled in queue with all other commands, so it measures processing backlog, not just network ping.
#[macro_export]
macro_rules! impl_client_v18__ping {
    () => {
        impl Client {
            pub fn ping(&self) -> Result<()> {
                self.call("ping", &[])
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setban` for version v18
///
/// Attempts to add or remove an IP/Subnet from the banned list.
#[macro_export]
macro_rules! impl_client_v18__setban {
    () => {
        impl Client {
            pub fn setban(&self, subnet: String, command: String, bantime: Option<i64>, absolute: Option<bool>) -> Result<()> {
                let mut params = vec![into_json(subnet)?, into_json(command)?];
                if let Some(bantime) = bantime {
                    params.push(into_json(bantime)?);
                }
                if let Some(absolute) = absolute {
                    params.push(into_json(absolute)?);
                }
                self.call("setban", &params)
            }
        }
    };
}

/// Implements Bitcoin Core JSON-RPC API method `setnetworkactive` for version v18
///
/// Disable/enable all p2p network activity.
#[macro_export]
macro_rules! impl_client_v18__setnetworkactive {
    () => {
        impl Client {
            pub fn setnetworkactive(&self, state: bool) -> Result<bool> {
                self.call("setnetworkactive", &[into_json(state)?])
            }
        }
    };
}

