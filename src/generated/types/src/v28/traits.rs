use crate::traits::*;

impl BlockchainResponse for v28 {
    type DumptxoutsetResponse = DumptxoutsetResponse;
    type GetbestblockhashResponse = GetbestblockhashResponse;
    type GetblockResponse = GetblockResponse;
    type GetblockchaininfoResponse = GetblockchaininfoResponse;
    type GetblockcountResponse = GetblockcountResponse;
    type GetblockfilterResponse = GetblockfilterResponse;
    type GetblockfrompeerResponse = GetblockfrompeerResponse;
    type GetblockhashResponse = GetblockhashResponse;
    type GetblockheaderResponse = GetblockheaderResponse;
    type GetblockstatsResponse = GetblockstatsResponse;
    type GetchainstatesResponse = GetchainstatesResponse;
    type GetchaintipsResponse = GetchaintipsResponse;
    type GetchaintxstatsResponse = GetchaintxstatsResponse;
    type GetdeploymentinfoResponse = GetdeploymentinfoResponse;
    type GetdifficultyResponse = GetdifficultyResponse;
    type GetmempoolancestorsResponse = GetmempoolancestorsResponse;
    type GetmempooldescendantsResponse = GetmempooldescendantsResponse;
    type GetmempoolentryResponse = GetmempoolentryResponse;
    type GetmempoolinfoResponse = GetmempoolinfoResponse;
    type GetrawmempoolResponse = GetrawmempoolResponse;
    type GettxoutResponse = GettxoutResponse;
    type GettxoutproofResponse = GettxoutproofResponse;
    type GettxoutsetinfoResponse = GettxoutsetinfoResponse;
    type GettxspendingprevoutResponse = GettxspendingprevoutResponse;
    type ImportmempoolResponse = ImportmempoolResponse;
    type LoadtxoutsetResponse = LoadtxoutsetResponse;
    type PreciousblockResponse = PreciousblockResponse;
    type PruneblockchainResponse = PruneblockchainResponse;
    type SavemempoolResponse = SavemempoolResponse;
    type ScanblocksResponse = ScanblocksResponse;
    type ScantxoutsetResponse = ScantxoutsetResponse;
    type VerifychainResponse = VerifychainResponse;
    type VerifytxoutproofResponse = VerifytxoutproofResponse;
}

impl MiningResponse for v28 {
    type GetblocktemplateResponse = GetblocktemplateResponse;
    type GetmininginfoResponse = GetmininginfoResponse;
    type GetnetworkhashpsResponse = GetnetworkhashpsResponse;
    type GetprioritisedtransactionsResponse = GetprioritisedtransactionsResponse;
    type PrioritisetransactionResponse = PrioritisetransactionResponse;
    type SubmitblockResponse = SubmitblockResponse;
    type SubmitheaderResponse = SubmitheaderResponse;
}

impl NetworkResponse for v28 {
    type AddnodeResponse = AddnodeResponse;
    type ClearbannedResponse = ClearbannedResponse;
    type DisconnectnodeResponse = DisconnectnodeResponse;
    type GetaddednodeinfoResponse = GetaddednodeinfoResponse;
    type GetaddrmaninfoResponse = GetaddrmaninfoResponse;
    type GetconnectioncountResponse = GetconnectioncountResponse;
    type GetnettotalsResponse = GetnettotalsResponse;
    type GetnetworkinfoResponse = GetnetworkinfoResponse;
    type GetnodeaddressesResponse = GetnodeaddressesResponse;
    type GetpeerinfoResponse = GetpeerinfoResponse;
    type ListbannedResponse = ListbannedResponse;
    type PingResponse = PingResponse;
    type SetbanResponse = SetbanResponse;
    type SetnetworkactiveResponse = SetnetworkactiveResponse;
}

