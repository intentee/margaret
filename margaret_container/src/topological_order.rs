use std::collections::BTreeMap;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::collection_table::CollectionTable;
use crate::container_error::ContainerError;
use crate::dependency_kind::DependencyKind;
use crate::provider::Provider;

enum Mark {
    Done,
    InProgress,
    Unvisited,
}

struct Visitor<'graph> {
    adjacency: &'graph [Vec<usize>],
    providers: &'graph [Provider],
    state: Vec<Mark>,
    stack: Vec<usize>,
    order: Vec<usize>,
}

pub(crate) fn topological_order(
    providers: &[Provider],
    collections: &CollectionTable,
) -> Result<Vec<usize>, ContainerError> {
    let adjacency = build_adjacency(providers, collections);
    let mut visitor = Visitor {
        adjacency: &adjacency,
        providers,
        state: providers.iter().map(|_| Mark::Unvisited).collect(),
        stack: Vec::new(),
        order: Vec::new(),
    };

    for index in 0..providers.len() {
        visitor.visit(index)?;
    }

    Ok(visitor.order)
}

impl Visitor<'_> {
    fn visit(&mut self, index: usize) -> Result<(), ContainerError> {
        match self.state[index] {
            Mark::Done => return Ok(()),
            Mark::InProgress => return Err(self.cycle(index)),
            Mark::Unvisited => {}
        }

        self.state[index] = Mark::InProgress;
        self.stack.push(index);

        let adjacency = self.adjacency;
        for &dependency in &adjacency[index] {
            self.visit(dependency)?;
        }

        self.stack.pop();
        self.state[index] = Mark::Done;
        self.order.push(index);

        Ok(())
    }

    fn cycle(&self, index: usize) -> ContainerError {
        let start = self
            .stack
            .iter()
            .position(|&node| node == index)
            .expect("an in-progress node is on the stack");
        let mut path: Vec<String> = self.stack[start..]
            .iter()
            .map(|&node| self.providers[node].provided.key().to_string())
            .collect();
        path.push(self.providers[index].provided.key().to_string());

        ContainerError::DependencyCycle {
            path: path.join(" -> "),
        }
    }
}

fn build_adjacency(providers: &[Provider], collections: &CollectionTable) -> Vec<Vec<usize>> {
    let mut index_by_key: BTreeMap<CanonicalPath, usize> = BTreeMap::new();
    for (index, provider) in providers.iter().enumerate() {
        index_by_key.insert(provider.provided.key().clone(), index);
    }

    providers
        .iter()
        .map(|provider| dependency_indices(provider, collections, &index_by_key))
        .collect()
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

fn index_of(index_by_key: &BTreeMap<CanonicalPath, usize>, key: &CanonicalPath) -> usize {
    *index_by_key
        .get(key)
        .expect("a resolved dependency key maps to a provider")
}
