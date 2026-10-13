use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_http::response::Response;

use crate::oauth_error::oauth_error;

pub(crate) fn requested_target(
    audience: Option<String>,
    resource: Option<String>,
) -> ControlFlow<Response, Option<String>> {
    match (audience, resource) {
        (Some(_), Some(_)) => ControlFlow::Break(oauth_error(
            400,
            BasicErrorResponseType::InvalidRequest,
            "the token request names both an audience and a resource",
        )),
        (Some(named), None) | (None, Some(named)) => ControlFlow::Continue(Some(named)),
        (None, None) => ControlFlow::Continue(None),
    }
}
