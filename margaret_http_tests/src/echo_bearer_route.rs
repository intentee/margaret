use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::bearer_challenge::BearerChallenge;
use margaret_http::handler::Handler;
use margaret_http::handler_error::HandlerError;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request::Request;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http::route_entry::RouteEntry;

struct EchoBearer;

#[async_trait]
impl Handler for EchoBearer {
    async fn handle(&self, request: &Request) -> Result<ResponseContinuation, HandlerError> {
        Ok(ResponseContinuation::Done(
            match request.inputs.server.authorization() {
                RequestAuthorization::Bearer(token) => Response::text(200, token.as_str()),
                RequestAuthorization::Absent | RequestAuthorization::OtherScheme => {
                    BearerChallenge::MissingCredentials.response()
                }
                RequestAuthorization::Malformed => BearerChallenge::InvalidRequest.response(),
            },
        ))
    }
}

#[must_use]
pub fn echo_bearer_route() -> RouteEntry {
    RouteEntry::new(
        "/",
        vec![MethodHandler::anonymous("GET", Arc::new(EchoBearer))],
    )
}
