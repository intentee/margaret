use std::future::Future;

use crate::request::Request;
use crate::response::Response;
use crate::response_continuation::ResponseContinuation;

pub async fn with_route_parameters<const COUNT: usize, TOutput, TContinuation, TFuture>(
    request: &Request,
    names: [&'static str; COUNT],
    refused: fn(ResponseContinuation) -> TOutput,
    continuation: TContinuation,
) -> TOutput
where
    TContinuation: FnOnce([String; COUNT]) -> TFuture,
    TFuture: Future<Output = TOutput>,
{
    match <[String; COUNT]>::try_from(
        names
            .iter()
            .filter_map(|name| request.path_param(name))
            .map(str::to_string)
            .collect::<Vec<String>>(),
    ) {
        Ok(parameters) => continuation(parameters).await,
        Err(_present) => refused(ResponseContinuation::from(Response::not_found())),
    }
}
