use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::content_handler::ContentHandler;
use crate::handler_future::HandlerFuture;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

struct ContentResponder<TResponder, TExtract> {
    extract: TExtract,
    responder: Arc<TResponder>,
}

#[async_trait]
impl<TTarget, TResponder, TExtract> ContentHandler<TTarget>
    for ContentResponder<TResponder, TExtract>
where
    TTarget: 'static,
    TResponder: Send + Sync + 'static,
    TExtract: for<'request> Fn(
            Arc<TResponder>,
            &'request Request,
            RequestBody,
        ) -> HandlerFuture<'request, TTarget>
        + Send
        + Sync
        + 'static,
{
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError> {
        (self.extract)(Arc::clone(&self.responder), request, body).await
    }
}

pub fn content_responder<TTarget, TResponder, TExtract>(
    responder: Arc<TResponder>,
    extract: TExtract,
) -> Arc<dyn ContentHandler<TTarget>>
where
    TTarget: 'static,
    TResponder: Send + Sync + 'static,
    TExtract: for<'request> Fn(
            Arc<TResponder>,
            &'request Request,
            RequestBody,
        ) -> HandlerFuture<'request, TTarget>
        + Send
        + Sync
        + 'static,
{
    Arc::new(ContentResponder { extract, responder })
}
