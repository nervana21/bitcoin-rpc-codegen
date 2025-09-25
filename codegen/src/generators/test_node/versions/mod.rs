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
/// - "v28", "V28", "v28.0", "v28.1" -> V28Helpers
/// - "v29", "V29", "v29.0", "v29.1" -> V29Helpers
///
/// # Arguments
/// * `version` - The version string (e.g., "V29", "v29", "29", "v29.1")
///
/// # Returns
/// * `Box<dyn VersionedClientHelpers>` - The appropriate helpers for the version
///
/// # Panics
/// * Panics if the version is not supported
pub fn get_helpers_for_version(version: &str) -> Box<dyn VersionedClientHelpers> {
    let version_clean = version.trim_start_matches('v').trim_start_matches('V');
    let major_version = version_clean.split('.').next().unwrap_or(version_clean);

    match major_version {
        "28" => Box::new(V28Helpers),
        "29" => Box::new(V29Helpers),
        _ => panic!("Unsupported version: {version}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_dispatch() {
        // Test that v29.1 maps to V29Helpers (the main new functionality)
        let _helpers = get_helpers_for_version("v29.1");

        // Test other v29 formats
        let _helpers = get_helpers_for_version("v29");
        let _helpers = get_helpers_for_version("V29");
        let _helpers = get_helpers_for_version("29");
        let _helpers = get_helpers_for_version("v29.0");

        // Test v28 formats
        let _helpers = get_helpers_for_version("v28");
        let _helpers = get_helpers_for_version("V28");
        let _helpers = get_helpers_for_version("28");
        let _helpers = get_helpers_for_version("v28.1");
    }

    #[test]
    #[should_panic(expected = "Unsupported version: invalid")]
    fn test_unsupported_version_panics() { get_helpers_for_version("invalid"); }

    #[test]
    #[should_panic(expected = "Unsupported version: v30.1")]
    fn test_unsupported_major_version_panics() { get_helpers_for_version("v30.1"); }
}
