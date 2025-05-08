use codegen::{CodeGenerator, TransportCodeGenerator};
use rpc_api::{ApiArgument, ApiMethod};

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

#[test]
fn transport_codegen_with_args() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: "".into(),
        arguments: vec![
            ApiArgument {
                names: vec!["arg1".into()],
                description: "".into(),
                optional: false,
                type_: "string".into(),
            },
            ApiArgument {
                names: vec!["arg2".into()],
                description: "".into(),
                optional: false,
                type_: "number".into(),
            },
        ],
        results: vec![],
    }];

    let gen = TransportCodeGenerator;
    let files = gen.generate(&methods);
    assert_eq!(files.len(), 1);

    let (_mod, src) = &files[0];
    // 1) signature includes both args
    assert!(src.contains(
        "pub async fn foo(transport: &Transport, arg1: serde_json::Value, arg2: serde_json::Value)"
    ));
    // 2) params vec serializes them in order
    assert!(src.contains("let params = vec![json!(arg1), json!(arg2)];"));
    // 3) send_request call remains correct
    assert!(src.contains("transport.send_request(\"foo\", &params).await"));
}
