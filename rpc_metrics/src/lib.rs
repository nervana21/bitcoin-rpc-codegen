// rpc_metrics/src/lib.rs

//! A simple metrics crate for the Bitcoin RPC pipeline.
//!
//! Provides initialization of a Prometheus metrics endpoint and
//! helper functions/macros for recording counters, gauges, and histograms.

use anyhow::Result;
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::{borrow::Cow, net::SocketAddr, sync::Once};

static START: Once = Once::new();
static mut HANDLE: Option<PrometheusHandle> = None;

/// Initialize the Prometheus recorder and HTTP endpoint.
/// This function is idempotent; subsequent calls do nothing.
///
/// # Arguments
///
/// * `addr` - String or socket address (e.g. "0.0.0.0:9000").
///
/// # Errors
///
/// Returns an error if parsing fails or recorder fails to install.
pub fn init(addr: &str) -> Result<()> {
    let mut result = Ok(());
    START.call_once(|| {
        let socket_addr = match addr.parse::<SocketAddr>() {
            Ok(sa) => sa,
            Err(e) => {
                result = Err(anyhow::anyhow!(e));
                return;
            }
        };
        let builder = PrometheusBuilder::new().with_http_listener(socket_addr);
        match builder.install_recorder() {
            Ok(handle) => unsafe { HANDLE = Some(handle) },
            Err(e) => result = Err(anyhow::anyhow!(e)),
        }
    });
    result
}

/// Records an increment to a counter metric.
///
/// # Example
///
/// ```rust
/// rpc_metrics::record_counter("rpc_requests_total", 1);
/// ```
pub fn record_counter(name: &str, value: u64) {
    counter!(Cow::Owned(name.to_string())).increment(value);
}

/// Records an absolute value to a gauge metric.
///
/// # Example
///
/// ```rust
/// rpc_metrics::record_gauge("rpc_queue_length", 5.0);
/// ```
pub fn record_gauge(name: &str, value: f64) {
    gauge!(Cow::Owned(name.to_string())).set(value);
}

/// Records a value in a histogram metric.
///
/// # Example
///
/// ```rust
/// rpc_metrics::record_histogram("rpc_latency_seconds", 0.123);
/// ```
pub fn record_histogram(name: &str, value: f64) {
    histogram!(Cow::Owned(name.to_string())).record(value);
}

/// Retrieve the Prometheus metric exposition body.
/// Useful for testing or embedding.
pub fn gather() -> Result<String> {
    unsafe {
        if let Some(handle) = &HANDLE {
            Ok(handle.render())
        } else {
            Err(anyhow::anyhow!("Metrics recorder not initialized"))
        }
    }
}

// -----------------------------------------------------------------------------
// Test-support: allow resetting state without exporting the static directly.
// -----------------------------------------------------------------------------
#[doc(hidden)]
pub fn clear_handle() {
    unsafe {
        HANDLE = None;
    }
}
