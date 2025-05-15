use client::RpcClient;
use mockito::Server;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Once;

static INIT: Once = Once::new();

fn setup() {
    INIT.call_once(|| {});
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestParams {
    value: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TestResponse {
    result: i32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ComplexResponse {
    name: String,
    sum: u32,
    details: Vec<String>,
}

#[tokio::test]
async fn test_low_level_api() {
    setup();
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":42,"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let params = vec![Value::Number(123.into())];
    let res: Value = client.call_raw("foo", &params).await.unwrap();
    assert_eq!(res, 42);
    mock.assert();
}

#[tokio::test]
async fn test_high_level_api() {
    setup();
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":{"result":42},"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let params = TestParams { value: 123 };
    let res: TestResponse = client.call("foo", &[params]).await.unwrap();
    assert_eq!(res, TestResponse { result: 42 });
    mock.assert();
}

#[tokio::test]
async fn test_error_handling() {
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let params = vec![Value::Number(123.into())];
    let res = client.call_raw("foo", &params).await;
    assert!(res.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_with_auth() {
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .match_header("authorization", "Basic dXNlcjpwYXNz")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":42,"id":1}"#)
        .create();

    let client = RpcClient::new_with_auth(&server.url(), "user", "pass");
    let params = vec![Value::Number(123.into())];
    let res: Value = client.call_raw("foo", &params).await.unwrap();
    assert_eq!(res, 42);
    mock.assert();
}

#[tokio::test]
async fn test_complex_params() {
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"{"jsonrpc":"2.0","result":{"name":"test","sum":15,"details":["a","b"]},"id":1}"#,
        )
        .create();

    let client = RpcClient::new(&server.url());
    let params = vec![
        Value::String("test".to_string()),
        Value::Number(15.into()),
        Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]),
    ];
    let res: Value = client.call_raw("foo", &params).await.unwrap();
    assert_eq!(
        res,
        serde_json::json!({
            "name": "test",
            "sum": 15,
            "details": ["a", "b"]
        })
    );
    mock.assert();
}

#[tokio::test]
async fn test_multiple_params() {
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":[1,2],"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let params1 = TestParams { value: 1 };
    let params2 = TestParams { value: 2 };
    let res: Vec<i32> = client.call("foo", &[params1, params2]).await.unwrap();
    assert_eq!(res, vec![1, 2]);
    mock.assert();
}

#[tokio::test]
async fn test_http_error() {
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"Internal Server Error"}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let res = client.call_raw("foo", &[]).await;
    assert!(res.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_invalid_json() {
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":invalid,"id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let res = client.call_raw("foo", &[]).await;
    assert!(res.is_err());
    mock.assert();
}

#[tokio::test]
async fn test_missing_result() {
    let mut server = Server::new_with_opts(mockito::ServerOpts::default());
    let mock = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","id":1}"#)
        .create();

    let client = RpcClient::new(&server.url());
    let res = client.call_raw("foo", &[]).await;
    assert!(res.is_err());
    mock.assert();
}
