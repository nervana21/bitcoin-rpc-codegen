// rpc_api/tests/basic.rs

use rpc_api::{ApiArgument, ApiMethod, ApiResult, Version, VersionError};
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
