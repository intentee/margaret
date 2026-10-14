use std::ops::ControlFlow;
use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_route_method::route_method::RouteMethod;

struct EchoBearer;

#[async_trait]
impl<TTarget> HeadHandler<TTarget> for EchoBearer {
    async fn handle(
        &self,
        request: &Request,
    ) -> Result<ResponseContinuation<TTarget>, HandlerError> {
        Ok(ResponseContinuation::Done(
            match request.inputs.server.authorization().bearer() {
                ControlFlow::Continue(token) => Response::text(200, token.as_str()),
                ControlFlow::Break(challenge) => challenge.response(),
            },
        ))
    }
}

#[must_use]
pub fn echo_bearer_route<TTarget: Send + 'static>() -> RouteEntry<TTarget> {
    RouteEntry::new(vec![MethodHandler::head(
        RouteMethod::Get,
        Arc::new(EchoBearer),
    )])
}
