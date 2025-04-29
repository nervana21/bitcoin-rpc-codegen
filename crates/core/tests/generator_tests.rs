// crates/core/tests/generator_tests.rs

use anyhow::Result;
use bitcoin_rpc_codegen::generator::generate_version_code;
use bitcoin_rpc_codegen::schema::{ApiMethod, ApiResult};
use std::fs;
use tempfile::tempdir;

#[test]
fn generate_minimal_schema_creates_expected_files() -> anyhow::Result<()> {
    // 1. Prepare a minimal ApiMethod with no args and one numeric result
    let methods = vec![ApiMethod {
        name: "foo".to_string(),
        type_: "method".to_string(),
        description: "A test method".to_string(),
        arguments: vec![],
        results: vec![ApiResult {
            type_: "number".to_string(),
            key_name: "bar".to_string(),
            description: "The bar result".to_string(),
            inner: Vec::new(),
        }],
        examples: Some("Example usage".to_string()),
    }];

    // 2. Create a temporary output directory
    let tmp = tempdir()?;
    let out = tmp.path();

    // 3. Run codegen
    generate_version_code("v0", &methods, out)?;

    // 4. Assert the expected modules exist
    let mod_rs = out.join("v0/mod.rs");
    let foo_rs = out.join("v0/foo.rs");
    let foo_types_rs = out.join("v0/foo_types.rs");

    assert!(mod_rs.exists(), "mod.rs not found at {:?}", mod_rs);
    assert!(foo_rs.exists(), "foo.rs not found at {:?}", foo_rs);
    assert!(
        foo_types_rs.exists(),
        "foo_types.rs not found at {:?}",
        foo_types_rs
    );

    // 5. Check that the module references our method
    let contents = fs::read_to_string(&mod_rs)?;
    assert!(
        contents.contains("pub mod foo"),
        "mod.rs should reference foo"
    );

    Ok(())
}

#[test]
fn test_generate_version_code() -> Result<()> {
    // 1. Prepare a minimal ApiMethod with no args and one numeric result
    let methods = vec![ApiMethod {
        name: "test".to_string(),
        type_: "method".to_string(),
        description: "Test method".to_string(),
        arguments: vec![],
        results: vec![ApiResult {
            type_: "string".to_string(),
            key_name: "result".to_string(),
            description: "Test result".to_string(),
            inner: vec![],
        }],
        examples: None,
    }];

    // 2. Create a temporary output directory
    let tmp = tempdir()?;
    let out = tmp.path();

    // 3. Run codegen
    generate_version_code("v1", &methods, out)?;

    // 4. Assert the expected files exist
    let version_dir = out.join("v1");
    let test_rs = version_dir.join("test.rs");
    let test_types_rs = version_dir.join("test_types.rs");
    let mod_rs = version_dir.join("mod.rs");

    assert!(test_rs.exists(), "test.rs not found at {:?}", test_rs);
    assert!(
        test_types_rs.exists(),
        "test_types.rs not found at {:?}",
        test_types_rs
    );
    assert!(mod_rs.exists(), "mod.rs not found at {:?}", mod_rs);

    // 5. Check that the module references our method
    let mod_contents = fs::read_to_string(&mod_rs)?;
    assert!(
        mod_contents.contains("pub mod test"),
        "mod.rs should reference test"
    );

    Ok(())
}
