// rpc_metrics/tests/basic.rs

use rpc_metrics;

/// Basic integration tests for the rpc_metrics crate.
///
/// Verifies that:
/// 1) init() can be called without panicking
/// 2) metrics recording functions work as expected
/// 3) gather() returns expected results
#[test]
fn init_and_record_metrics() {
    // Initialize the metrics recorder with a test port
    rpc_metrics::init("127.0.0.1:0").expect("metrics init failed");

    // Record some test metrics
    rpc_metrics::record_counter("test_counter", 1);
    rpc_metrics::record_gauge("test_gauge", 42.0);
    rpc_metrics::record_histogram("test_histogram", 0.123);

    // Gather metrics and verify they contain our test metrics
    let metrics = rpc_metrics::gather().expect("gather failed");
    assert!(metrics.contains("test_counter"));
    assert!(metrics.contains("test_gauge"));
    assert!(metrics.contains("test_histogram"));
}

#[test]
fn init_is_idempotent() {
    // Ensure clean state
    rpc_metrics::clear_handle();

    // First initialization
    rpc_metrics::init("127.0.0.1:0").expect("first init failed");

    // Second initialization should not fail
    rpc_metrics::init("127.0.0.1:0").expect("second init failed");

    // Record and verify metrics still work
    rpc_metrics::record_counter("idempotent_test", 1);
    let metrics = rpc_metrics::gather().expect("gather failed");
    assert!(metrics.contains("idempotent_test"));
}

#[test]
fn gather_fails_when_not_initialized() {
    // Ensure no recorder is active
    rpc_metrics::clear_handle();

    // Attempting to gather should fail
    assert!(rpc_metrics::gather().is_err());
}
