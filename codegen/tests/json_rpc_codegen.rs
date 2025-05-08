use codegen::{CodeGenerator, TransportCodeGenerator};
use rpc_api::ApiMethod;

#[test]
fn test_json_rpc_codegen_transport() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: "".into(),
        arguments: vec![],
        results: vec![],
    }];

    let gen = TransportCodeGenerator;
    let files = gen.generate(&methods);

    // should produce exactly one module named "foo"
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "foo");

    // now should reference Transport
    assert!(src.contains("use transport::Transport;"));
    assert!(src.contains("transport.send_request(\"foo\""));
    assert!(src.contains("pub async fn foo"));
}
