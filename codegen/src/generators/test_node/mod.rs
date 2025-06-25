//! Code-gen: build a thin `TestNode` client with typed-parameter helpers.
//!
//! This module contains the modularized test node generator components,
//! split into logical units for better maintainability and testing.

use crate::wallet_methods::WALLET_METHODS;
use crate::CodeGenerator;
use rpc_api::ApiMethod;

pub mod emit_combined_client;
pub mod emit_helpers;
pub mod emit_params;
pub mod emit_results;
pub mod emit_subclient;
pub mod utils;

/// A code generator that creates a type-safe Rust client library for Bitcoin Core test environments.
///
/// This generator takes Bitcoin Core RPC API definitions and produces a complete Rust client library
/// that provides a high-level, type-safe interface for:
/// - Node lifecycle management (start/stop)
/// - Wallet management and operations
/// - Block mining and chain manipulation
/// - All Bitcoin Core RPC methods with proper typing
///
/// The generated client library serves as a test harness that bridges Bitcoin Core's RPC interface
/// with Rust's type system, making it easier to write reliable Bitcoin Core integration tests
/// without dealing with low-level RPC details.
///
/// The generator produces several key components:
/// - Type-safe parameter structs for RPC calls
/// - Type-safe result structs for RPC responses
/// - A high-level `BitcoinTestClient` with ergonomic helpers
/// - Separate node and wallet client interfaces
///
/// This abstraction layer enables developers to focus on test logic rather than RPC mechanics,
/// while maintaining type safety and proper error handling throughout the test suite.
pub struct TestNodeGenerator {
    version: String,
}

impl TestNodeGenerator {
    /// Creates a new `TestNodeGenerator` configured for a specific Bitcoin Core version.
    ///
    /// The `version` string determines which RPC methods and structures are used when generating
    /// type-safe test clients and associated modules. This allows test code to stay in sync with
    /// version-specific behavior in Bitcoin Core.
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }
}

impl CodeGenerator for TestNodeGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let params_code = emit_params::generate_params_code(methods);
        let result_code = emit_results::generate_result_code(methods);

        let wallet_methods: Vec<_> = methods
            .iter()
            .filter(|m| WALLET_METHODS.contains(&m.name.as_str()))
            .cloned()
            .collect();
        let node_methods: Vec<_> = methods
            .iter()
            .filter(|m| !WALLET_METHODS.contains(&m.name.as_str()))
            .cloned()
            .collect();

        let wallet_code = emit_subclient::generate_subclient(
            "BitcoinWalletClient",
            &wallet_methods,
            &self.version,
        )
        .unwrap();
        let node_code =
            emit_subclient::generate_subclient("BitcoinNodeClient", &node_methods, &self.version)
                .unwrap();
        let client_code = emit_combined_client::generate_combined_client(
            "BitcoinTestClient",
            methods,
            &self.version,
        )
        .unwrap();

        let mod_rs_code = utils::generate_mod_rs();

        vec![
            ("wallet.rs".to_string(), wallet_code),
            ("node.rs".to_string(), node_code),
            ("client.rs".to_string(), client_code),
            ("params.rs".to_string(), params_code),
            ("response.rs".to_string(), result_code),
            ("mod.rs".to_string(), mod_rs_code),
        ]
    }
}
