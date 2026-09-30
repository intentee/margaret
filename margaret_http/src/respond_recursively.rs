use std::sync::Arc;

use crate::forward_targets::ForwardTargets;
use crate::handler::Handler;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::respond_to_outcome::respond_to_outcome;
use crate::response::Response;

pub(crate) async fn respond_recursively(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    body: RequestBody,
    first: Arc<dyn Handler>,
) -> Response {
    let outcome = first.handle(&request, body).await;

    respond_to_outcome(forward_targets, request, outcome).await
}
