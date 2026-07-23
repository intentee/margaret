use std::collections::BTreeMap;
use std::collections::BTreeSet;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_toposort::cycle::Cycle;
use margaret_toposort::topological_order::topological_order as order_by_dependencies;

use crate::collection_table::CollectionTable;
use crate::container_error::ContainerError;
use crate::dependency_kind::DependencyKind;
use crate::provider::Provider;

fn dependency_cycle(cycle: Cycle<CanonicalPath>) -> ContainerError {
    ContainerError::DependencyCycle {
        path: cycle
            .path
            .iter()
            .map(CanonicalPath::to_string)
            .collect::<Vec<String>>()
            .join(" -> "),
    }
}

fn dependency_keys(provider: &Provider, collections: &CollectionTable) -> BTreeSet<CanonicalPath> {
    let mut keys: BTreeSet<CanonicalPath> = BTreeSet::new();

    for dependency in provider.dependencies() {
        match dependency {
            DependencyKind::Single { provider_key } => {
                keys.insert(provider_key.clone());
            }
            DependencyKind::Collection { trait_path } => {
                for member in collections.members_of(trait_path) {
                    keys.insert(member.clone());
                }
            }
            DependencyKind::ConsoleArgument { .. } => {}
        }
    }

    keys
}

pub(crate) fn topological_order(
    providers: &BTreeMap<CanonicalPath, Provider>,
    collections: &CollectionTable,
) -> Result<(), ContainerError> {
    let dependencies: BTreeMap<CanonicalPath, BTreeSet<CanonicalPath>> = providers
        .iter()
        .map(|(key, provider)| (key.clone(), dependency_keys(provider, collections)))
        .collect();

    order_by_dependencies(&dependencies).map_err(dependency_cycle)?;

    Ok(())
}
