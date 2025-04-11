// SPDX-License-Identifier: CC0-1.0

//! Common traits for Bitcoin RPC operations across different versions.
//! These traits provide a unified interface for working with different Bitcoin Core versions.

use serde::{Deserialize, Serialize};

/// Common trait for blockchain-related RPC responses
pub trait BlockchainResponse {
    type GetBlockResponse: Deserialize<'static> + Serialize;
    type GetBlockHashResponse: Deserialize<'static> + Serialize;
    type GetBlockchainInfoResponse: Deserialize<'static> + Serialize;
}

/// Common trait for wallet-related RPC responses
pub trait WalletResponse {
    type GetBalanceResponse: Deserialize<'static> + Serialize;
    type GetTransactionResponse: Deserialize<'static> + Serialize;
}

/// Common trait for mining-related RPC responses
pub trait MiningResponse {
    type GetBlockTemplateResponse: Deserialize<'static> + Serialize;
    type GetMiningInfoResponse: Deserialize<'static> + Serialize;
}

/// Common trait for network-related RPC responses
pub trait NetworkResponse {
    type GetNetworkInfoResponse: Deserialize<'static> + Serialize;
    type GetPeerInfoResponse: Deserialize<'static> + Serialize;
}

/// Common trait for control-related RPC responses
pub trait ControlResponse {
    type GetMemoryInfoResponse: Deserialize<'static> + Serialize;
    type HelpResponse: Deserialize<'static> + Serialize;
    type UptimeResponse: Deserialize<'static> + Serialize;
}

/// Common trait for raw transaction-related RPC responses
pub trait RawTransactionResponse {
    type GetRawTransactionResponse: Deserialize<'static> + Serialize;
    type DecodeRawTransactionResponse: Deserialize<'static> + Serialize;
    type CreateRawTransactionResponse: Deserialize<'static> + Serialize;
}

/// Common trait for utility-related RPC responses
pub trait UtilResponse {
    type ValidateAddressResponse: Deserialize<'static> + Serialize;
    type CreateMultiSigResponse: Deserialize<'static> + Serialize;
    type VerifyMessageResponse: Deserialize<'static> + Serialize;
}

/// Common trait for signer-related RPC responses
pub trait SignerResponse {
    type SignRawTransactionResponse: Deserialize<'static> + Serialize;
    type SignRawTransactionWithKeyResponse: Deserialize<'static> + Serialize;
}
