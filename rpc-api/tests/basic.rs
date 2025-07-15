// rpc_api/tests/basic.rs

use rpc_api::{ApiArgument, ApiMethod, ApiResult, Type, Version, VersionError};
use serde_json;
use std::str::FromStr;

/// Make sure all supported version strings round‐trip through `Version::from_str`.
#[test]
fn supported_versions_parseable() {
    let versions = [Version::V28];
    for version in &versions {
        let s = version.to_string();
        let parsed = Version::from_str(&s).unwrap();
        assert_eq!(
            parsed, *version,
            "Version string → enum → string roundtrip failed"
        );
    }
}

/// Unknown version strings should error
#[test]
fn unknown_version_errors() {
    let err = Version::from_str("vX").unwrap_err();
    match err {
        VersionError::InvalidFormat(s) => assert_eq!(s, "vX"),
        _ => panic!("wrong error variant for invalid version"),
    }
}

/// Round‐trip ApiArgument through JSON
#[test]
fn api_argument_roundtrip() {
    let arg = ApiArgument {
        names: vec!["foo".into(), "bar".into()],
        type_: "string".into(),
        optional: true,
        description: "a test argument".into(),
    };
    let json = serde_json::to_string(&arg).expect("serialize ApiArgument");
    let de: ApiArgument = serde_json::from_str(&json).expect("deserialize ApiArgument");
    assert_eq!(de.names, arg.names);
    assert_eq!(de.type_, arg.type_);
    assert_eq!(de.optional, arg.optional);
    assert_eq!(de.description, arg.description);
}

/// Round‐trip ApiResult through JSON
#[test]
fn api_result_roundtrip() {
    let res = ApiResult {
        key_name: "result_key".into(),
        type_: "number".into(),
        description: "a test result".into(),
        inner: vec![],
        optional: false,
    };
    let json = serde_json::to_string(&res).expect("serialize ApiResult");
    let de: ApiResult = serde_json::from_str(&json).expect("deserialize ApiResult");
    assert_eq!(de.key_name, res.key_name);
    assert_eq!(de.type_, res.type_);
    assert_eq!(de.description, res.description);
}

/// Round‐trip ApiMethod through JSON
#[test]
fn api_method_roundtrip() {
    let method = ApiMethod {
        name: "testMethod".into(),
        description: "does nothing".into(),
        arguments: vec![ApiArgument {
            names: vec!["a".into()],
            type_: "bool".into(),
            optional: false,
            description: "a bool".into(),
        }],
        results: vec![ApiResult {
            key_name: "".into(),
            type_: "none".into(),
            description: "".into(),
            inner: vec![],
            optional: false,
        }],
    };
    let json = serde_json::to_string(&method).expect("serialize ApiMethod");
    let de: ApiMethod = serde_json::from_str(&json).expect("deserialize ApiMethod");
    assert_eq!(de.name, method.name);
    assert_eq!(de.description, method.description);
    assert_eq!(de.arguments.len(), 1);
    assert_eq!(de.results.len(), 1);
}

#[test]
fn type_is_optional_and_to_rust_type() {
    // Primitive
    let t = Type::Primitive("string".into());
    assert!(!t.is_optional());
    assert_eq!(t.to_rust_type(), "String");
    let t = Type::Primitive("boolean".into());
    assert_eq!(t.to_rust_type(), "bool");
    let t = Type::Primitive("number".into());
    assert_eq!(t.to_rust_type(), "f64");
    let t = Type::Primitive("integer".into());
    assert_eq!(t.to_rust_type(), "i64");
    let t = Type::Primitive("hex".into());
    assert_eq!(t.to_rust_type(), "String");
    let t = Type::Primitive("time".into());
    assert_eq!(t.to_rust_type(), "u64");
    let t = Type::Primitive("amount".into());
    assert_eq!(t.to_rust_type(), "f64");
    let t = Type::Primitive("unknown".into());
    assert_eq!(t.to_rust_type(), "serde_json::Value");
    // Option
    let t = Type::Option(Box::new(Type::Primitive("string".into())));
    assert!(t.is_optional());
    // Object
    let t = Type::Object(vec![("foo".into(), Type::Primitive("string".into()))]);
    assert_eq!(t.to_rust_type(), "{\n    pub foo: String,\n}");
    // Array
    let t = Type::Array(Box::new(Type::Primitive("string".into())));
    assert_eq!(t.to_rust_type(), "Vec<String>");
    // Tuple
    let t = Type::Tuple(vec![
        Type::Primitive("string".into()),
        Type::Primitive("number".into()),
    ]);
    assert_eq!(t.to_rust_type(), "(String, f64)");
    // Unit
    let t = Type::Unit;
    assert_eq!(t.to_rust_type(), "()");
}

