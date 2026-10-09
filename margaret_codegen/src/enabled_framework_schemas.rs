use margaret_attributes::crate_root::CrateRoot;
use margaret_container::container_bindings::ContainerBindings;

use crate::framework_state_stores::framework_state_stores;

pub(crate) fn enabled_framework_schemas(bindings: &ContainerBindings) -> Vec<CrateRoot> {
    framework_state_stores()
        .into_iter()
        .filter(|store| bindings.provides(&store.provided))
        .map(|store| store.schema)
        .collect()
}
