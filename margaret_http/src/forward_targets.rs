use std::collections::HashMap;
use std::sync::Arc;

use crate::handler::Handler;
use crate::named_handler::NamedHandler;

pub struct ForwardTargets {
    by_name: HashMap<&'static str, Arc<dyn Handler>>,
}

impl ForwardTargets {
    #[must_use]
    pub fn new(targets: Vec<NamedHandler>) -> Self {
        let by_name = targets
            .into_iter()
            .map(|named_handler| (named_handler.name(), named_handler.into_handler()))
            .collect();

        Self { by_name }
    }

    pub(crate) fn resolve(&self, name: &str) -> Option<Arc<dyn Handler>> {
        self.by_name.get(name).cloned()
    }
}