#[test]
fn type_from_api_results_cases() {
    use rpc_api::ApiResult;
    // Empty
    let t = Type::from_api_results(&[]);
    assert!(matches!(t, Type::Unit));
    // Single unnamed primitive
    let t = Type::from_api_results(&[ApiResult {
        key_name: "".into(),
        type_: "string".into(),
        description: "desc".into(),
        inner: vec![],
        optional: false,
    }]);
    assert!(matches!(t, Type::Primitive(_)));
    // Single unnamed object with inner
    let t = Type::from_api_results(&[ApiResult {
        key_name: "".into(),
        type_: "object".into(),
        description: "desc".into(),
        inner: vec![ApiResult {
            key_name: "foo".into(),
            type_: "string".into(),
            description: "desc".into(),
            inner: vec![],
            optional: false,
        }],
        optional: false,
    }]);
    assert!(matches!(t, Type::Object(_)));
    // Single named result
    let t = Type::from_api_results(&[ApiResult {
        key_name: "foo".into(),
        type_: "string".into(),
        description: "desc".into(),
        inner: vec![],
        optional: false,
    }]);
    assert!(matches!(t, Type::Object(_)));
    // Multiple results
    let t = Type::from_api_results(&[
        ApiResult {
            key_name: "foo".into(),
            type_: "string".into(),
            description: "desc".into(),
            inner: vec![],
            optional: false,
        },
        ApiResult {
            key_name: "bar".into(),
            type_: "number".into(),
            description: "desc".into(),
            inner: vec![],
            optional: true,
        },
    ]);
    if let Type::Object(fields) = t {
        assert_eq!(fields.len(), 2);
        assert!(matches!(fields[1].1, Type::Option(_)));
    } else {
        panic!("Expected Type::Object");
    }
}

#[test]
fn extract_examples_cases() {
    use rpc_api::extract_examples;
    let desc = "This method.\nExample: foo\n```\nbar\n```\n";
    let ex = extract_examples(desc);
    assert!(ex.iter().any(|l| l.starts_with("Example:")));
    assert!(ex.iter().any(|l| l.starts_with("```")));
}

#[test]
fn parse_api_json_cases() {
    use rpc_api::parse_api_json;
    let json = r#"{"commands":{"foo":[{"description":"desc","arguments":[],"results":[]}]}}"#;
    let parsed = parse_api_json(json).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0].name, "foo");
    assert_eq!(parsed[0].description, "desc");
    assert!(parsed[0].arguments.is_empty());
    assert!(parsed[0].results.is_empty());
}

#[test]
fn from_apimethod_for_rpcmethod() {
    use rpc_api::{ApiArgument, ApiMethod, ApiResult, RpcMethod};
    let api_method = ApiMethod {
        name: "getfoo".into(),
        description: "desc".into(),
        arguments: vec![ApiArgument {
            names: vec!["foo".into()],
            type_: "string".into(),
            optional: false,
            description: "desc".into(),
        }],
        results: vec![ApiResult {
            key_name: "bar".into(),
            type_: "number".into(),
            description: "desc".into(),
            inner: vec![],
            optional: false,
        }],
    };
    let rpc_method: RpcMethod = api_method.into();
    assert_eq!(rpc_method.name, "getfoo");
    assert_eq!(rpc_method.description, "desc");
    assert_eq!(rpc_method.params.len(), 1);
    assert_eq!(rpc_method.result.is_some(), true);
}
