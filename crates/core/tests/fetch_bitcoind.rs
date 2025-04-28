use std::{env, fs, io::Write, path::PathBuf};
use tempfile::tempdir;

use bitcoin_rpc_codegen as core;

#[test]
fn fetch_bitcoind_prefers_env_var() {
    // create a dummy "bitcoind" file
    let dir = tempdir().unwrap();
    let dummy = dir.path().join("bitcoind");
    let mut f = fs::File::create(&dummy).unwrap();
    writeln!(f, "#!/bin/sh\necho fake").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perm = fs::metadata(&dummy).unwrap().permissions();
        perm.set_mode(0o755);
        fs::set_permissions(&dummy, perm).unwrap();
    }

    // point BITCOIND_PATH at our dummy
    env::set_var("BITCOIND_PATH", &dummy);
    let found = core::fetch_bitcoind("v999").unwrap();
    assert_eq!(found, dummy);

    // clean up
    env::remove_var("BITCOIND_PATH");
}

#[test]
fn fetch_bitcoind_errors_when_missing_and_unsupported_version() {
    env::remove_var("BITCOIND_PATH");
    // v0 → normalization to "0.0" → no such download
    let err = core::fetch_bitcoind("v0").unwrap_err();
    let msg = format!("{}", err);
    assert!(
        msg.contains("Failed HTTP GET") || msg.contains("bitcoind not found"),
        "unexpected error: {}",
        msg
    );
}

#[test]
fn default_platform_reports_something() {
    // We can only check it returns a non‐empty string
    let plat = core::fetch::default_platform();
    assert!(!plat.trim().is_empty());
}
