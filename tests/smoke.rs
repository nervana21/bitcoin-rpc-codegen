// tests/smoke.rs
//! Smoke-test: ensure generator’s version logic and parser exist

#[test]
fn smoke() {
    use anyhow::Result;
    use bitcoin_rpc_codegen::generator::versions::Version;
    use bitcoin_rpc_codegen::parser;
    let _versions: &[Version] = Version::SUPPORTED;
    let _parse_fn: fn(&str) -> Result<Vec<parser::ApiMethod>> = parser::parse_api_json;
}
