use std::collections::HashSet;
use std::sync::Arc;

use margaret_handler_error::handler_error::HandlerError;

use crate::cookie_changes::CookieChanges;
use crate::forward_targets::ForwardTargets;
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

                let carried = CookieChanges {
                    cookies: forward.carried_cookies().to_vec(),
                };

                request = request.with_path_params(forward.into_path_params());
                outcome = carried.precede(target.handle(&request).await?);
            }
            ResponseContinuation::Redirect(redirect) => return Ok(redirect.into_response()),
        }
    }
}
