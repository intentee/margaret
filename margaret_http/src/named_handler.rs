use std::sync::Arc;

use crate::head_handler::HeadHandler;

pub struct NamedHandler {
    handler: Arc<dyn HeadHandler>,
    name: &'static str,
}

impl NamedHandler {
    pub(crate) fn new(name: &'static str, handler: Arc<dyn HeadHandler>) -> Self {
        Self { handler, name }
    }

    pub(crate) fn into_handler(self) -> Arc<dyn HeadHandler> {
        self.handler
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }
}
