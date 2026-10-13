use std::collections::BTreeMap;
use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_toposort::cycle::Cycle;
use margaret_toposort::topological_order;

use crate::container_error::ContainerError;
use crate::provider::Provider;

fn dependency_cycle(cycle: &Cycle<CanonicalPath>) -> ContainerError {
    ContainerError::DependencyCycle {
        path: cycle
            .path
            .iter()
            .map(CanonicalPath::to_string)
            .collect::<Vec<String>>()
            .join(" -> "),
    }
}

fn dependency_keys(provider: &Provider) -> BTreeSet<CanonicalPath> {
    provider
        .dependencies()
        .iter()
        .flat_map(|dependency| dependency.provider_keys().iter().cloned())
        .collect()
}

pub(crate) fn topological_order(
    providers: &BTreeMap<CanonicalPath, Provider>,
) -> Result<Vec<CanonicalPath>, ContainerError> {
    let dependencies: BTreeMap<CanonicalPath, BTreeSet<CanonicalPath>> = providers
        .iter()
        .map(|(key, provider)| (key.clone(), dependency_keys(provider)))
        .collect();

    topological_order::topological_order(&dependencies).map_err(|cycle| dependency_cycle(&cycle))
}
