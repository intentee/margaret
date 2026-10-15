use crate::request::Request;
use crate::request_body::RequestBody;
use crate::respond_to_outcome::respond_to_outcome;
use crate::response::Response;
use crate::route_handler::RouteHandler;

pub(crate) async fn respond_with_route_handler(
    request: &Request,
    body: RequestBody,
    handler: RouteHandler,
) -> Response {
    respond_to_outcome(match handler {
        RouteHandler::Content(handler) => handler.handle(request, body).await,
        RouteHandler::Head(handler) => handler.handle(request).await,
    })
}
