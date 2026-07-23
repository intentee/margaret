use std::cell::Cell;
use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::collection_table::CollectionTable;
use crate::container_error::ContainerError;
use crate::dependency_kind::DependencyKind;
use crate::provider::Provider;

#[derive(Clone, Copy)]
enum Mark {
    Done,
    InProgress,
    Unvisited,
}

struct Node<'plan> {
    adjacency: Vec<&'plan CanonicalPath>,
    mark: Cell<Mark>,
}

fn dependency_keys<'plan>(
    provider: &'plan Provider,
    collections: &'plan CollectionTable,
) -> Vec<&'plan CanonicalPath> {
    let mut keys = Vec::new();

    for dependency in provider.dependencies() {
        match dependency {
            DependencyKind::Single { provider_key } => keys.push(provider_key),
            DependencyKind::Collection { trait_path } => {
                keys.extend(collections.members_of(trait_path));
            }
            DependencyKind::ConsoleArgument { .. } => {}
        }
    }

    keys
}

fn nodes_of<'plan>(
    providers: &'plan BTreeMap<CanonicalPath, Provider>,
    collections: &'plan CollectionTable,
) -> BTreeMap<&'plan CanonicalPath, Node<'plan>> {
    providers
        .iter()
        .map(|(key, provider)| {
            (
                key,
                Node {
                    adjacency: dependency_keys(provider, collections),
                    mark: Cell::new(Mark::Unvisited),
                },
            )
        })
        .collect()
}

fn cycle(key: &CanonicalPath, stack: &[&CanonicalPath]) -> ContainerError {
    let mut path: Vec<String> = stack
        .iter()
        .copied()
        .skip_while(|entry| *entry != key)
        .map(CanonicalPath::to_string)
        .collect();
    path.push(key.to_string());

    ContainerError::DependencyCycle {
        path: path.join(" -> "),
    }
}

fn visit<'plan>(
    nodes: &BTreeMap<&'plan CanonicalPath, Node<'plan>>,
    key: &'plan CanonicalPath,
    stack: &mut Vec<&'plan CanonicalPath>,
) -> Result<(), ContainerError> {
    let node = &nodes[key];

    match node.mark.get() {
        Mark::Done => return Ok(()),
        Mark::InProgress => return Err(cycle(key, stack)),
        Mark::Unvisited => {}
    }

    node.mark.set(Mark::InProgress);
    stack.push(key);

    for &dependency in &node.adjacency {
        visit(nodes, dependency, stack)?;
    }

    stack.pop();
    node.mark.set(Mark::Done);

    Ok(())
}

pub(crate) fn topological_order(
    providers: &BTreeMap<CanonicalPath, Provider>,
    collections: &CollectionTable,
) -> Result<(), ContainerError> {
    let nodes = nodes_of(providers, collections);
    let mut stack = Vec::new();

    for key in providers.keys() {
        visit(&nodes, key, &mut stack)?;
    }

    Ok(())
}
