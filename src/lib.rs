// src/lib.rs

// Re-export external types
pub use external_bitcoin::Amount;
pub use external_bitcoin::absolute::Time;
pub use external_bitcoin::consensus::serde::Hex;

// Expose our core modules
pub mod bitcoin;
#[path = "local_serde_json.rs"]
pub mod serde_json;

// Bring your new client into the top‑level API
pub mod client;
pub use client::BitcoinRpcClient;
