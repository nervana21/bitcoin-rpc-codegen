// logging/tests/basic.rs

use logging;
use tracing;

/// A very basic integration test for the `pipeline_logging` crate.
///
/// This simply verifies that:
/// 1) `init()` can be called without panicking
/// 2) after initialization, the `tracing` macros work as expected
#[test]
fn init_and_emit_logs() {
    // Initialize the global tracing subscriber
    logging::init();

    // These calls should not panic.
    // The info-level message will be emitted by default.
    tracing::info!("basic test: info-level log");

    // The debug-level message may be filtered out by default,
    // but the macro invocation itself should not panic.
    tracing::debug!("basic test: debug-level log");
}
