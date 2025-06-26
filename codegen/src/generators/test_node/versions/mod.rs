//! Version-specific Bitcoin Core client helper implementations.
//!
//! This module contains version-specific implementations of Bitcoin Core
//! client helper functions for different API versions.

/// Bitcoin Core v28 API helper implementations
pub mod v28;
/// Bitcoin Core v29 API helper implementations
pub mod v29;

use v28::V28Helpers;
use v29::V29Helpers;

/// Trait for version-specific Bitcoin Core client helper functions.
///
/// This trait provides methods to generate version-specific code for common
/// Bitcoin operations like sending transactions, wallet configuration,
/// block mining, and chain reset functionality.
pub trait VersionedClientHelpers {
    /// Emits helper functions for sending Bitcoin to addresses.
    ///
    /// This function generates version-specific code for sending transactions
    /// to Bitcoin addresses, including proper parameter handling and error
    /// management for the specific Bitcoin Core version.
    ///
    /// # Arguments
    /// * `code` - A mutable reference to the String where the generated code will be appended
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Success or error result from the code generation
    fn emit_send_to_address_helpers(&self, code: &mut String) -> std::io::Result<()>;

    /// Emits a wallet options struct definition.
    ///
    /// This function generates version-specific wallet configuration structures
    /// that define the available options and parameters for wallet operations
    /// in the specific Bitcoin Core version.
    ///
    /// # Arguments
    /// * `code` - A mutable reference to the String where the generated code will be appended
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Success or error result from the code generation
    fn emit_wallet_options_struct(&self, code: &mut String) -> std::io::Result<()>;

    /// Emits helper functions for block mining operations.
    ///
    /// This function generates version-specific code for mining blocks,
    /// including functions to generate blocks, mine transactions, and handle
    /// mining-related operations for the specific Bitcoin Core version.
    ///
    /// # Arguments
    /// * `code` - A mutable reference to the String where the generated code will be appended
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Success or error result from the code generation
    fn emit_block_mining_helpers(&self, code: &mut String) -> std::io::Result<()>;

    /// Emits code for resetting the blockchain state.
    ///
    /// This function generates version-specific code for resetting the blockchain
    /// to a clean state, typically used in testing scenarios to ensure a known
    /// starting point for the specific Bitcoin Core version.
    ///
    /// # Arguments
    /// * `code` - A mutable reference to the String where the generated code will be appended
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Success or error result from the code generation
    fn emit_reset_chain(&self, code: &mut String) -> std::io::Result<()>;
}

/// Returns the appropriate version-specific helpers for the given version string.
///
/// This function accepts version strings in multiple formats:
/// - "v28", "V28" -> V28Helpers
/// - "v29", "V29" -> V29Helpers
///
/// # Arguments
/// * `version` - The version string (e.g., "V29", "v29", "29")
///
/// # Returns
/// * `Box<dyn VersionedClientHelpers>` - The appropriate helpers for the version
///
/// # Panics
/// * Panics if the version is not supported
pub fn get_helpers_for_version(version: &str) -> Box<dyn VersionedClientHelpers> {
    println!("[dispatch] using version: {}", version);
    match version.trim_start_matches('v').trim_start_matches('V') {
        "28" => Box::new(V28Helpers),
        "29" => Box::new(V29Helpers),
        _ => panic!("Unsupported version: {}", version),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_dispatch_works() {
        let _ = get_helpers_for_version("v28");
        let _ = get_helpers_for_version("28");
        let _ = get_helpers_for_version("V29");
        let _ = get_helpers_for_version("29");
    }

    #[test]
    #[should_panic(expected = "Unsupported version")]
    fn unsupported_version_panics() {
        let _ = get_helpers_for_version("v100");
    }
}
