use std::sync::Arc;

use async_trait::async_trait;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;
use margaret_http::with_verified_peer::with_verified_peer;
use margaret_route_method::route_method::RouteMethod;

struct EchoPeer;

#[async_trait]
impl HeadHandler for EchoPeer {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        with_verified_peer(request, Ok, |spiffe_id| async move {
            Ok(ResponseContinuation::Done(Response::text(
                200,
                format!("{}{}", spiffe_id.trust_domain(), spiffe_id.path()),
            )))
        })
        .await
    }
}

#[must_use]
pub fn echo_peer_route() -> RouteEntry {
    RouteEntry::new(vec![MethodHandler::head(
        RouteMethod::Get,
        Arc::new(EchoPeer),
    )])
}
