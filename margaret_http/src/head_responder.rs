use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::handler_future::HandlerFuture;
use crate::head_handler::HeadHandler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct HeadResponder<TResponder, TExtract> {
    extract: TExtract,
    responder: Arc<TResponder>,
}

#[async_trait]
impl<TResponder, TExtract> HeadHandler for HeadResponder<TResponder, TExtract>
where
    TResponder: Send + Sync + 'static,
    TExtract: for<'request> Fn(Arc<TResponder>, &'request Request) -> HandlerFuture<'request>
        + Send
        + Sync
        + 'static,
{
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        (self.extract)(Arc::clone(&self.responder), request).await
    }
}

pub fn head_responder<TResponder, TExtract>(
    responder: Arc<TResponder>,
    extract: TExtract,
) -> Arc<dyn HeadHandler>
where
    TResponder: Send + Sync + 'static,
    TExtract: for<'request> Fn(Arc<TResponder>, &'request Request) -> HandlerFuture<'request>
        + Send
        + Sync
        + 'static,
{
    Arc::new(HeadResponder { extract, responder })
}
