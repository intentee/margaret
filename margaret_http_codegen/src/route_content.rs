use std::num::NonZeroU64;

use margaret_request_binding_codegen::content_binding::ContentBinding;
use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

pub(crate) enum RouteContent {
    Read {
        binding: ContentBinding,
        limit: NonZeroU64,
        method: ContentMethod,
    },
    Unread {
        method: RouteMethod,
    },
}

impl RouteContent {
    pub(crate) fn method(&self) -> RouteMethod {
        match self {
            Self::Read { method, .. } => method.route_method(),
            Self::Unread { method } => *method,
        }
    }
}
