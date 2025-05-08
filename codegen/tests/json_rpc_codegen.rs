use codegen::{CodeGenerator, JsonRpcCodeGenerator};
use rpc_api::ApiMethod;

#[test]
fn json_rpc_codegen_emits_reqwest_and_url() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: "".into(),
        arguments: vec![],
        results: vec![],
    }];

    let gen = JsonRpcCodeGenerator {
        url: "http://example.com".into(),
    };
    let files = gen.generate(&methods);

    // should produce exactly one module named "foo"
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "foo");

    // spot the reqwest import, the correct URL, and the JSON-RPC method name
    assert!(src.contains("use reqwest::Client;"));
    assert!(src.contains(r#"post("http://example.com")"#));
    assert!(src.contains(r#""method": "foo""#));
}
