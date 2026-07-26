use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::accessor_fallibility::AccessorFallibility;
use crate::provider::Provider;

pub(crate) struct ContainerPlan {
    pub(crate) constructions: BTreeMap<CanonicalPath, Provider>,
    pub(crate) fallibility: AccessorFallibility,
    pub(crate) providers: BTreeMap<CanonicalPath, Provider>,
}
