// crates/core/tests/generator_tests.rs

use bitcoin_rpc_codegen::generator::generate_version_code;
use bitcoin_rpc_codegen::schema::{ApiMethod, ApiResult};
use std::fs;
use tempfile::tempdir;

#[test]
fn generate_minimal_schema_creates_expected_files() -> anyhow::Result<()> {
    // 1. Prepare a minimal ApiMethod with no args and one numeric result
    let methods = vec![ApiMethod {
        name: "foo".to_string(),
        description: "A test method".to_string(),
        arguments: vec![],
        results: vec![ApiResult {
            type_: "number".to_string(),
            key_name: "bar".to_string(),
            description: "The bar result".to_string(),
            inner: Vec::new(),
        }],
    }];

    // 2. Create a temporary output directory
    let tmp = tempdir()?;
    let out = tmp.path().join("gen");

    // 3. Run codegen, passing a &PathBuf for the output dir
    generate_version_code("v0", &methods, &out)?;

    // 4. Assert the expected client and types modules exist
    let client_mod = out.join("client/src/v0/mod.rs");
    let client_foo = out.join("client/src/v0/foo.rs");
    let types_mod = out.join("types/src/v0/mod.rs");
    let types_foo = out.join("types/src/v0/foo.rs");

    assert!(
        client_mod.exists(),
        "client mod.rs not found at {:?}",
        client_mod
    );
    assert!(
        client_foo.exists(),
        "client foo.rs not found at {:?}",
        client_foo
    );
    assert!(
        types_mod.exists(),
        "types mod.rs not found at {:?}",
        types_mod
    );
    assert!(
        types_foo.exists(),
        "types foo.rs not found at {:?}",
        types_foo
    );

    // 5. Check that the client module references our method
    let contents = fs::read_to_string(&client_mod)?;
    assert!(
        contents.contains("pub mod foo"),
        "mod.rs should reference foo"
    );

    Ok(())
}
