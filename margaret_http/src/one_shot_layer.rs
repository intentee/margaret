use std::sync::Arc;

use async_trait::async_trait;

use crate::handler_error::HandlerError;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct OneShotLayer {
    inner: Box<dyn OneShotHandler>,
    middleware: Arc<dyn HttpMiddleware>,
}

#[async_trait]
impl OneShotHandler for OneShotLayer {
    async fn handle(
        self: Box<Self>,
        request: &Request,
    ) -> Result<ResponseContinuation, HandlerError> {
        let Self { inner, middleware } = *self;

        middleware.process(request, Next::one_shot(inner)).await
    }
}

pub(crate) fn one_shot_layer(
    middleware: Arc<dyn HttpMiddleware>,
    inner: Box<dyn OneShotHandler>,
) -> Box<dyn OneShotHandler> {
    Box::new(OneShotLayer { inner, middleware })
}
