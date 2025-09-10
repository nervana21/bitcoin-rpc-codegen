// node/src/test_config.rs

use bitcoin::Network;
use config::{BitcoinConfig, Config};
use std::env;

/// TestConfig represents the configuration needed to run a Bitcoin node in a test environment.
/// This struct is the single source of truth for test‑node settings: RPC port, username, and password.
/// Defaults are:
/// - `rpc_port = 0` (auto‑select a free port)
/// - `rpc_username = "rpcuser"`
/// - `rpc_password = "rpcpassword"`
/// - `network = Network::Regtest` (for isolation and testability)
///
/// To override any of these, simply modify fields on `TestConfig::default()`
/// (or assign directly in code). If you prefer not to recompile for every change,
/// consider using `TestConfig::from_env()` to read overrides from environment variables.
///
/// # Examples
///
/// ```rust,ignore
/// let mut cfg = TestConfig::default();
/// cfg.rpc_port = 18545;
/// cfg.rpc_username = "alice".into();
/// cfg.network = Network::Testnet;
/// ```
///
/// # Environment Overrides
///
/// Reads `RPC_PORT`, `RPC_USER`, `RPC_PASS`, and `RPC_NETWORK` environment variables to override defaults.
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// The port number for RPC communication with the Bitcoin node.
    /// A value of 0 indicates that an available port should be automatically selected.
    pub rpc_port: u16,
    /// The username for RPC authentication.
    /// Can be customized to match your `bitcoin.conf` `rpcuser` setting.
    pub rpc_username: String,
    /// The password for RPC authentication.
    /// Can be customized to match your `bitcoin.conf` `rpcpassword` setting.
    pub rpc_password: String,
    /// Which Bitcoin network to run against.
    pub network: Network,
}

impl TestConfig {
    /// Return the value used with `-chain=<value>` for the configured network
    pub fn as_chain_str(&self) -> &'static str {
        #[allow(unreachable_patterns)]
        match self.network {
            Network::Bitcoin => "main",
            Network::Regtest => "regtest",
            Network::Signet => "signet",
            Network::Testnet => "testnet",
            Network::Testnet4 => "testnet4",
            _ => panic!("Unsupported network variant"),
        }
    }

    /// Parse network from common strings (case-insensitive). Accepts: regtest, testnet|test,
    /// signet, mainnet|main|bitcoin, testnet4.
    pub fn network_from_str(s: &str) -> Option<Network> {
        match s.to_ascii_lowercase().as_str() {
            "regtest" => Some(Network::Regtest),
            "testnet" | "test" => Some(Network::Testnet),
            "signet" => Some(Network::Signet),
            "mainnet" | "main" | "bitcoin" => Some(Network::Bitcoin),
            "testnet4" => Some(Network::Testnet4),
            _ => None,
        }
    }

    /// Create a `TestConfig`, overriding defaults with environment variables:
    /// - `RPC_PORT`: overrides `rpc_port`
    /// - `RPC_USER`: overrides `rpc_username`
    /// - `RPC_PASS`: overrides `rpc_password`
    /// - `RPC_NETWORK`: one of `regtest`, `testnet|test`, `signet`, `mainnet|main|bitcoin`, `testnet4`
    #[allow(clippy::field_reassign_with_default)]
    pub fn from_env() -> Self {
        let mut cfg = Self::default();
        if let Ok(port_str) = env::var("RPC_PORT") {
            if let Ok(port) = port_str.parse() {
                cfg.rpc_port = port;
            }
        }
        if let Ok(user) = env::var("RPC_USER") {
            cfg.rpc_username = user;
        }
        if let Ok(pass) = env::var("RPC_PASS") {
            cfg.rpc_password = pass;
        }
        if let Ok(net) = env::var("RPC_NETWORK") {
            if let Some(n) = Self::network_from_str(&net) {
                cfg.network = n;
            }
        }
        cfg
    }

    /// Convert this test configuration into a full Config instance
    pub fn into_config(self) -> Config {
        Config {
            bitcoin: BitcoinConfig {
                host: "127.0.0.1".to_string(),
                port: self.rpc_port,
                username: self.rpc_username,
                password: self.rpc_password,
                network: Some(self.network),
            },
            ..Config::default()
        }
    }

    /// Create a TestConfig from a full Config instance
    pub fn from_config(config: &Config) -> Self {
        Self {
            rpc_port: config.bitcoin.port,
            rpc_username: config.bitcoin.username.clone(),
            rpc_password: config.bitcoin.password.clone(),
            network: config.bitcoin.network.unwrap_or(Network::Regtest), // Use config network if available
        }
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            rpc_port: 0,
            rpc_username: "rpcuser".to_string(),
            rpc_password: "rpcpassword".to_string(),
            network: Network::Regtest,
        }
    }
}
