use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;

use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct OneShotLayer<TTarget> {
    inner: Box<dyn OneShotHandler<TTarget>>,
    middleware: Arc<dyn HttpMiddleware<TTarget>>,
}

#[async_trait]
impl<TTarget: Send + 'static> OneShotHandler<TTarget> for OneShotLayer<TTarget> {
    async fn handle(
        self: Box<Self>,
        request: &Request,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError> {
        let Self { inner, middleware } = *self;

        middleware.process(request, Next::one_shot(inner)).await
    }
}

pub(crate) fn one_shot_layer<TTarget: Send + 'static>(
    middleware: Arc<dyn HttpMiddleware<TTarget>>,
    inner: Box<dyn OneShotHandler<TTarget>>,
) -> Box<dyn OneShotHandler<TTarget>> {
    Box::new(OneShotLayer { inner, middleware })
}
