use std::sync::Arc;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::planned_dependency::PlannedDependency;
use crate::provider::Provider;

pub(crate) struct PlannedProvider {
    pub(crate) dependencies: Arc<[PlannedDependency]>,
    pub(crate) is_async: bool,
    pub(crate) key: CanonicalPath,
    pub(crate) provider: Provider,
    pub(crate) serve_input_slots: Arc<[usize]>,
    pub(crate) serve_inputs: Arc<[ServeInput]>,
}
