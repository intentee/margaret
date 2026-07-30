use std::sync::Arc;

use crate::handler::Handler;
use crate::handler_name::HandlerName;

pub struct MethodHandler {
    pub handler: Arc<dyn Handler>,
    pub method: &'static str,
    pub name: HandlerName,
}

impl MethodHandler {
    #[must_use]
    pub fn anonymous(method: &'static str, handler: Arc<dyn Handler>) -> Self {
        Self {
            handler,
            method,
            name: HandlerName::Anonymous,
        }
    }

    #[must_use]
    pub fn named(method: &'static str, name: &'static str, handler: Arc<dyn Handler>) -> Self {
        Self {
            handler,
            method,
            name: HandlerName::Named(name),
        }
    }
}
