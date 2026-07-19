use std::cell::Cell;

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

fn dependency_indices(provider: &Provider, collections: &CollectionTable) -> Vec<usize> {
    let mut indices = Vec::new();

    for dependency in provider.dependencies() {
        match dependency {
            DependencyKind::Single { provider_index } => indices.push(*provider_index),
            DependencyKind::Collection { trait_path } => {
                indices.extend(collections.members_of(trait_path).iter().copied());
            }
        }
    }

    indices
}

fn nodes_of(providers: &[Provider], collections: &CollectionTable) -> Vec<Node> {
    providers
        .iter()
        .map(|provider| Node {
            adjacency: dependency_indices(provider, collections),
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
    let node = &nodes[index];

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
