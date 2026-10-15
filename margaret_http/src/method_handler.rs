use std::sync::Arc;

use margaret_route_method::content_method::ContentMethod;
use margaret_route_method::route_method::RouteMethod;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;
use crate::route_handler::RouteHandler;

pub struct MethodHandler {
    pub(crate) handler: RouteHandler,
    pub(crate) method: RouteMethod,
}

impl MethodHandler {
    #[must_use]
    pub fn content(method: ContentMethod, handler: Arc<dyn ContentHandler>) -> Self {
        Self {
            handler: RouteHandler::Content(handler),
            method: method.route_method(),
        }
    }

    #[must_use]
    pub fn head(method: RouteMethod, handler: Arc<dyn HeadHandler>) -> Self {
        Self {
            handler: RouteHandler::Head(handler),
            method,
        }
    }
}
