use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::head_handler::HeadHandler;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

pub(crate) struct LayeredHeadHandler<TTarget, TMiddleware: HttpMiddleware<TTarget> + ?Sized> {
    pub(crate) inner: Arc<dyn HeadHandler<TTarget>>,
    pub(crate) middleware: Arc<TMiddleware>,
}

#[async_trait]
impl<TTarget, TMiddleware> HeadHandler<TTarget> for LayeredHeadHandler<TTarget, TMiddleware>
where
    TTarget: Send + 'static,
    TMiddleware: HttpMiddleware<TTarget> + Send + Sync + ?Sized + 'static,
{
    async fn handle(
        &self,
        request: &Request,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError> {
        self.middleware
            .process(request, Next::head(Arc::clone(&self.inner)))
            .await
    }
}
