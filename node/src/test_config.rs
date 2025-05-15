// node/src/test_config.rs

use config::{BitcoinConfig, Config};
use std::env;

/// TestConfig represents the configuration needed to run a Bitcoin node in a test environment.
/// This struct is the single source of truth for test‑node settings: RPC port, username, and password.
/// Defaults are:
/// - `rpc_port = 0` (auto‑select a free port)
/// - `rpc_username = "rpcuser"`
/// - `rpc_password = "rpcpassword"`
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
/// ```
///
/// # Environment Overrides
///
/// Reads `RPC_PORT`, `RPC_USER`, and `RPC_PASS` environment variables to override defaults.
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
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            rpc_port: 0,
            rpc_username: "rpcuser".to_string(),
            rpc_password: "rpcpassword".to_string(),
        }
    }
}

impl TestConfig {
    /// Create a `TestConfig`, overriding defaults with environment variables:
    /// - `RPC_PORT`: overrides `rpc_port`
    /// - `RPC_USER`: overrides `rpc_username`
    /// - `RPC_PASS`: overrides `rpc_password`
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
                core_version: None,
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
        }
    }
}
