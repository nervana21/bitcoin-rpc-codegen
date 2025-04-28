use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

use bitcoin_rpc_codegen::discover_methods;
use std::{os::unix::fs::PermissionsExt, path::Path};

#[test]
fn discover_on_bad_path_yields_empty() {
    let v = discover_methods(&PathBuf::from("/no/such/bin")).unwrap();
    assert!(v.is_empty());
}

#[test]
fn discover_methods_from_dummy_cli() {
    // Create a temp dir with dummy binaries
    let dir = tempdir().unwrap();
    let bitcoind = dir.path().join("bitcoind");
    let cli = dir.path().join("bitcoin-cli");

    // Dummy bitcoind (just exists)
    fs::File::create(&bitcoind).unwrap();
    // Dummy bitcoin-cli with help output
    {
        let mut f = fs::File::create(&cli).unwrap();
        writeln!(
            f,
            "#!/bin/sh
echo foo
echo bar baz
echo invalid!line"
        )
        .unwrap();
    }
    // Make both executable
    let mut perms = fs::metadata(&bitcoind).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&bitcoind, perms.clone()).unwrap();
    fs::set_permissions(&cli, perms).unwrap();

    // Call discover_methods on our dummy bitcoind
    let methods = discover_methods(&bitcoind).unwrap();

    // Should only capture 'foo' and 'bar'
    assert_eq!(methods, vec!["foo".to_string(), "bar".to_string()]);
}
