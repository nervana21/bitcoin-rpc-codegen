/// 
macro_rules! impl_client_v29__abortrescan {
    () => {
        pub fn abortrescan(
            &self,
            
        ) -> RpcResult<AbortrescanResponse> {
            self.call("abortrescan", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__clearbanned {
    () => {
        pub fn clearbanned(
            &self,
            
        ) -> RpcResult<ClearbannedResponse> {
            self.call("clearbanned", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getaddrmaninfo {
    () => {
        pub fn getaddrmaninfo(
            &self,
            
        ) -> RpcResult<GetaddrmaninfoResponse> {
            self.call("getaddrmaninfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getbalances {
    () => {
        pub fn getbalances(
            &self,
            
        ) -> RpcResult<GetbalancesResponse> {
            self.call("getbalances", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getbestblockhash {
    () => {
        pub fn getbestblockhash(
            &self,
            
        ) -> RpcResult<GetbestblockhashResponse> {
            self.call("getbestblockhash", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getblockchaininfo {
    () => {
        pub fn getblockchaininfo(
            &self,
            
        ) -> RpcResult<GetblockchaininfoResponse> {
            self.call("getblockchaininfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getblockcount {
    () => {
        pub fn getblockcount(
            &self,
            
        ) -> RpcResult<GetblockcountResponse> {
            self.call("getblockcount", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getchainstates {
    () => {
        pub fn getchainstates(
            &self,
            
        ) -> RpcResult<GetchainstatesResponse> {
            self.call("getchainstates", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getchaintips {
    () => {
        pub fn getchaintips(
            &self,
            
        ) -> RpcResult<GetchaintipsResponse> {
            self.call("getchaintips", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getconnectioncount {
    () => {
        pub fn getconnectioncount(
            &self,
            
        ) -> RpcResult<GetconnectioncountResponse> {
            self.call("getconnectioncount", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getdifficulty {
    () => {
        pub fn getdifficulty(
            &self,
            
        ) -> RpcResult<GetdifficultyResponse> {
            self.call("getdifficulty", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getmempoolinfo {
    () => {
        pub fn getmempoolinfo(
            &self,
            
        ) -> RpcResult<GetmempoolinfoResponse> {
            self.call("getmempoolinfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getmininginfo {
    () => {
        pub fn getmininginfo(
            &self,
            
        ) -> RpcResult<GetmininginfoResponse> {
            self.call("getmininginfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getnettotals {
    () => {
        pub fn getnettotals(
            &self,
            
        ) -> RpcResult<GetnettotalsResponse> {
            self.call("getnettotals", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getnetworkinfo {
    () => {
        pub fn getnetworkinfo(
            &self,
            
        ) -> RpcResult<GetnetworkinfoResponse> {
            self.call("getnetworkinfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getpeerinfo {
    () => {
        pub fn getpeerinfo(
            &self,
            
        ) -> RpcResult<GetpeerinfoResponse> {
            self.call("getpeerinfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getprioritisedtransactions {
    () => {
        pub fn getprioritisedtransactions(
            &self,
            
        ) -> RpcResult<GetprioritisedtransactionsResponse> {
            self.call("getprioritisedtransactions", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getrpcinfo {
    () => {
        pub fn getrpcinfo(
            &self,
            
        ) -> RpcResult<GetrpcinfoResponse> {
            self.call("getrpcinfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getunconfirmedbalance {
    () => {
        pub fn getunconfirmedbalance(
            &self,
            
        ) -> RpcResult<GetunconfirmedbalanceResponse> {
            self.call("getunconfirmedbalance", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getwalletinfo {
    () => {
        pub fn getwalletinfo(
            &self,
            
        ) -> RpcResult<GetwalletinfoResponse> {
            self.call("getwalletinfo", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__getzmqnotifications {
    () => {
        pub fn getzmqnotifications(
            &self,
            
        ) -> RpcResult<GetzmqnotificationsResponse> {
            self.call("getzmqnotifications", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__listaddressgroupings {
    () => {
        pub fn listaddressgroupings(
            &self,
            
        ) -> RpcResult<ListaddressgroupingsResponse> {
            self.call("listaddressgroupings", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__listbanned {
    () => {
        pub fn listbanned(
            &self,
            
        ) -> RpcResult<ListbannedResponse> {
            self.call("listbanned", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__listlockunspent {
    () => {
        pub fn listlockunspent(
            &self,
            
        ) -> RpcResult<ListlockunspentResponse> {
            self.call("listlockunspent", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__listwalletdir {
    () => {
        pub fn listwalletdir(
            &self,
            
        ) -> RpcResult<ListwalletdirResponse> {
            self.call("listwalletdir", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__listwallets {
    () => {
        pub fn listwallets(
            &self,
            
        ) -> RpcResult<ListwalletsResponse> {
            self.call("listwallets", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__ping {
    () => {
        pub fn ping(
            &self,
            
        ) -> RpcResult<PingResponse> {
            self.call("ping", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__savemempool {
    () => {
        pub fn savemempool(
            &self,
            
        ) -> RpcResult<SavemempoolResponse> {
            self.call("savemempool", json!([]))
        }
    };
}


/// 
macro_rules! impl_client_v29__stop {
    () => {
        pub fn stop(
            &self,
            
        ) -> RpcResult<StopResponse> {
            self.call("stop", json!([]))
        }
    };
}