impl UtilResponse for v28 {
    type CreatemultisigResponse = CreatemultisigResponse;
    type DeriveaddressesResponse = DeriveaddressesResponse;
    type EstimatesmartfeeResponse = EstimatesmartfeeResponse;
    type GetdescriptorinfoResponse = GetdescriptorinfoResponse;
    type GetindexinfoResponse = GetindexinfoResponse;
    type SignmessagewithprivkeyResponse = SignmessagewithprivkeyResponse;
    type ValidateaddressResponse = ValidateaddressResponse;
    type VerifymessageResponse = VerifymessageResponse;
}

impl WalletResponse for v28 {
    type AbandontransactionResponse = AbandontransactionResponse;
    type AbortrescanResponse = AbortrescanResponse;
    type AddmultisigaddressResponse = AddmultisigaddressResponse;
    type BackupwalletResponse = BackupwalletResponse;
    type BumpfeeResponse = BumpfeeResponse;
    type CreatewalletResponse = CreatewalletResponse;
    type CreatewalletdescriptorResponse = CreatewalletdescriptorResponse;
    type DumpprivkeyResponse = DumpprivkeyResponse;
    type DumpwalletResponse = DumpwalletResponse;
    type EncryptwalletResponse = EncryptwalletResponse;
    type GetaddressesbylabelResponse = GetaddressesbylabelResponse;
    type GetaddressinfoResponse = GetaddressinfoResponse;
    type GetbalanceResponse = GetbalanceResponse;
    type GetbalancesResponse = GetbalancesResponse;
    type GethdkeysResponse = GethdkeysResponse;
    type GetnewaddressResponse = GetnewaddressResponse;
    type GetrawchangeaddressResponse = GetrawchangeaddressResponse;
    type GetreceivedbyaddressResponse = GetreceivedbyaddressResponse;
    type GetreceivedbylabelResponse = GetreceivedbylabelResponse;
    type GettransactionResponse = GettransactionResponse;
    type GetunconfirmedbalanceResponse = GetunconfirmedbalanceResponse;
    type GetwalletinfoResponse = GetwalletinfoResponse;
    type ImportaddressResponse = ImportaddressResponse;
    type ImportdescriptorsResponse = ImportdescriptorsResponse;
    type ImportmultiResponse = ImportmultiResponse;
    type ImportprivkeyResponse = ImportprivkeyResponse;
    type ImportprunedfundsResponse = ImportprunedfundsResponse;
    type ImportpubkeyResponse = ImportpubkeyResponse;
    type ImportwalletResponse = ImportwalletResponse;
    type KeypoolrefillResponse = KeypoolrefillResponse;
    type ListaddressgroupingsResponse = ListaddressgroupingsResponse;
    type ListdescriptorsResponse = ListdescriptorsResponse;
    type ListlabelsResponse = ListlabelsResponse;
    type ListlockunspentResponse = ListlockunspentResponse;
    type ListreceivedbyaddressResponse = ListreceivedbyaddressResponse;
    type ListreceivedbylabelResponse = ListreceivedbylabelResponse;
    type ListsinceblockResponse = ListsinceblockResponse;
    type ListtransactionsResponse = ListtransactionsResponse;
    type ListunspentResponse = ListunspentResponse;
    type ListwalletdirResponse = ListwalletdirResponse;
    type ListwalletsResponse = ListwalletsResponse;
    type LoadwalletResponse = LoadwalletResponse;
    type LockunspentResponse = LockunspentResponse;
    type MigratewalletResponse = MigratewalletResponse;
    type NewkeypoolResponse = NewkeypoolResponse;
    type PsbtbumpfeeResponse = PsbtbumpfeeResponse;
    type RemoveprunedfundsResponse = RemoveprunedfundsResponse;
    type RescanblockchainResponse = RescanblockchainResponse;
    type RestorewalletResponse = RestorewalletResponse;
    type SendResponse = SendResponse;
    type SendallResponse = SendallResponse;
    type SendmanyResponse = SendmanyResponse;
    type SendtoaddressResponse = SendtoaddressResponse;
    type SethdseedResponse = SethdseedResponse;
    type SetlabelResponse = SetlabelResponse;
    type SettxfeeResponse = SettxfeeResponse;
    type SetwalletflagResponse = SetwalletflagResponse;
    type SignmessageResponse = SignmessageResponse;
    type SignrawtransactionwithwalletResponse = SignrawtransactionwithwalletResponse;
    type SimulaterawtransactionResponse = SimulaterawtransactionResponse;
    type UnloadwalletResponse = UnloadwalletResponse;
    type UpgradewalletResponse = UpgradewalletResponse;
    type WalletcreatefundedpsbtResponse = WalletcreatefundedpsbtResponse;
    type WalletdisplayaddressResponse = WalletdisplayaddressResponse;
    type WalletlockResponse = WalletlockResponse;
    type WalletpassphraseResponse = WalletpassphraseResponse;
    type WalletpassphrasechangeResponse = WalletpassphrasechangeResponse;
    type WalletprocesspsbtResponse = WalletprocesspsbtResponse;
}

