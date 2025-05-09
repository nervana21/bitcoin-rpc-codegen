// logging/src/lib.rs

//! A small helper to initialize tracing for the pipeline.
//!
//! Usage:
//! ```rust,ignore
//! logging::init();
//! tracing::info!("pipeline started");
//! tracing::debug!("detailed state: {:?}", some_struct);
//! ```

use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the global tracing subscriber:
/// - Reads `RUST_LOG` for filter directives, falling back to `"pipeline=info"`.
/// - Uses a pretty-printed, line-based formatter.
pub fn init() {
    // Try to parse RUST_LOG; default to showing info+ on our crate
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("pipeline=info"));

    fmt()
        .with_env_filter(filter)
        // You can tweak formatting here (timestamps, targets, etc.)
        .init();
}
