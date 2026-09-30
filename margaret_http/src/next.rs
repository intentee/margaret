use std::sync::Arc;

use crate::handler::Handler;
use crate::handler_error::HandlerError;
use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::response_continuation::ResponseContinuation;

enum NextHandler {
    OneShot(Box<dyn OneShotHandler>),
    Shared {
        body: RequestBody,
        handler: Arc<dyn Handler>,
    },
}

pub struct Next {
    handler: NextHandler,
}

impl Next {
    pub(crate) fn new(handler: Arc<dyn Handler>, body: RequestBody) -> Self {
        Self {
            handler: NextHandler::Shared { body, handler },
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
            NextHandler::OneShot(handler) => handler.handle(request).await,
            NextHandler::Shared { body, handler } => handler.handle(request, body).await,
        }
    }
}
