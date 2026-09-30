use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::handler_error::HandlerError;
use crate::handler_future::HandlerFuture;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

struct FnHandler<Responder, Extract> {
    extract: Extract,
    responder: Arc<Responder>,
}

#[async_trait]
impl<Responder, Extract> Handler for FnHandler<Responder, Extract>
where
    Responder: Send + Sync + 'static,
    Extract: for<'request> Fn(Arc<Responder>, &'request Request, RequestBody) -> HandlerFuture<'request>
        + Send
        + Sync
        + 'static,
{
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        (self.extract)(self.responder.clone(), request, body).await
    }
}

pub fn responder_handler<Responder, Extract>(
    responder: Arc<Responder>,
    extract: Extract,
) -> Arc<dyn Handler>
where
    Responder: Send + Sync + 'static,
    Extract: for<'request> Fn(Arc<Responder>, &'request Request, RequestBody) -> HandlerFuture<'request>
        + Send
        + Sync
        + 'static,
{
    Arc::new(FnHandler { extract, responder })
}
