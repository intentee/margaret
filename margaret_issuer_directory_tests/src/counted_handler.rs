use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use async_trait::async_trait;

use margaret_http::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

pub struct CountedHandler {
    pub handled: Arc<AtomicUsize>,
    pub inner: Arc<dyn HeadHandler>,
}

#[async_trait]
impl HeadHandler for CountedHandler {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        self.handled.fetch_add(1, Ordering::SeqCst);
        self.inner.handle(request).await
    }
}
