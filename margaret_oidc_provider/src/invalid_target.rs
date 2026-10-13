use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_http::response::Response;

use crate::oauth_error::oauth_error;

pub(crate) fn invalid_target<Targeted>(
    description: &'static str,
) -> ControlFlow<Response, Targeted> {
    ControlFlow::Break(oauth_error(
        400,
        BasicErrorResponseType::Extension("invalid_target".to_string()),
        description,
    ))
}
