// src/lib.rs

// Expose our common prelude.

pub mod bitcoin;
pub use bitcoin::*;

// Expose our patched serde_json module.
#[path = "local_serde_json.rs"]
pub mod serde_json;
