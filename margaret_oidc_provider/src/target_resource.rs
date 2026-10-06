use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_accepted_clients::non_empty_set::NonEmptySet;
use margaret_http::response::Response;
use margaret_registered_claims::audience::Audience;

use crate::oauth_error::oauth_error;

fn invalid_target(description: &'static str) -> ControlFlow<Response, Audience> {
    ControlFlow::Break(oauth_error(
        400,
        BasicErrorResponseType::Extension("invalid_target".to_string()),
        description,
    ))
}

pub(crate) fn target_resource(
    resources: &NonEmptySet<Audience>,
    requested: Option<&str>,
) -> ControlFlow<Response, Audience> {
    match requested {
        Some(requested) => match requested.parse::<Audience>() {
            Ok(resource) if resources.members().contains(&resource) => {
                ControlFlow::Continue(resource)
            }
            Ok(_) | Err(_) => invalid_target("the client may not request the resource"),
        },
        None => match resources.only() {
            Some(resource) => ControlFlow::Continue(resource.clone()),
            None => invalid_target("the client serves several resources and must name one"),
        },
    }
}
