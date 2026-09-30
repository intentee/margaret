use std::ops::ControlFlow;

use oauth2::basic::BasicErrorResponseType;

use margaret_accepted_clients::accepted_client::AcceptedClient;
use margaret_http::response::Response;
use margaret_registered_claims::audience::Audience;

use crate::oauth_error::oauth_error;

fn invalid_target(description: &str) -> ControlFlow<Response, Audience> {
    ControlFlow::Break(oauth_error(
        400,
        BasicErrorResponseType::Extension("invalid_target".to_string()),
        description,
    ))
}

pub(crate) fn target_resource(
    client: &AcceptedClient,
    requested: Option<&str>,
) -> ControlFlow<Response, Audience> {
    match requested {
        Some(requested) => match requested.parse::<Audience>() {
            Ok(resource) if client.resources.contains(&resource) => ControlFlow::Continue(resource),
            Ok(_) | Err(_) => invalid_target("the client may not request the resource"),
        },
        None => match client.resources.first() {
            Some(resource) if client.resources.len() == 1 => {
                ControlFlow::Continue(resource.clone())
            }
            Some(_) | None => {
                invalid_target("the client serves several resources and must name one")
            }
        },
    }
}
