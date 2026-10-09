use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::content_handler::ContentHandler;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

pub(crate) struct LayeredContentHandler<TMiddleware: HttpMiddleware + ?Sized> {
    pub(crate) inner: Arc<dyn ContentHandler>,
    pub(crate) middleware: Arc<TMiddleware>,
}

#[async_trait]
impl<TMiddleware> ContentHandler for LayeredContentHandler<TMiddleware>
where
    TMiddleware: HttpMiddleware + Send + Sync + ?Sized + 'static,
{
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        self.middleware
            .process(request, Next::content(Arc::clone(&self.inner), body))
            .await
    }
}
