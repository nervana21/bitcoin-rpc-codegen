use bitcoin_rpc_codegen as core;
use std::path::PathBuf;

#[test]
fn smoke_pipeline_signatures_compile() {
    // Ensure BITCOIND_PATH is unset so fetch_bitcoind errors
    std::env::remove_var("BITCOIND_PATH");
    // fetch should fail for "v0"
    assert!(core::fetch_bitcoind("v0").is_err());

    // discover on dummy path yields empty vec
    let v = core::discover_methods(&PathBuf::from("/no/such/bin")).unwrap();
    assert!(v.is_empty());

    // parse on empty JSON yields empty vec
    let p = core::parse_api_json("");
    assert!(p.unwrap().is_empty());

    // generate with no methods is a no-op
    let out = PathBuf::from("target/tmp");
    assert!(core::generate_version_code("v0", &[], &out).is_ok());
}
