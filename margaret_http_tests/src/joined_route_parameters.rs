use std::convert::Infallible;
use std::future::Ready;
use std::future::ready;

use margaret_handler_error::handler_error::HandlerError;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;

pub fn joined_route_parameters<const COUNT: usize>(
    parameters: [String; COUNT],
) -> Ready<Result<ResponseContinuation<Infallible>, HandlerError>> {
    ready(Ok(ResponseContinuation::from(Response::text(
        200,
        Vec::from(parameters).join("/"),
    ))))
}
