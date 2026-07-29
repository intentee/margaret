use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::cycle::Cycle;

fn cycle_from<NodeId>(node: &NodeId, stack: &[NodeId]) -> Cycle<NodeId>
where
    NodeId: Clone + PartialEq,
{
    let mut path: Vec<NodeId> = stack
        .iter()
        .skip_while(|entry| *entry != node)
        .cloned()
        .collect();

    path.push(node.clone());

    Cycle { path }
}

fn visit<NodeId>(
    node: &NodeId,
    dependencies: &BTreeMap<NodeId, BTreeSet<NodeId>>,
    marks: &mut BTreeMap<NodeId, Mark>,
    stack: &mut Vec<NodeId>,
    order: &mut Vec<NodeId>,
) -> Result<(), Cycle<NodeId>>
where
    NodeId: Clone + Ord,
{
    match marks.get(node) {
        Some(Mark::Done) => return Ok(()),
        Some(Mark::InProgress) => return Err(cycle_from(node, stack)),
        None => {}
    }

    marks.insert(node.clone(), Mark::InProgress);
    stack.push(node.clone());

    if let Some(targets) = dependencies.get(node) {
        for target in targets {
            visit(target, dependencies, marks, stack, order)?;
        }
    }

    stack.pop();
    marks.insert(node.clone(), Mark::Done);
    order.push(node.clone());

    Ok(())
}

enum Mark {
    Done,
    InProgress,
}

pub fn topological_order<NodeId>(
    dependencies: &BTreeMap<NodeId, BTreeSet<NodeId>>,
) -> Result<Vec<NodeId>, Cycle<NodeId>>
where
    NodeId: Clone + Ord,
{
    let mut nodes: BTreeSet<NodeId> = dependencies.keys().cloned().collect();

    for targets in dependencies.values() {
        nodes.extend(targets.iter().cloned());
    }

    let mut marks: BTreeMap<NodeId, Mark> = BTreeMap::new();
    let mut stack: Vec<NodeId> = Vec::new();
    let mut order: Vec<NodeId> = Vec::new();

    for node in &nodes {
        visit(node, dependencies, &mut marks, &mut stack, &mut order)?;
    }

    Ok(order)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    use super::topological_order;

    fn graph(edges: &[(&str, &[&str])]) -> BTreeMap<String, BTreeSet<String>> {
        edges
            .iter()
            .map(|(node, dependencies)| {
                (
                    (*node).to_string(),
                    dependencies
                        .iter()
                        .map(|dependency| (*dependency).to_string())
                        .collect(),
                )
            })
            .collect()
    }

    fn order(edges: &[(&str, &[&str])]) -> Option<Vec<String>> {
        topological_order(&graph(edges)).ok()
    }

    fn names(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| (*name).to_string()).collect()
    }

    #[test]
    fn orders_an_empty_graph_as_empty() {
        assert_eq!(order(&[]), Some(names(&[])));
    }

    #[test]
    fn orders_a_single_node() {
        assert_eq!(order(&[("a", &[])]), Some(names(&["a"])));
    }

    #[test]
    fn orders_dependencies_before_dependents() {
        assert_eq!(
            order(&[("a", &["b"]), ("b", &[])]),
            Some(names(&["b", "a"]))
        );
    }

    #[test]
    fn orders_independent_nodes_in_sorted_order() {
        assert_eq!(
            order(&[("zebra", &[]), ("apple", &[])]),
            Some(names(&["apple", "zebra"]))
        );
    }

    #[test]
    fn orders_a_diamond_with_a_shared_dependency_first() {
        assert_eq!(
            order(&[("a", &["b", "c"]), ("b", &["d"]), ("c", &["d"]), ("d", &[])]),
            Some(names(&["d", "b", "c", "a"]))
        );
    }

    #[test]
    fn includes_targets_that_are_not_declared_as_nodes() {
        assert_eq!(order(&[("a", &["b"])]), Some(names(&["b", "a"])));
    }

    #[test]
    fn reports_a_mutual_dependency_as_a_cycle() {
        let cycle = topological_order(&graph(&[("a", &["b"]), ("b", &["a"])]))
            .expect_err("a mutual dependency is a cycle");

        assert_eq!(cycle.path, ["a", "b", "a"]);
    }

    #[test]
    fn reports_a_self_dependency_as_a_cycle() {
        let cycle =
            topological_order(&graph(&[("a", &["a"])])).expect_err("a self dependency is a cycle");

        assert_eq!(cycle.path, ["a", "a"]);
    }
}
