use std::ops::ControlFlow;

use margaret_http::response::Response;

use crate::invalid_target::invalid_target;
use crate::resource_target::ResourceTarget;

pub(crate) fn target_resource(
    resources: &'static [&'static str],
    requested: Option<&str>,
) -> ControlFlow<Response, ResourceTarget> {
    match requested {
        Some(requested) => match resources.iter().find(|resource| **resource == requested) {
            Some(resource) => ControlFlow::Continue(ResourceTarget::Named(resource)),
            None => invalid_target("the client may not request the resource"),
        },
        None => match resources {
            [] => ControlFlow::Continue(ResourceTarget::Unnamed),
            [resource] => ControlFlow::Continue(ResourceTarget::Named(resource)),
            [_, _, ..] => invalid_target("the client serves several resources and must name one"),
        },
    }
}
