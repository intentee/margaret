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
impl<TResponder, TExtract> ContentHandler for ContentResponder<TResponder, TExtract>
where
    TResponder: Send + Sync + 'static,
    TExtract: for<'request> Fn(Arc<TResponder>, &'request Request, RequestBody) -> HandlerFuture<'request>
        + Send
        + Sync
        + 'static,
{
    async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        (self.extract)(Arc::clone(&self.responder), request, body).await
    }
}

pub fn content_responder<TResponder, TExtract>(
    responder: Arc<TResponder>,
    extract: TExtract,
) -> Arc<dyn ContentHandler>
where
    TResponder: Send + Sync + 'static,
    TExtract: for<'request> Fn(Arc<TResponder>, &'request Request, RequestBody) -> HandlerFuture<'request>
        + Send
        + Sync
        + 'static,
{
    Arc::new(ContentResponder { extract, responder })
}
