use crate::forward_scope::ForwardScope;
use crate::request::Request;
use crate::request_body::RequestBody;
use crate::respond_to_outcome::respond_to_outcome;
use crate::response::Response;
use crate::route_handler::RouteHandler;

pub(crate) async fn respond_recursively<TTarget: ForwardScope>(
    targets: &TTarget::Targets,
    request: Request,
    body: RequestBody,
    first: RouteHandler<TTarget>,
) -> Response {
    let entered = first.entered();
    let outcome = match first {
        RouteHandler::Content(handler) => handler.handle(&request, body).await,
        RouteHandler::Head(handler) => handler.handle(&request).await,
    };

    respond_to_outcome(targets, entered, request, outcome).await
}
