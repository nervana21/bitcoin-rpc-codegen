// config/tests/basic.rs

use config::{Config, ConfigError};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_default_config() {
    let config = Config::default();

    // Test Bitcoin config defaults
    assert_eq!(config.bitcoin.host, "127.0.0.1");
    assert_eq!(config.bitcoin.port, 8332);
    assert_eq!(config.bitcoin.username, "rpcuser");
    assert_eq!(config.bitcoin.password, "rpcpassword");

    // Test Logging config defaults
    assert_eq!(config.logging.level, "info");
    assert!(config.logging.file.is_none());

    // Test Metrics config defaults
    assert!(config.metrics.enabled);
    assert_eq!(config.metrics.port, 9090);
}

#[test]
fn test_config_serialization_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create and save config
    let original_config = Config::default();
    original_config.save(&config_path).unwrap();

    // Verify file exists
    assert!(config_path.exists());

    let loaded_config = Config::from_file(&config_path).unwrap();
    assert_eq!(original_config.bitcoin.host, loaded_config.bitcoin.host);
    assert_eq!(original_config.bitcoin.port, loaded_config.bitcoin.port);
    assert_eq!(
        original_config.bitcoin.username,
        loaded_config.bitcoin.username
    );
    assert_eq!(
        original_config.bitcoin.password,
        loaded_config.bitcoin.password
    );
    assert_eq!(
        original_config.bitcoin.core_version,
        loaded_config.bitcoin.core_version
    );

    assert_eq!(original_config.logging.level, loaded_config.logging.level);
    assert_eq!(
        original_config.metrics.enabled,
        loaded_config.metrics.enabled
    );
    assert_eq!(original_config.metrics.port, loaded_config.metrics.port);
}

#[test]
fn test_invalid_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("invalid.toml");

    // Create invalid TOML content
    fs::write(&config_path, "invalid toml content").unwrap();

    // Attempt to load invalid config
    let result = Config::from_file(&config_path);
    assert!(matches!(result, Err(ConfigError::Parse(_))));
}

#[test]
fn test_nonexistent_config_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("nonexistent.toml");

    // Attempt to load nonexistent config
    let result = Config::from_file(&config_path);
    assert!(matches!(result, Err(ConfigError::FileRead(_))));
}

#[test]
fn test_custom_config_values() {
    let mut config = Config::default();

    // Modify Bitcoin config
    config.bitcoin.host = "192.168.1.1".to_string();
    config.bitcoin.port = 8333;
    config.bitcoin.username = "custom_user".to_string();
    config.bitcoin.password = "custom_pass".to_string();

    // Modify Logging config
    config.logging.level = "debug".to_string();

    // Modify Metrics config
    config.metrics.enabled = false;
    config.metrics.port = 9091;

    // Verify custom values
    assert_eq!(config.bitcoin.host, "192.168.1.1");
    assert_eq!(config.bitcoin.port, 8333);
    assert_eq!(config.bitcoin.username, "custom_user");
    assert_eq!(config.bitcoin.password, "custom_pass");
    assert_eq!(config.logging.level, "debug");
    assert!(!config.metrics.enabled);
    assert_eq!(config.metrics.port, 9091);
}
