use std::sync::Arc;

use http::Method;

use crate::handler::Handler;

pub struct MethodHandler {
    pub handler: Arc<dyn Handler>,
    pub method: Method,
}

impl MethodHandler {
    #[must_use]
    pub fn new(method: Method, handler: Arc<dyn Handler>) -> Self {
        Self { handler, method }
    }
}
