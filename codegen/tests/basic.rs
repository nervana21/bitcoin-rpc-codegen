// codegen/tests/basic.rs

use codegen::{BasicCodeGenerator, CodeGenerator};
use rpc_api::ApiMethod;

#[test]
fn basic_codegen_emits_a_stub() {
    let methods = vec![ApiMethod {
        name: "foo".into(),
        description: String::new(),
        arguments: vec![],
        results: vec![],
    }];

    let gen = BasicCodeGenerator;
    let files = gen.generate(&methods);
    assert_eq!(files.len(), 1);
    let (mod_name, src) = &files[0];
    assert_eq!(mod_name, "foo");
    assert!(src.contains("pub fn foo"));
    assert!(src.contains("unimplemented!"));
}
