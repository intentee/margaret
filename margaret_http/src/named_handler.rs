use std::sync::Arc;

use crate::handler::Handler;

pub struct NamedHandler {
    handler: Arc<dyn Handler>,
    name: &'static str,
}

impl NamedHandler {
    pub fn new(name: &'static str, handler: Arc<dyn Handler>) -> Self {
        Self { handler, name }
    }

    pub(crate) fn into_handler(self) -> Arc<dyn Handler> {
        self.handler
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }
}
