// transport/tests/transport.rs

use mockito::Server;
use serde_json::json;
use std::sync::Arc;
use transport::{BatchTransport, Transport, TransportError, TransportTrait};

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
        TransportError::Rpc(s) => {
            assert!(s.contains("oops"));
        }
        other => panic!("expected Rpc error, got {:?}", other),
    }
}

#[test]
fn test_connection_error() {
    // Create a transport pointing to a non-existent port
    let tx = Transport::new("http://127.0.0.1:0");
    let rt = tokio::runtime::Runtime::new().unwrap();

    let err = rt
        .block_on(tx.send_request("foo", &[] as &[u8]))
        .unwrap_err();

    match err {
        TransportError::Http(0, _) => {
            // Success - we got a connection error with status 0
        }
        other => panic!("expected Http(0, _) error, got {:?}", other),
    }
}

#[test]
fn send_batch_success() {
    // 1) Spin up the mockito server (synchronous)
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[{"jsonrpc":"2.0","result":123,"id":0},{"jsonrpc":"2.0","result":"abc","id":1}]"#,
        )
        .create();

    // 2) Create our Transport pointing at mockito
    let tx = Transport::new(server.url());

    // 3) Use a dedicated Tokio runtime to run the async send_batch
    let rt = tokio::runtime::Runtime::new().unwrap();
    let batch_requests = vec![
        json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "foo",
            "params": []
        }),
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "bar",
            "params": ["test"]
        }),
    ];
    let results = rt.block_on(tx.send_batch(&batch_requests)).unwrap();

    // 4) Assert we got back the expected results
    assert_eq!(results.len(), 2);
    assert_eq!(results[0]["result"], json!(123));
    assert_eq!(results[1]["result"], json!("abc"));
}

// NEW TESTS BELOW

#[test]
fn call_method_with_type_deserialization() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":{"name":"test","value":42},"id":1}"#)
        .create();

    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct TestResult {
        name: String,
        value: u32,
    }

    let tx = Transport::new(server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result: TestResult = rt.block_on(tx.call("test_method", &[] as &[u8])).unwrap();

    assert_eq!(
        result,
        TestResult {
            name: "test".to_string(),
            value: 42,
        }
    );
}

#[test]
fn missing_result_error() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","id":1}"#) // Missing result field
        .create();

    let tx = Transport::new(server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let err = rt
        .block_on(tx.send_request("foo", &[] as &[u8]))
        .unwrap_err();

    match err {
        TransportError::MissingResult => {
            // Success - we got the expected error
        }
        other => panic!("expected MissingResult error, got {:?}", other),
    }
}

#[test]
fn serialization_error() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"invalid json"#) // Invalid JSON
        .create();

    let tx = Transport::new(server.url());
    let rt = tokio::runtime::Runtime::new().unwrap();
    let err = rt
        .block_on(tx.send_request("foo", &[] as &[u8]))
        .unwrap_err();

    match err {
        TransportError::Http(0, reqwest_error) => {
            // Robust: check that this is a decode error (JSON parse error)
            assert!(
                reqwest_error.is_decode(),
                "Expected a decode error, got: {:?}",
                reqwest_error
            );
        }
        other => panic!("expected Http(0, _) error, got {:?}", other),
    }
}

#[test]
fn transport_trait_send_request() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"jsonrpc":"2.0","result":"trait_test","id":1}"#)
        .create();

    let tx = Arc::new(Transport::new(server.url()));
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt
        .block_on(tx.send_request("foo", &[] as &[serde_json::Value]))
        .unwrap();

    assert_eq!(result, json!("trait_test"));
}

#[test]
fn batch_transport_basic_functionality() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"[{"jsonrpc":"2.0","result":123,"id":0},{"jsonrpc":"2.0","result":"abc","id":1}]"#,
        )
        .create();

    let inner_tx = Arc::new(Transport::new(server.url()));
    let batch_tx = BatchTransport::new(inner_tx);
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Test that we're not batching initially
    assert!(!batch_tx.is_batching());

    // Start a batch
    batch_tx.begin_batch();
    assert!(batch_tx.is_batching());

    // Queue some requests
    let _ = rt.block_on(batch_tx.send_request("foo", &[] as &[serde_json::Value]));
    let _ = rt.block_on(batch_tx.send_request("bar", &[json!("test")] as &[serde_json::Value]));

    // End the batch and get results
    let results = rt.block_on(batch_tx.end_batch()).unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0], json!(123));
    assert_eq!(results[1], json!("abc"));

    // Verify we're no longer batching
    assert!(!batch_tx.is_batching());
}

#[test]
fn batch_transport_empty_batch() {
    let inner_tx = Arc::new(Transport::new("http://127.0.0.1:18443"));
    let batch_tx = BatchTransport::new(inner_tx);
    let rt = tokio::runtime::Runtime::new().unwrap();

    batch_tx.begin_batch();
    let results = rt.block_on(batch_tx.end_batch()).unwrap();
    assert_eq!(results.len(), 0);
}

#[test]
fn batch_transport_no_batch_in_progress() {
    let inner_tx = Arc::new(Transport::new("http://127.0.0.1:18443"));
    let batch_tx = BatchTransport::new(inner_tx);
    let rt = tokio::runtime::Runtime::new().unwrap();

    let err = rt.block_on(batch_tx.end_batch()).unwrap_err();
    match err {
        transport::BatchError::NoBatchInProgress => {
            // Success - we got the expected error
        }
        other => panic!("expected NoBatchInProgress error, got {:?}", other),
    }
}

#[test]
fn batch_transport_rpc_error() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"jsonrpc":"2.0","error":{"code":-1,"message":"batch error"},"id":0}]"#)
        .create();

    let inner_tx = Arc::new(Transport::new(server.url()));
    let batch_tx = BatchTransport::new(inner_tx);
    let rt = tokio::runtime::Runtime::new().unwrap();

    batch_tx.begin_batch();
    let _ = rt.block_on(batch_tx.send_request("foo", &[] as &[serde_json::Value]));

    let err = rt.block_on(batch_tx.end_batch()).unwrap_err();
    match err {
        transport::BatchError::Rpc(error_value) => {
            assert!(error_value.to_string().contains("batch error"));
        }
        other => panic!("expected Rpc error, got {:?}", other),
    }
}

#[test]
fn batch_transport_send_batch_delegation() {
    let mut server = Server::new();
    let _m = server
        .mock("POST", "/")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"jsonrpc":"2.0","result":"delegated","id":0}]"#)
        .create();

    let inner_tx = Arc::new(Transport::new(server.url()));
    let batch_tx = BatchTransport::new(inner_tx);
    let rt = tokio::runtime::Runtime::new().unwrap();

    let batch_requests = vec![json!({
        "jsonrpc": "2.0",
        "id": 0,
        "method": "foo",
        "params": []
    })];

    let results = rt.block_on(batch_tx.send_batch(&batch_requests)).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["result"], json!("delegated"));
}
