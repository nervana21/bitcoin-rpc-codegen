// rpc-cli/tests/basic.rs

use anyhow::Result;
use clap::Parser;
use rpc_cli::Cli;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_cli_parsing() -> Result<()> {
    // Test with minimal args
    let args = vec!["rpc-cli"];
    let cli = Cli::try_parse_from(args)?;
    assert!(cli.core_version.is_none());
    assert!(cli.bitcoind_path.is_none());
    assert!(cli.output_dir.is_none());
    assert!(!cli.verbose);

    // Test with all args
    let temp_dir = tempdir()?;
    let args = vec![
        "bitcoin-rpc-codegen-cli",
        "--bitcoin-core-version",
        "v29",
        "--bitcoind",
        "/usr/local/bin/bitcoind",
        "--output",
        temp_dir.path().to_str().unwrap(),
        "--verbose",
    ];
    let cli = Cli::try_parse_from(args)?;
    assert_eq!(cli.core_version, Some("v29".to_string()));
    assert_eq!(
        cli.bitcoind_path,
        Some(PathBuf::from("/usr/local/bin/bitcoind"))
    );
    assert_eq!(cli.output_dir, Some(temp_dir.path().to_path_buf()));
    assert!(cli.verbose);

    Ok(())
}
