// transport/src/batch_transport.rs

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use serde_json::{json, Value};
use thiserror::Error;

use super::{TransportError, TransportTrait};

/// Errors that can occur during batch operations
#[derive(Debug, Error)]
pub enum BatchError {
    /// Error from the underlying transport
    #[error("Transport error: {0}")]
    Transport(#[from] crate::TransportError),

    /// Error parsing batch response
    #[error("Invalid batch response: {0}")]
    InvalidResponse(String),

    /// Error when trying to end a batch that hasn't been started
    #[error("No batch in progress")]
    NoBatchInProgress,

    /// Error from a specific RPC call in the batch
    #[error("RPC error in batch: {0}")]
    Rpc(Value),
}

/// A transport wrapper that supports batching multiple RPC calls into a single request.
///
/// # Usage
/// 1. Call [`begin_batch`] to start collecting requests.
/// 2. Use [`send_request`] to queue each RPC call (these will not be sent immediately).
/// 3. Call [`end_batch`] to send all queued requests as a single JSON-RPC batch and receive results.
///
/// This struct is thread-safe and can be shared between threads. Each batch is associated with a unique set of request IDs.
pub struct BatchTransport {
    inner: Arc<dyn TransportTrait>,
    batch: Arc<Mutex<Option<Vec<BatchRequest>>>>,
    /// Used to generate unique IDs for each request in a batch.
    next_id: AtomicU64,
}

/// Represents a single RPC request that has been queued for batch processing.
///
/// This struct holds the method name, parameters, and a unique identifier for each
/// request that will be sent as part of a JSON-RPC batch. The `id` field ensures
/// that responses can be matched back to their corresponding requests when the
/// batch is executed.
pub struct BatchRequest {
    method: String,
    params: Vec<Value>,
    id: usize,
}

impl BatchTransport {
    /// Create a new batch transport that wraps the given transport
    pub fn new(inner: Arc<dyn TransportTrait>) -> Self {
        Self { inner, batch: Arc::new(Mutex::new(None)), next_id: AtomicU64::new(0) }
    }

    /// Begin collecting requests into a batch.
    ///
    /// Any subsequent calls to [`send_request`] will be queued until [`end_batch`] is called.
    /// If a batch is already in progress, it will be replaced (and any queued requests will be lost).
    pub fn begin_batch(&self) {
        let mut batch = self.batch.lock().unwrap();
        *batch = Some(Vec::new());
    }

    /// End the current batch and send all collected requests as a single JSON-RPC batch.
    ///
    /// Returns a vector of results in the same order as the requests were queued.
    /// If any request in the batch fails, the entire batch fails.
    ///
    /// # Errors
    /// - Returns [`BatchError::NoBatchInProgress`] if no batch was started.
    /// - Returns [`BatchError::Transport`] if the underlying transport fails.
    /// - Returns [`BatchError::Rpc`] if any RPC call in the batch returns an error.
    pub async fn end_batch(&self) -> Result<Vec<Value>, BatchError> {
        // 1) Take the queued calls
        let requests = {
            let mut b = self.batch.lock().unwrap();
            b.take().ok_or(BatchError::NoBatchInProgress)?
        };
        if requests.is_empty() {
            return Ok(vec![]);
        }

        // 2) Build the JSON-RPC batch array
        let batch_json: Vec<Value> = requests
            .iter()
            .map(|req| {
                json!({
                    "jsonrpc": "2.0",
                    "id": req.id,
                    "method": req.method,
                    "params": req.params,
                })
            })
            .collect();

        // 3) Fire the HTTP request (this re-uses your auth'd DefaultTransport behind the scenes,
        //    so you don't need to think about headers or basic_auth here)
        let resp = self.inner.send_batch(&batch_json).await.map_err(BatchError::Transport)?;

        // 4) Parse the array of responses
        let arr: Vec<Value> = resp;

        // 5) Extract each "result" or bail on the first error
        let mut results = Vec::with_capacity(arr.len());
        for obj in arr {
            if let Some(err) = obj.get("error") {
                return Err(BatchError::Rpc(err.clone()));
            }
            // assume "result" is present
            results.push(obj.get("result").cloned().unwrap_or(Value::Null));
        }
        Ok(results)
    }

    /// Check if a batch is currently in progress.
    pub fn is_batching(&self) -> bool { self.batch.lock().unwrap().is_some() }
}

impl TransportTrait for BatchTransport {
    /// Queue a request if batching, or send immediately if not batching.
    ///
    /// # Returns
    /// - If batching, always returns an error future, since results are only available after [`end_batch`].
    /// - If not batching, delegates to the inner transport and returns the result.
    fn send_request<'a>(
        &'a self,
        method: &'a str,
        params: &'a [Value],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Value, crate::TransportError>> + Send + 'a>,
    > {
        let mut batch = self.batch.lock().unwrap();

        // If we're not in a batch, send immediately
        if batch.is_none() {
            drop(batch);
            return Box::pin(self.inner.send_request(method, params));
        }

        // Add to batch without channel
        let id = self.next_id.fetch_add(1, Ordering::SeqCst) as usize;
        batch.as_mut().unwrap().push(BatchRequest {
            method: method.to_string(),
            params: params.to_vec(),
            id,
        });

        // Return a future that immediately returns an error since we can't wait for the batch
        // This is by design: end_batch() must be called to get results for all queued requests.
        Box::pin(async move {
            Err(TransportError::Rpc(
                "Cannot wait for individual request result in batch mode. Use end_batch() to get all results.".to_string()
            ))
        })
    }

    fn send_batch<'a>(
        &'a self,
        bodies: &'a [Value],
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Vec<Value>, crate::TransportError>> + Send + 'a,
        >,
    > {
        // Delegate to the inner transport's send_batch method
        Box::pin(self.inner.send_batch(bodies))
    }

    fn url(&self) -> &str { self.inner.url() }
}
