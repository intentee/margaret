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

struct Node {
    adjacency: Vec<usize>,
    key: String,
    mark: Cell<Mark>,
}

fn index_of(index_by_key: &BTreeMap<CanonicalPath, usize>, key: &CanonicalPath) -> usize {
    index_by_key.get(key).copied().unwrap_or_default()
}

fn dependency_indices(
    provider: &Provider,
    collections: &CollectionTable,
    index_by_key: &BTreeMap<CanonicalPath, usize>,
) -> Vec<usize> {
    let mut indices = Vec::new();

    for dependency in provider.dependencies() {
        match dependency {
            DependencyKind::Single { provider_key } => {
                indices.push(index_of(index_by_key, provider_key));
            }
            DependencyKind::Collection { trait_path } => {
                for member in collections.members_of(trait_path) {
                    indices.push(index_of(index_by_key, member));
                }
            }
        }
    }

    indices
}

fn nodes_of(providers: &[Provider], collections: &CollectionTable) -> Vec<Node> {
    let mut index_by_key: BTreeMap<CanonicalPath, usize> = BTreeMap::new();

    for (index, provider) in providers.iter().enumerate() {
        index_by_key.insert(provider.provided.key().clone(), index);
    }

    providers
        .iter()
        .map(|provider| Node {
            adjacency: dependency_indices(provider, collections, &index_by_key),
            key: provider.provided.key().to_string(),
            mark: Cell::new(Mark::Unvisited),
        })
        .collect()
}

fn cycle(node: &Node, index: usize, stack: &[(usize, String)]) -> ContainerError {
    let mut path: Vec<String> = stack
        .iter()
        .skip_while(|(node_index, _)| *node_index != index)
        .map(|(_, key)| key.clone())
        .collect();
    path.push(node.key.clone());

    ContainerError::DependencyCycle {
        path: path.join(" -> "),
    }
}

fn visit(
    nodes: &[Node],
    index: usize,
    stack: &mut Vec<(usize, String)>,
    order: &mut Vec<usize>,
) -> Result<(), ContainerError> {
    let Some(node) = nodes.get(index) else {
        return Ok(());
    };

    match node.mark.get() {
        Mark::Done => return Ok(()),
        Mark::InProgress => return Err(cycle(node, index, stack)),
        Mark::Unvisited => {}
    }

    node.mark.set(Mark::InProgress);
    stack.push((index, node.key.clone()));

    for &dependency in &node.adjacency {
        visit(nodes, dependency, stack, order)?;
    }

    stack.pop();
    node.mark.set(Mark::Done);
    order.push(index);

    Ok(())
}

pub(crate) fn topological_order(
    providers: &[Provider],
    collections: &CollectionTable,
) -> Result<Vec<usize>, ContainerError> {
    let nodes = nodes_of(providers, collections);
    let mut stack = Vec::new();
    let mut order = Vec::new();

    for index in 0..nodes.len() {
        visit(&nodes, index, &mut stack, &mut order)?;
    }

    Ok(order)
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::Mark;
    use super::Node;
    use super::visit;

    #[test]
    fn visiting_an_index_beyond_the_graph_is_a_no_op() {
        let nodes = vec![Node {
            adjacency: Vec::new(),
            key: "crate::Only".to_string(),
            mark: Cell::new(Mark::Unvisited),
        }];
        let mut stack = Vec::new();
        let mut order = Vec::new();

        assert!(visit(&nodes, 5, &mut stack, &mut order).is_ok());
        assert!(order.is_empty());
    }
}
