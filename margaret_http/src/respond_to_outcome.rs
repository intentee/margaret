use margaret_handler_error::handler_error::HandlerError;

use crate::entered_responders::EnteredResponders;
use crate::forward_scope::ForwardScope;
use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub(crate) async fn respond_to_outcome<TTarget: ForwardScope>(
    targets: &TTarget::Targets,
    entered: EnteredResponders,
    request: Request,
    outcome: Result<ResponseContinuation<TTarget>, HandlerError>,
) -> Response {
    let outcome = match outcome {
        Ok(outcome) => TTarget::conclude(targets, entered, request, outcome).await,
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
