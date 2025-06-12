// config/tests/basic.rs

use config::{Config, ConfigError};
use std::{env, fs, path::PathBuf};
use tempfile::TempDir;

// Helper function to set up test environment
fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    env::set_var("OUT_DIR", temp_dir.path());
    temp_dir
}

#[test]
fn test_default_config() {
    let temp_dir = setup_test_env();
    let config = Config::default();

    // Test Bitcoin config defaults
    assert_eq!(config.bitcoin.host, "127.0.0.1");
    assert_eq!(config.bitcoin.port, 8332);
    assert_eq!(config.bitcoin.username, "rpcuser");
    assert_eq!(config.bitcoin.password, "rpcpassword");
    assert_eq!(config.bitcoin.core_version, None);

    // Test Logging config defaults
    assert_eq!(config.logging.level, "info");
    assert!(config.logging.file.is_none());

    // Test Codegen config defaults
    assert_eq!(config.codegen.input_path, Config::default_help_path());

    // Instead of comparing exact paths, verify that the output_dir is within the temp directory
    assert!(config.codegen.output_dir.starts_with(temp_dir.path()));
}

#[test]
fn test_config_serialization_roundtrip() {
    let temp_dir = setup_test_env();
    let config_path = temp_dir.path().join("config.toml");

    // Create and save config
    let original_config = Config::default();
    original_config.save(&config_path).unwrap();

    // Verify file exists
    assert!(config_path.exists());

    let loaded_config = Config::from_file(&config_path).unwrap();

    // Test Bitcoin config
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

    // Test Logging config
    assert_eq!(original_config.logging.level, loaded_config.logging.level);
    assert_eq!(original_config.logging.file, loaded_config.logging.file);

    // Test Codegen config
    assert_eq!(
        original_config.codegen.input_path,
        loaded_config.codegen.input_path
    );
    assert_eq!(
        original_config.codegen.output_dir,
        loaded_config.codegen.output_dir
    );
}

#[test]
fn test_invalid_config_file() {
    let temp_dir = setup_test_env();
    let config_path = temp_dir.path().join("invalid.toml");

    // Create invalid TOML content
    fs::write(&config_path, "invalid toml content").unwrap();

    // Attempt to load invalid config
    let result = Config::from_file(&config_path);
    assert!(matches!(result, Err(ConfigError::Parse(_))));
}

#[test]
fn test_nonexistent_config_file() {
    let temp_dir = setup_test_env();
    let config_path = temp_dir.path().join("nonexistent.toml");

    // Attempt to load nonexistent config
    let result = Config::from_file(&config_path);
    assert!(matches!(result, Err(ConfigError::FileRead(_))));
}

#[test]
fn test_custom_config_values() {
    setup_test_env();
    let mut config = Config::default();

    // Modify Bitcoin config
    config.bitcoin.host = "192.168.1.1".to_string();
    config.bitcoin.port = 8333;
    config.bitcoin.username = "custom_user".to_string();
    config.bitcoin.password = "custom_pass".to_string();
    config.bitcoin.core_version = Some("v29".to_string());

    // Modify Logging config
    config.logging.level = "debug".to_string();
    config.logging.file = Some(PathBuf::from("/var/log/bitcoin-rpc.log"));

    // Modify Codegen config
    config.codegen.input_path = PathBuf::from("custom/help.txt");
    config.codegen.output_dir = PathBuf::from("custom/output");

    // Verify custom values
    assert_eq!(config.bitcoin.host, "192.168.1.1");
    assert_eq!(config.bitcoin.port, 8333);
    assert_eq!(config.bitcoin.username, "custom_user");
    assert_eq!(config.bitcoin.password, "custom_pass");
    assert_eq!(config.bitcoin.core_version, Some("v29".to_string()));

    assert_eq!(config.logging.level, "debug");
    assert_eq!(
        config.logging.file,
        Some(PathBuf::from("/var/log/bitcoin-rpc.log"))
    );

    assert_eq!(config.codegen.input_path, PathBuf::from("custom/help.txt"));
    assert_eq!(config.codegen.output_dir, PathBuf::from("custom/output"));
}