impl HiddenResponse for v28 {
    type AddconnectionResponse = AddconnectionResponse;
    type AddpeeraddressResponse = AddpeeraddressResponse;
    type EchoResponse = EchoResponse;
    type EchoipcResponse = EchoipcResponse;
    type EchojsonResponse = EchojsonResponse;
    type EstimaterawfeeResponse = EstimaterawfeeResponse;
    type GenerateResponse = GenerateResponse;
    type GenerateblockResponse = GenerateblockResponse;
    type GeneratetoaddressResponse = GeneratetoaddressResponse;
    type GeneratetodescriptorResponse = GeneratetodescriptorResponse;
    type GetorphantxsResponse = GetorphantxsResponse;
    type GetrawaddrmanResponse = GetrawaddrmanResponse;
    type InvalidateblockResponse = InvalidateblockResponse;
    type MockschedulerResponse = MockschedulerResponse;
    type ReconsiderblockResponse = ReconsiderblockResponse;
    type SendmsgtopeerResponse = SendmsgtopeerResponse;
    type SetmocktimeResponse = SetmocktimeResponse;
    type SyncwithvalidationinterfacequeueResponse = SyncwithvalidationinterfacequeueResponse;
    type WaitforblockResponse = WaitforblockResponse;
    type WaitforblockheightResponse = WaitforblockheightResponse;
    type WaitfornewblockResponse = WaitfornewblockResponse;
}

impl RawtransactionsResponse for v28 {
    type AnalyzepsbtResponse = AnalyzepsbtResponse;
    type CombinepsbtResponse = CombinepsbtResponse;
    type CombinerawtransactionResponse = CombinerawtransactionResponse;
    type ConverttopsbtResponse = ConverttopsbtResponse;
    type CreatepsbtResponse = CreatepsbtResponse;
    type CreaterawtransactionResponse = CreaterawtransactionResponse;
    type DecodepsbtResponse = DecodepsbtResponse;
    type DecoderawtransactionResponse = DecoderawtransactionResponse;
    type DecodescriptResponse = DecodescriptResponse;
    type DescriptorprocesspsbtResponse = DescriptorprocesspsbtResponse;
    type FinalizepsbtResponse = FinalizepsbtResponse;
    type FundrawtransactionResponse = FundrawtransactionResponse;
    type GetrawtransactionResponse = GetrawtransactionResponse;
    type JoinpsbtsResponse = JoinpsbtsResponse;
    type SendrawtransactionResponse = SendrawtransactionResponse;
    type SignrawtransactionwithkeyResponse = SignrawtransactionwithkeyResponse;
    type SubmitpackageResponse = SubmitpackageResponse;
    type TestmempoolacceptResponse = TestmempoolacceptResponse;
    type UtxoupdatepsbtResponse = UtxoupdatepsbtResponse;
}

impl ControlResponse for v28 {
    type ApiResponse = ApiResponse;
    type GetmemoryinfoResponse = GetmemoryinfoResponse;
    type GetrpcinfoResponse = GetrpcinfoResponse;
    type HelpResponse = HelpResponse;
    type LoggingResponse = LoggingResponse;
    type StopResponse = StopResponse;
    type UptimeResponse = UptimeResponse;
}

impl SignerResponse for v28 {
    type EnumeratesignersResponse = EnumeratesignersResponse;
}

