use std::sync::Arc;

use crate::content_handler::ContentHandler;
use crate::handler_error::HandlerError;
use crate::head_handler::HeadHandler;
use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

enum NextHandler {
    Content {
        body: RequestBody,
        handler: Arc<dyn ContentHandler>,
    },
    Head(Arc<dyn HeadHandler>),
    OneShot(Box<dyn OneShotHandler>),
}

pub struct Next {
    handler: NextHandler,
}

impl Next {
    pub(crate) fn content(handler: Arc<dyn ContentHandler>, body: RequestBody) -> Self {
        Self {
            handler: NextHandler::Content { body, handler },
        }
    }

    pub(crate) fn head(handler: Arc<dyn HeadHandler>) -> Self {
        Self {
            handler: NextHandler::Head(handler),
        }
    }

    pub(crate) fn one_shot(inner: Box<dyn OneShotHandler>) -> Self {
        Self {
            handler: NextHandler::OneShot(inner),
        }
    }

    /// # Errors
    ///
    /// Returns `HandlerError` propagated from the work it performs.
    pub async fn run(self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        match self.handler {
            NextHandler::Content { body, handler } => handler.handle(request, body).await,
            NextHandler::Head(handler) => handler.handle(request).await,
            NextHandler::OneShot(handler) => handler.handle(request).await,
        }
    }
}
