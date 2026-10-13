use std::collections::HashMap;
use std::sync::Arc;

use crate::head_handler::HeadHandler;
use crate::named_handler::NamedHandler;

pub struct ForwardTargets {
    by_name: HashMap<&'static str, Arc<dyn HeadHandler>>,
}

impl ForwardTargets {
    #[must_use]
    pub fn new(targets: Vec<NamedHandler>) -> Self {
        let mut by_name = HashMap::with_capacity(targets.len());

        for named_handler in targets {
            by_name.insert(named_handler.name(), named_handler.into_handler());
        }

        Self { by_name }
    }

    pub(crate) fn resolve(&self, name: &str) -> Option<Arc<dyn HeadHandler>> {
        self.by_name.get(name).cloned()
    }
}
