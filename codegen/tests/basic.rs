// codegen/tests/basic.rs

use codegen::{write_generated, BasicCodeGenerator, CodeGenerator};
use rpc_api::ApiMethod;
use std::fs;
use tempfile::TempDir;

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

#[test]
fn file_emission_smoke_test() {
    // 1) prepare a single dummy ApiMethod
    let m = ApiMethod {
        name: "foo".into(),
        description: String::new(),
        arguments: vec![],
        results: vec![],
    };
    let methods = vec![m];

    // 2) generate stubs in memory
    let gen = BasicCodeGenerator;
    let files = gen.generate(&methods);

    // 3) emit to a temp dir
    let tmp = TempDir::new().expect("tempdir");
    write_generated(tmp.path(), &files).expect("write_generated");

    // 4) assert foo.rs exists and contains the stub
    let foo_rs = tmp.path().join("foo.rs");
    assert!(foo_rs.exists(), "expected {} to exist", foo_rs.display());

    let contents = fs::read_to_string(&foo_rs).unwrap();
    assert!(contents.contains("pub fn foo"), "stub missing");
}
