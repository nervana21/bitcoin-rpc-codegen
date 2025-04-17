// src/local_serde_json.rs

pub use ::serde_json::*;

// The generated code later refers to serde_json::Fees and serde_json::Block_info.
// Provide type aliases for them.
pub type Fees = Value;
#[allow(non_camel_case_types)]
pub type Block_info = Value;
