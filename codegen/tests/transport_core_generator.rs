use codegen::{CodeGenerator, TransportCoreGenerator};
use rpc_api::ApiMethod;

#[test]
fn transport_core_generator_includes_wallet_fallback_logic() {
    // Methods are unused by this generator, but pass an empty list for clarity
    let gen = TransportCoreGenerator;
    let files = gen.generate(&[] as &[ApiMethod]);

    // Should produce exactly one file named "core.rs"
    assert_eq!(files.len(), 1);
    let (name, src) = &files[0];
    assert_eq!(name, "core.rs");

    // Verify the generated transport contains wallet-aware routing and fallback
    assert!(
        src.contains("with_wallet("),
        "expected .with_wallet(...) helper in DefaultTransport"
    );
    assert!(
        src.contains("/wallet/"),
        "expected wallet endpoint path to be constructed"
    );
    assert!(
        src.contains("-32601"),
        "expected -32601 (method not found) fallback logic"
    );
    assert!(
        src.contains("wallet_name"),
        "expected wallet_name field for scoping requests"
    );
}

#[test]
fn transport_core_generator_emits_transport_traits() {
    let gen = TransportCoreGenerator;
    let files = gen.generate(&[] as &[ApiMethod]);
    let (_name, src) = &files[0];

    // Basic API surface checks
    assert!(src.contains("pub trait TransportTrait"));
    assert!(src.contains("pub trait TransportExt"));
    assert!(src.contains("pub struct DefaultTransport"));
}
