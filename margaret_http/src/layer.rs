use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::handler_error::HandlerError;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

struct LayeredHandler<Middleware: HttpMiddleware + ?Sized> {
    inner: Arc<dyn Handler>,
    middleware: Arc<Middleware>,
}

#[async_trait]
impl<Middleware> Handler for LayeredHandler<Middleware>
where
    Middleware: HttpMiddleware + Send + Sync + ?Sized + 'static,
{
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        self.middleware
            .process(request, Next::new(self.inner.clone(), body))
            .await
    }
}

pub fn layer<Middleware>(middleware: Arc<Middleware>, inner: Arc<dyn Handler>) -> Arc<dyn Handler>
where
    Middleware: HttpMiddleware + Send + Sync + ?Sized + 'static,
{
    Arc::new(LayeredHandler { inner, middleware })
}
