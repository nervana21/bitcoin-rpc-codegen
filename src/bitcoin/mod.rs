// src/bitcoin/mod.rs

// The generated code expects `bitcoin::amount::Amount`—we reexport it from external_bitcoin.
pub mod amount {
    pub use external_bitcoin::amount::Amount;
}

// The generated code expects a type called Hex (e.g. for hex-encoded values).
// For our proof-of-concept, alias Hex to String.
pub mod hex {
    pub type Hex = String;
}

// The generated code expects a type called Time.
// For our purposes, alias Time to u64.
pub mod time {
    pub type Time = u64;
}

// Re-export the types at the module level for easier access
pub use amount::Amount;
pub use hex::Hex;
pub use time::Time;
