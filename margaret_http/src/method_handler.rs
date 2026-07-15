use std::sync::Arc;

use crate::handler::Handler;

pub struct MethodHandler {
    pub handler: Arc<dyn Handler>,
    pub method: &'static str,
}

impl MethodHandler {
    #[must_use]
    pub fn new(method: &'static str, handler: Arc<dyn Handler>) -> Self {
        Self { handler, method }
    }
}
