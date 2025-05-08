use client::RpcClient;
use mockito::Server;
use serde_json::json;

#[test]
fn rpcclient_call_method() {
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
