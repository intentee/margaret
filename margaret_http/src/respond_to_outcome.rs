use std::sync::Arc;

use margaret_handler_error::handler_error::HandlerError;

use crate::forward_targets::ForwardTargets;
use crate::request::Request;
use crate::resolve_continuation::resolve_continuation;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub(crate) async fn respond_to_outcome(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    outcome: Result<ResponseContinuation, HandlerError>,
) -> Response {
    let outcome = match outcome {
        Ok(outcome) => resolve_continuation(forward_targets, request, outcome).await,
        Err(error) => Err(error),
    };

    match outcome {
        Ok(response) => response,
        Err(error) => {
            eprintln!("margaret_http: request handling failed: {error:#}");

            Response::text(500, "Internal Server Error")
        }
    }
}
