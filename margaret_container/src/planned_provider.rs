use std::sync::Arc;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::planned_dependency::PlannedDependency;
use crate::provider::Provider;
use crate::slotted_serve_input::SlottedServeInput;

pub(crate) struct PlannedProvider {
    pub(crate) dependencies: Arc<[PlannedDependency]>,
    pub(crate) is_async: bool,
    pub(crate) key: CanonicalPath,
    pub(crate) provider: Provider,
    pub(crate) serve_inputs: Arc<[SlottedServeInput]>,
}
