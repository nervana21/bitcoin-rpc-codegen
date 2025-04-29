use bitcoin_rpc_codegen::error::CoreError;
use bitcoin_rpc_codegen::{
    discover_methods, extract_api_docs, fetch_bitcoind, generate_version_code, parse_api_json,
};
use std::fs;

#[test]
fn e2e_pipeline_test() -> Result<(), CoreError> {
    let version = "v29";

    let bitcoind_path = fetch_bitcoind(version)?;
    let methods = discover_methods(&bitcoind_path)?;
    assert!(!methods.is_empty());

    let temp_dir = tempfile::tempdir()?;
    let docs_dir = temp_dir.path().join("docs");
    let schema_file = temp_dir.path().join("schema.json");

    // Simulate dumping discovered methods into text files...
    extract_api_docs(&docs_dir, &schema_file)?;

    // Now parse and generate code
    let methods = parse_api_json(&fs::read_to_string(&schema_file)?)?;
    generate_version_code(version, &methods, temp_dir.path())?;

    Ok(())
}
