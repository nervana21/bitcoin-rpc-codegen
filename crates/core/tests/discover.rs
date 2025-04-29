use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

use bitcoin_rpc_codegen::{discover_methods, parse_help_output};
use std::os::unix::fs::PermissionsExt;

/// Helper to create an executable shell script
fn create_executable(path: &Path, contents: &str) {
    let mut f = fs::File::create(path).unwrap();
    writeln!(f, "{}", contents).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

#[test]
fn parse_help_output_extracts_clean_rpc_names() {
    let sample = "getblockchaininfo\n getnetworkinfo \nsetban\nfoo_bar\n invalid-method\n";
    let parsed = parse_help_output(sample);
    assert_eq!(
        parsed,
        vec!["getblockchaininfo", "getnetworkinfo", "setban", "foo_bar"]
    );
}

#[test]
fn discover_on_bad_path_yields_empty() {
    let v = discover_methods(&PathBuf::from("/no/such/bin")).unwrap();
    assert!(v.is_empty());
}

#[test]
fn discover_methods_and_dump_help_texts() {
    let dir = tempdir().unwrap();
    let bitcoind = dir.path().join("bitcoind");
    let cli = dir.path().join("bitcoin-cli");

    // Create dummy bitcoind binary (no-op, just needed for path resolution)
    fs::File::create(&bitcoind).unwrap();

    // Fake bitcoin-cli script
    create_executable(
        &cli,
        r#"#!/bin/sh
if [ "$1" = "getnetworkinfo" ]; then
    echo '{ "version": 990000 }'
elif [ "$1" = "help" ] && [ -z "$2" ]; then
    echo foo
    echo bar
elif [ "$1" = "help" ] && [ "$2" = "foo" ]; then
    echo "Help text for foo"
elif [ "$1" = "help" ] && [ "$2" = "bar" ]; then
    echo "Help text for bar"
else
    echo "Unknown command" >&2
    exit 1
fi
"#,
    );

    // Call discover_methods
    let methods = discover_methods(&bitcoind).unwrap();
    assert_eq!(methods, vec!["foo".to_string(), "bar".to_string()]);

    // Check output files exist and are correct
    let docs_dir = Path::new("resources/docs/v99");
    let foo_txt = docs_dir.join("foo.txt");
    let bar_txt = docs_dir.join("bar.txt");

    assert!(foo_txt.exists());
    assert!(bar_txt.exists());

    let foo_contents = fs::read_to_string(&foo_txt).unwrap();
    assert!(foo_contents.contains("Help text for foo"));

    let bar_contents = fs::read_to_string(&bar_txt).unwrap();
    assert!(bar_contents.contains("Help text for bar"));
}
