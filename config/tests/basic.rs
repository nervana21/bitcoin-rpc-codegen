// config/tests/basic.rs

use config::Config;
use std::{env, path::PathBuf, thread, time::Instant};
use tempfile::TempDir;

struct TestEnv {
    temp_dir: TempDir,
    original_out_dir: Option<String>,
}

impl TestEnv {
    fn new() -> Self {
        let start = Instant::now();
        println!("TestEnv::new() - Starting at {:?}", start.elapsed());

        let temp_dir = TempDir::new().unwrap();
        println!(
            "TestEnv::new() - Created temp_dir: {:?} at {:?}",
            temp_dir.path(),
            start.elapsed()
        );

        let original_out_dir = env::var("OUT_DIR").ok();
        println!(
            "TestEnv::new() - Original OUT_DIR: {:?} at {:?}",
            original_out_dir,
            start.elapsed()
        );

        let out_dir_path = temp_dir.path().to_string_lossy().to_string();
        env::set_var("OUT_DIR", &out_dir_path);
        println!(
            "TestEnv::new() - Set OUT_DIR to: {:?} at {:?}",
            out_dir_path,
            start.elapsed()
        );

        // Verify the environment variable was set correctly
        let verify_out_dir = env::var("OUT_DIR").unwrap();
        println!(
            "TestEnv::new() - Verified OUT_DIR: {:?} at {:?}",
            verify_out_dir,
            start.elapsed()
        );

        Self {
            temp_dir,
            original_out_dir,
        }
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        let start = Instant::now();
        println!("TestEnv::drop() - Starting at {:?}", start.elapsed());

        match &self.original_out_dir {
            Some(value) => {
                env::set_var("OUT_DIR", value);
                println!(
                    "TestEnv::drop() - Restored OUT_DIR to: {:?} at {:?}",
                    value,
                    start.elapsed()
                );
            }
            None => {
                env::remove_var("OUT_DIR");
                println!("TestEnv::drop() - Removed OUT_DIR at {:?}", start.elapsed());
            }
        }
    }
}

#[test]
fn test_default_config() {
    let start = Instant::now();
    println!(
        "=== Starting test_default_config at {:?} ===",
        start.elapsed()
    );
    println!("Thread ID: {:?}", thread::current().id());

    let test_env = TestEnv::new();
    println!(
        "Created TestEnv with temp_dir: {:?} at {:?}",
        test_env.temp_dir.path(),
        start.elapsed()
    );
    println!(
        "OUT_DIR environment variable: {:?} at {:?}",
        env::var("OUT_DIR"),
        start.elapsed()
    );

    let config = Config::default();
    println!(
        "Created Config with output_dir: {:?} at {:?}",
        config.codegen.output_dir,
        start.elapsed()
    );

    assert_eq!(config.bitcoin.host, "127.0.0.1");
    assert_eq!(config.bitcoin.port, 18443);
    assert_eq!(config.bitcoin.username, "rpcuser");
    assert_eq!(config.bitcoin.password, "rpcpassword");
    assert_eq!(config.bitcoin.core_version, None);

    assert_eq!(config.logging.level, "info");
    assert!(config.logging.file.is_none());

    assert_eq!(config.codegen.input_path, PathBuf::from("api.json"));

    println!(
        "=== test_default_config completed successfully at {:?} ===",
        start.elapsed()
    );
}

#[test]
fn test_config_serialization_roundtrip() {
    let start = Instant::now();
    println!(
        "=== Starting test_config_serialization_roundtrip at {:?} ===",
        start.elapsed()
    );
    println!("Thread ID: {:?}", thread::current().id());

    let test_env = TestEnv::new();
    let config_path = test_env.temp_dir.path().join("config.toml");

    let original_config = Config::default();
    original_config.save(&config_path).unwrap();

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
    assert_eq!(original_config.logging.file, loaded_config.logging.file);

    assert_eq!(
        original_config.codegen.input_path,
        loaded_config.codegen.input_path
    );

    println!(
        "=== test_config_serialization_roundtrip completed at {:?} ===",
        start.elapsed()
    );
}

#[test]
fn test_custom_config_values() {
    let start = Instant::now();
    println!(
        "=== Starting test_custom_config_values at {:?} ===",
        start.elapsed()
    );
    println!("Thread ID: {:?}", thread::current().id());

    let _test_env = TestEnv::new();
    let mut config = Config::default();

    config.bitcoin.host = "192.168.1.1".to_string();
    config.bitcoin.port = 8333;
    config.bitcoin.username = "custom_user".to_string();
    config.bitcoin.password = "custom_pass".to_string();
    config.bitcoin.core_version = Some(28);

    config.logging.level = "debug".to_string();
    config.logging.file = Some(PathBuf::from("/var/log/bitcoin-rpc.log"));

    config.codegen.input_path = PathBuf::from("custom/api.json");
    config.codegen.output_dir = PathBuf::from("custom/output");

    assert_eq!(config.bitcoin.host, "192.168.1.1");
    assert_eq!(config.bitcoin.port, 8333);
    assert_eq!(config.bitcoin.username, "custom_user");
    assert_eq!(config.bitcoin.password, "custom_pass");
    assert_eq!(config.bitcoin.core_version, Some(28));

    assert_eq!(config.logging.level, "debug");
    assert_eq!(
        config.logging.file,
        Some(PathBuf::from("/var/log/bitcoin-rpc.log"))
    );

    assert_eq!(config.codegen.input_path, PathBuf::from("custom/api.json"));
    assert_eq!(config.codegen.output_dir, PathBuf::from("custom/output"));

    println!(
        "=== test_custom_config_values completed at {:?} ===",
        start.elapsed()
    );
}
