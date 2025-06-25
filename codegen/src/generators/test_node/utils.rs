//! Utility functions for test node generation

use std::fmt::Write as _;

/// Capitalizes the first character of a string and converts snake_case/kebab-case to PascalCase.
///
/// This function takes a string and converts it to PascalCase by:
/// - Capitalizing the first character
/// - Converting underscores and hyphens to spaces
/// - Capitalizing the first letter of each word
/// - Removing spaces and converting to uppercase
pub fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut up = true;
    for ch in s.chars() {
        if ch == '_' || ch == '-' {
            up = true;
        } else if up {
            out.push(ch.to_ascii_uppercase());
            up = false;
        } else {
            out.push(ch);
        }
    }
    out
}

/// Generates a module file for the test node client.
///
/// This function creates a module file that contains the client structs and implementations
/// for the test node client. The generated file is wrapped in `#[cfg(test)]` to ensure it's
/// only available during testing, making it suitable for integration tests with Bitcoin nodes.
pub fn generate_mod_rs() -> String {
    let mut code = String::new();
    writeln!(
        code,
        "//! Test node module for Bitcoin RPC testing
#[cfg(test)]
pub mod params;
pub mod response;
pub mod wallet;
pub mod node;
pub mod client;

// re-export common clients
pub use client::BitcoinTestClient;
pub use wallet::BitcoinWalletClient;
pub use node::BitcoinNodeClient;
"
    )
    .unwrap();
    code
}
