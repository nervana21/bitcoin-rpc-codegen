// transport/tests/transport.rs

use mockito::Server;
use serde_json::json;
use transport::{Transport, TransportError};

#[test]
fn send_request_success() {
    // 1) Spin up the mockito server (synchronous)
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":123,"id":1}"#)
        .create();

    // 2) Create our Transport pointing at mockito
    let tx = Transport::new(server.url());

    // 3) Use a dedicated Tokio runtime to run the async send_request
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(tx.send_request("foo", &[] as &[u8])).unwrap();

    // 4) Assert we got back the stubbed result
    assert_eq!(result, json!(123));
}

#[test]
fn send_request_rpc_error() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","error":{"code":-1,"message":"oops"},"id":1}"#)
        .create();

    let tx = Transport::new(server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let err = rt
        .block_on(tx.send_request("bar", &[] as &[u8]))
        .unwrap_err();

    match err {
        TransportError::Rpc(v) => {
            assert_eq!(v["message"], json!("oops"));
        }
        other => panic!("expected Rpc error, got {:?}", other),
    }
}
