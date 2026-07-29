use std::sync::Arc;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

use crate::planned_dependency::PlannedDependency;
use crate::provider::Provider;

pub(crate) struct PlannedProvider {
    pub(crate) console_arguments: Arc<[ConsoleArgument]>,
    pub(crate) console_slots: Arc<[usize]>,
    pub(crate) dependencies: Arc<[PlannedDependency]>,
    pub(crate) is_async: bool,
    pub(crate) key: CanonicalPath,
    pub(crate) provider: Provider,
}
