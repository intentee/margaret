use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_http::response::Response;

use crate::oauth_error::oauth_error;

fn invalid_target(description: &'static str) -> ControlFlow<Response, &'static str> {
    ControlFlow::Break(oauth_error(
        400,
        BasicErrorResponseType::Extension("invalid_target".to_string()),
        description,
    ))
}

pub(crate) fn target_resource(
    resources: &'static [&'static str],
    requested: Option<&str>,
) -> ControlFlow<Response, &'static str> {
    match requested {
        Some(requested) => match resources.iter().find(|resource| **resource == requested) {
            Some(resource) => ControlFlow::Continue(resource),
            None => invalid_target("the client may not request the resource"),
        },
        None => match resources {
            [resource] => ControlFlow::Continue(resource),
            _ => invalid_target("the client serves several resources and must name one"),
        },
    }
}
