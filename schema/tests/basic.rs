// schema/tests/basic.rs

use parser::{DefaultHelpParser, HelpParser};
use schema::{DefaultSchemaNormalizer, DefaultSchemaValidator, SchemaNormalizer, SchemaValidator};

static SAMPLE_HELP: &str = r#"
foo
First method.

bar
Second method.
"#;

#[test]
fn normalize_and_validate_success() {
    let parser = DefaultHelpParser;
    let helps = parser.parse(SAMPLE_HELP).expect("parse help");
    let schema = DefaultSchemaNormalizer
        .normalize(&helps)
        .expect("normalize");
    assert_eq!(schema.len(), 2);
    assert_eq!(schema[0].name, "foo");
    assert_eq!(schema[1].name, "bar");

    // validation should pass
    DefaultSchemaValidator.validate(&schema).unwrap();
}

#[test]
fn normalize_empty_blocks_errors() {
    let schema_err = DefaultSchemaNormalizer.normalize(&[]).unwrap_err();
    matches!(schema_err, schema::NormalizeError::NoHelpBlocks);
}

#[test]
fn validate_duplicate_name_errors() {
    let dup = vec![
        rpc_api::ApiMethod {
            name: "dup".into(),
            description: "".into(),
            arguments: vec![],
            results: vec![],
        },
        rpc_api::ApiMethod {
            name: "dup".into(),
            description: "".into(),
            arguments: vec![],
            results: vec![],
        },
    ];
    let err = DefaultSchemaValidator.validate(&dup).unwrap_err();
    matches!(err, schema::ValidateError::DuplicateName(_));
}
