//! Smoke-test: ensure generator and parser modules exist

#[test]
fn smoke() {
    // Verify that the codegen generator and parser modules compile and link
    use bitcoin_rpc_codegen::generator;
    use bitcoin_rpc_codegen::parser;

    // Access constants and functions to assert presence
    let _versions: &[&str] = generator::SUPPORTED_VERSIONS;
    let _parse_fn: fn(&str) -> Result<_, _> = parser::parse_api_json;
}
