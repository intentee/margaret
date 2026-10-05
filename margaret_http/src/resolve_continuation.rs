use std::collections::HashSet;
use std::sync::Arc;

use crate::forward_targets::ForwardTargets;
use crate::handler_error::HandlerError;
use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub(crate) async fn resolve_continuation(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    first_outcome: ResponseContinuation,
) -> Result<Response, HandlerError> {
    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut request = request;
    let mut outcome = first_outcome;

    loop {
        match outcome {
            ResponseContinuation::Done(response) => return Ok(response),
            ResponseContinuation::Forward(forward) => {
                let name = forward.name();

                if !visited.insert(name) {
                    return Err(HandlerError::ForwardCycle { responder: name });
                }

                let Some(target) = forward_targets.resolve(name) else {
                    return Err(HandlerError::UnknownForwardTarget { responder: name });
                };

                request = request.with_path_params(forward.into_path_params());
                outcome = target.handle(&request).await?;
            }
            ResponseContinuation::Redirect(redirect) => return Ok(redirect.into_response()),
        }
    }
}
