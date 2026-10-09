use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::head_handler::HeadHandler;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

pub(crate) struct LayeredHeadHandler<TMiddleware: HttpMiddleware + ?Sized> {
    pub(crate) inner: Arc<dyn HeadHandler>,
    pub(crate) middleware: Arc<TMiddleware>,
}

#[async_trait]
impl<TMiddleware> HeadHandler for LayeredHeadHandler<TMiddleware>
where
    TMiddleware: HttpMiddleware + Send + Sync + ?Sized + 'static,
{
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        self.middleware
            .process(request, Next::head(Arc::clone(&self.inner)))
            .await
    }
}
