use std::sync::Arc;

use crate::forward_targets::ForwardTargets;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::respond_to_outcome::respond_to_outcome;
use crate::response::Response;
use crate::route_handler::RouteHandler;

pub(crate) async fn respond_recursively(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    body: RequestBody,
    first: RouteHandler,
) -> Response {
    let outcome = match first {
        RouteHandler::Content(handler) => handler.handle(&request, body).await,
        RouteHandler::Head(handler) => handler.handle(&request).await,
    };

    respond_to_outcome(forward_targets, request, outcome).await
}
