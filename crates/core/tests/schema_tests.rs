// crates/core/tests/schema_tests.rs

use bitcoin_rpc_codegen::schema::{parse_method_doc, ApiArgument, ApiResult};

/// A minimal help‐text example, covering signature, description, one arg, and one result.
const MINIMAL_DOC: &str = r#"
mycmd "arg1" (string, required)

Does something cool.

Arguments:
1. arg1   (string, required) The first argument.

Result:
"res"    (numeric) The result.
"#;

#[test]
fn parse_method_doc_minimal() {
    let method = parse_method_doc("mycmd", MINIMAL_DOC);

    // Name
    assert_eq!(method.name, "mycmd");

    // Description should skip the signature line and blank, capturing only the prose
    assert_eq!(method.description.trim(), "Does something cool.");

    // Single argument
    assert_eq!(
        method.arguments,
        vec![ApiArgument {
            names: vec!["arg1".into()],
            type_: "string".into(),
            optional: false,
            description: "The first argument.".into(),
        }],
    );

    // Single result field
    assert_eq!(
        method.results,
        vec![ApiResult {
            type_: "number".into(),
            key_name: "res".into(),
            description: "The result.".into(),
            inner: Vec::new(),
        }],
    );
}
