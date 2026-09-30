use std::sync::Arc;

use margaret_route_method::route_method::RouteMethod;

use crate::handler::Handler;
use crate::handler_name::HandlerName;

pub struct MethodHandler {
    pub handler: Arc<dyn Handler>,
    pub method: RouteMethod,
    pub name: HandlerName,
}

impl MethodHandler {
    #[must_use]
    pub fn anonymous(method: RouteMethod, handler: Arc<dyn Handler>) -> Self {
        Self {
            handler,
            method,
            name: HandlerName::Anonymous,
        }
    }

    #[must_use]
    pub fn forwardable(name: &'static str, handler: Arc<dyn Handler>) -> Self {
        Self {
            handler,
            method: RouteMethod::Get,
            name: HandlerName::Forwardable(name),
        }
    }
}
