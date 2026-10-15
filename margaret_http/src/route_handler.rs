use std::sync::Arc;

use margaret_handler_error::handler_error::HandlerError;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

#[derive(Clone)]
pub(crate) enum RouteHandler {
    Content(Arc<dyn ContentHandler>),
    Head(Arc<dyn HeadHandler>),
}

impl RouteHandler {
    pub(crate) async fn handle(
        &self,
        request: &Request,
        body: RequestBody,
    ) -> Result<ResponseContinuation, HandlerError> {
        match self {
            Self::Content(handler) => handler.handle(request, body).await,
            Self::Head(handler) => handler.handle(request).await,
        }
    }
}
