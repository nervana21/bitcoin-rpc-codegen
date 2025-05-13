use client::RpcClient;
use mockito::Server;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize)]
struct TestParams {
    value: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestResponse {
    result: u32,
}

#[derive(Debug, Serialize)]
struct ComplexParams {
    name: String,
    values: Vec<u32>,
    optional: Option<bool>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ComplexResponse {
    name: String,
    sum: u32,
    details: Vec<String>,
}

#[test]
fn test_low_level_api() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":"ok","id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(client.call_method("foo", &[])).unwrap();
    assert_eq!(res, json!("ok"));
}

#[test]
fn test_high_level_api() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":{"result":42},"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let params = TestParams { value: 123 };
    let res: TestResponse = rt.block_on(client.call("foo", &[params])).unwrap();
    assert_eq!(res, TestResponse { result: 42 });
}

#[test]
fn test_error_handling() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","error":{"code":-1,"message":"oops"},"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let err = rt.block_on(client.call_method("foo", &[])).unwrap_err();
    assert!(err.to_string().contains("oops"));
}

#[test]
fn test_with_auth() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .match_header("authorization", "Basic dXNlcjpwYXNz")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":"ok","id":1}"#)
        .create();

    let client = RpcClient::new_with_auth(&server.url(), "user", "pass");
    let rt = tokio::runtime::Runtime::new().unwrap();
    let res = rt.block_on(client.call_method("foo", &[])).unwrap();
    assert_eq!(res, json!("ok"));
}

#[test]
fn test_complex_params() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"jsonrpc":"2.0","result":{"name":"test","sum":15,"details":["a","b"]},"id":1}"#,
        )
        .create();

    let client = RpcClient::new(&server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let params = ComplexParams {
        name: "test".to_string(),
        values: vec![5, 10],
        optional: Some(true),
    };
    let res: ComplexResponse = rt.block_on(client.call("foo", &[params])).unwrap();
    assert_eq!(
        res,
        ComplexResponse {
            name: "test".to_string(),
            sum: 15,
            details: vec!["a".to_string(), "b".to_string()],
        }
    );
}

#[test]
fn test_multiple_params() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":42,"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let params1 = TestParams { value: 1 };
    let params2 = TestParams { value: 2 };
    let res: u32 = rt
        .block_on(client.call("foo", &[params1, params2]))
        .unwrap();
    assert_eq!(res, 42);
}

// TODO: Add tests for http error
// #[test]
// fn test_http_error() {
//     let mut server = Server::new();
//     let _m = server
//         .mock("POST", "/")
//         .with_status(500)
//         .with_header("content-type", "application/json")
//         .with_body(r#"{"error":"Internal Server Error"}"#)
//         .create();

//     let client = RpcClient::new(&server.url());
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let err = rt.block_on(client.call_method("foo", &[])).unwrap_err();
//     assert!(err.to_string().contains("500"));
// }

// TODO: Add tests for invalid json
// #[test]
// fn test_invalid_json() {
//     let mut server = Server::new();
//     let _m = server
//         .mock("POST", "/")
//         .with_status(200)
//         .with_header("content-type", "application/json")
//         .with_body(r#"{"jsonrpc":"2.0","result":invalid,"id":1}"#)
//         .create();

//     let client = RpcClient::new(&server.url());
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let err = rt.block_on(client.call_method("foo", &[])).unwrap_err();
//     assert!(err.to_string().contains("invalid"));
// }

// TODO: Add tests for missing result
// #[test]
// fn test_missing_result() {
//     let mut server = Server::new();
//     let _m = server
//         .mock("POST", "/")
//         .with_status(200)
//         .with_header("content-type", "application/json")
//         .with_body(r#"{"jsonrpc":"2.0","id":1}"#)
//         .create();

//     let client = RpcClient::new(&server.url());
//     let rt = tokio::runtime::Runtime::new().unwrap();
//     let err = rt.block_on(client.call_method("foo", &[])).unwrap_err();
//     assert!(err.to_string().contains("missing"));
// }
