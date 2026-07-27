use std::sync::Arc;

use crate::handler::Handler;
use crate::handler_error::HandlerError;
use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

enum NextHandler {
    OneShot(Box<dyn OneShotHandler>),
    Shared(Arc<dyn Handler>),
}

pub struct Next {
    handler: NextHandler,
}

impl Next {
    pub(crate) fn new(inner: Arc<dyn Handler>) -> Self {
        Self {
            handler: NextHandler::Shared(inner),
        }
    }

    pub(crate) fn one_shot(inner: Box<dyn OneShotHandler>) -> Self {
        Self {
            handler: NextHandler::OneShot(inner),
        }
    }

    pub async fn run(self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        match self.handler {
            NextHandler::OneShot(handler) => handler.handle(request).await,
            NextHandler::Shared(handler) => handler.handle(request).await,
        }
    }
}
