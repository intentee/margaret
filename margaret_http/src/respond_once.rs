use std::sync::Arc;

use crate::forward_targets::ForwardTargets;
use crate::one_shot_handler::OneShotHandler;
use crate::request::Request;
use crate::respond_to_outcome::respond_to_outcome;
use crate::response::Response;

pub(crate) async fn respond_once(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    first: Box<dyn OneShotHandler>,
) -> Response {
    let outcome = first.handle(&request).await;

    respond_to_outcome(forward_targets, request, outcome).await
}
