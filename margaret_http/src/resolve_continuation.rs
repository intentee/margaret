use std::collections::HashSet;
use std::sync::Arc;

use crate::forward_targets::ForwardTargets;
use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub(crate) async fn resolve_continuation(
    forward_targets: &Arc<ForwardTargets>,
    request: Request,
    first_outcome: ResponseContinuation,
) -> Response {
    let mut visited: HashSet<&'static str> = HashSet::new();
    let mut request = request;
    let mut outcome = first_outcome;

    loop {
        match outcome {
            ResponseContinuation::Done(response) => return response,
            ResponseContinuation::Forward(forward) => {
                let name = forward.name();

                if !visited.insert(name) {
                    eprintln!("margaret_http: forward cycle re-entered the responder `{name}`");

                    return Response::text(500, "Internal Server Error");
                }

                let Some(target) = forward_targets.resolve(name) else {
                    eprintln!(
                        "margaret_http: no forward target is registered for `{name}` on this server"
                    );

                    return Response::text(500, "Internal Server Error");
                };

                request = request.with_path_params(forward.into_path_params());
                outcome = target.handle(&request).await;
            }
            ResponseContinuation::Redirect(redirect) => return redirect.into_response(),
        }
    }
}
