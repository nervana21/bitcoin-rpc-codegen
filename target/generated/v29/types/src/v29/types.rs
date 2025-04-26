use serde::{Deserialize, Serialize};

/// Response for the AbortrescanResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AbortrescanResponse {
    pub result: bool,
}



/// Response for the GetaddrmaninfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetaddrmaninfoResponse {
    pub all_networks: serde_json::Value,
    pub cjdns: serde_json::Value,
    pub i2p: serde_json::Value,
    pub ipv4: serde_json::Value,
    pub ipv6: serde_json::Value,
    pub onion: serde_json::Value,

}



/// Response for the GetbalancesResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbalancesResponse {
    pub lastprocessedblock: serde_json::Value,
    pub mine: serde_json::Value,

}



/// Response for the GetbestblockhashResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetbestblockhashResponse {
    pub result: String,
}



/// Response for the GetblockchaininfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockchaininfoResponse {
    pub bestblockhash: serde_json::Value,
    pub bits: serde_json::Value,
    pub blocks: serde_json::Value,
    pub chain: serde_json::Value,
    pub chainwork: serde_json::Value,
    pub difficulty: serde_json::Value,
    pub headers: serde_json::Value,
    pub initialblockdownload: serde_json::Value,
    pub mediantime: serde_json::Value,
    pub pruned: serde_json::Value,
    pub size_on_disk: serde_json::Value,
    pub target: serde_json::Value,
    pub time: serde_json::Value,
    pub verificationprogress: serde_json::Value,
    pub warnings: serde_json::Value,

}



/// Response for the GetblockcountResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetblockcountResponse {
    pub result: f64,
}



/// Response for the GetchainstatesResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchainstatesResponse {
    pub chainstates: serde_json::Value,
    pub headers: serde_json::Value,

}



/// Response for the GetchaintipsResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetchaintipsResponse {
    pub result: serde_json::Value,

}



/// Response for the GetconnectioncountResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetconnectioncountResponse {
    pub result: f64,
}



/// Response for the GetdifficultyResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetdifficultyResponse {
    pub result: f64,
}



/// Response for the GetmempoolinfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmempoolinfoResponse {
    pub bytes: serde_json::Value,
    pub fullrbf: serde_json::Value,
    pub incrementalrelayfee: serde_json::Value,
    pub loaded: serde_json::Value,
    pub maxmempool: serde_json::Value,
    pub mempoolminfee: serde_json::Value,
    pub minrelaytxfee: serde_json::Value,
    pub size: serde_json::Value,
    pub total_fee: serde_json::Value,
    pub unbroadcastcount: serde_json::Value,
    pub usage: serde_json::Value,

}



/// Response for the GetmininginfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetmininginfoResponse {
    pub bits: serde_json::Value,
    pub blocks: serde_json::Value,
    pub chain: serde_json::Value,
    pub difficulty: serde_json::Value,
    pub networkhashps: serde_json::Value,
    pub next: serde_json::Value,
    pub pooledtx: serde_json::Value,
    pub target: serde_json::Value,
    pub warnings: serde_json::Value,

}



/// Response for the GetnettotalsResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnettotalsResponse {
    pub timemillis: serde_json::Value,
    pub totalbytesrecv: serde_json::Value,
    pub totalbytessent: serde_json::Value,
    pub uploadtarget: serde_json::Value,

}



/// Response for the GetnetworkinfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetnetworkinfoResponse {
    pub connections: serde_json::Value,
    pub connections_in: serde_json::Value,
    pub connections_out: serde_json::Value,
    pub incrementalfee: serde_json::Value,
    pub localaddresses: serde_json::Value,
    pub localrelay: serde_json::Value,
    pub localservices: serde_json::Value,
    pub localservicesnames: serde_json::Value,
    pub networkactive: serde_json::Value,
    pub networks: serde_json::Value,
    pub protocolversion: serde_json::Value,
    pub relayfee: serde_json::Value,
    pub subversion: serde_json::Value,
    pub timeoffset: serde_json::Value,
    pub version: serde_json::Value,
    pub warnings: serde_json::Value,

}



/// Response for the GetpeerinfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetpeerinfoResponse {
    pub result: String,
}



/// Response for the GetprioritisedtransactionsResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetprioritisedtransactionsResponse {
    pub result: serde_json::Value,
}



/// Response for the GetrpcinfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetrpcinfoResponse {
    pub active_commands: serde_json::Value,
    pub logpath: serde_json::Value,

}



/// Response for the GetunconfirmedbalanceResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetunconfirmedbalanceResponse {
    pub result: f64,
}



/// Response for the GetwalletinfoResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetwalletinfoResponse {
    pub avoid_reuse: serde_json::Value,
    pub balance: serde_json::Value,
    pub birthtime: serde_json::Value,
    pub blank: serde_json::Value,
    pub descriptors: serde_json::Value,
    pub external_signer: serde_json::Value,
    pub format: serde_json::Value,
    pub immature_balance: serde_json::Value,
    pub keypoolsize: serde_json::Value,
    pub keypoolsize_hd_internal: serde_json::Value,
    pub lastprocessedblock: serde_json::Value,
    pub paytxfee: serde_json::Value,
    pub private_keys_enabled: serde_json::Value,
    pub scanning: serde_json::Value,
    pub txcount: serde_json::Value,
    pub unconfirmed_balance: serde_json::Value,
    pub walletname: serde_json::Value,
    pub walletversion: serde_json::Value,

}



/// Response for the GetzmqnotificationsResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetzmqnotificationsResponse {
    pub result: String,
}



/// Response for the ListaddressgroupingsResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListaddressgroupingsResponse {
    pub result: String,
}



/// Response for the ListbannedResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListbannedResponse {
    pub result: String,
}



/// Response for the ListlockunspentResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListlockunspentResponse {
    pub result: String,
}



/// Response for the ListwalletdirResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletdirResponse {
    pub wallets: serde_json::Value,

}



/// Response for the ListwalletsResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ListwalletsResponse {
    pub result: String,

}



/// Response for the SavemempoolResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SavemempoolResponse {
    pub filename: serde_json::Value,

}



/// Response for the StopResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct StopResponse {
    pub result: String,
}



/// Response for the UptimeResponse RPC call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UptimeResponse {
    pub result: f64,
}



