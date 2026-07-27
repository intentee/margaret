use std::collections::BTreeMap;
use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::provider::Provider;

pub(crate) struct ContainerPlan {
    pub(crate) dependency_order: Vec<CanonicalPath>,
    pub(crate) entries: BTreeMap<CanonicalPath, Provider>,
    pub(crate) injectable: BTreeSet<CanonicalPath>,
}

impl ContainerPlan {
    pub(crate) fn entry(&self, key: &CanonicalPath) -> &Provider {
        &self.entries[key]
    }
}
