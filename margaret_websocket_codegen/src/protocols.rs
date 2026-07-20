use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::protocol::Protocol;
use crate::state_role::StateRole;
use crate::websocket_codegen_error::WebsocketCodegenError;
use crate::websocket_state::WebsocketState;
use crate::websocket_transition::WebsocketTransition;

#[derive(Eq, Hash, PartialEq)]
struct RouteKey {
    path: String,
    server: String,
}

struct EntryRoute<'model> {
    entry: &'model WebsocketState,
    path: String,
    server: String,
}

fn backward_edges(
    transitions: &[WebsocketTransition],
) -> HashMap<CanonicalPath, Vec<CanonicalPath>> {
    let mut edges: HashMap<CanonicalPath, Vec<CanonicalPath>> = HashMap::new();

    for transition in transitions {
        edges
            .entry(transition.next.clone())
            .or_default()
            .push(transition.from.clone());
    }

    edges
}

fn check_dead_ends(
    states: &[WebsocketState],
    transitions: &[WebsocketTransition],
) -> Result<(), WebsocketCodegenError> {
    let productive = productive_states(states, transitions);

    for state in states {
        if !state.role.is_terminal() && !productive.contains(&state.canonical_path) {
            return Err(WebsocketCodegenError::DeadEndState {
                state: state.canonical_path.to_string(),
            });
        }
    }

    Ok(())
}

fn check_terminal_transitions(
    states: &[WebsocketState],
    transitions: &[WebsocketTransition],
) -> Result<(), WebsocketCodegenError> {
    for state in states {
        if !state.role.is_terminal() {
            continue;
        }

        if let Some(transition) = transitions
            .iter()
            .find(|transition| transition.from == state.canonical_path)
        {
            return Err(WebsocketCodegenError::TerminalStateHasTransitions {
                state: state.canonical_path.to_string(),
                on: transition.trigger.canonical_path().to_string(),
            });
        }
    }

    Ok(())
}

fn entry_routes(
    states: &[WebsocketState],
) -> Result<Vec<EntryRoute<'_>>, WebsocketCodegenError> {
    let mut routes = Vec::new();
    let mut seen: HashMap<RouteKey, CanonicalPath> = HashMap::new();

    for state in states {
        let StateRole::Entry { server, path } = &state.role else {
            continue;
        };

        let key = RouteKey {
            path: path.clone(),
            server: server.clone(),
        };

        if let Some(first) = seen.insert(key, state.canonical_path.clone()) {
            return Err(WebsocketCodegenError::DuplicateProtocolRoute {
                server: server.clone(),
                path: path.clone(),
                first: first.to_string(),
                second: state.canonical_path.to_string(),
            });
        }

        routes.push(EntryRoute {
            entry: state,
            path: path.clone(),
            server: server.clone(),
        });
    }

    if routes.is_empty() {
        return Err(WebsocketCodegenError::MissingEntryState);
    }

    Ok(routes)
}

fn forward_edges(transitions: &[WebsocketTransition]) -> HashMap<CanonicalPath, Vec<CanonicalPath>> {
    let mut edges: HashMap<CanonicalPath, Vec<CanonicalPath>> = HashMap::new();

    for transition in transitions {
        edges
            .entry(transition.from.clone())
            .or_default()
            .push(transition.next.clone());
    }

    edges
}

fn productive_states(
    states: &[WebsocketState],
    transitions: &[WebsocketTransition],
) -> HashSet<CanonicalPath> {
    let edges = backward_edges(transitions);
    let mut productive = HashSet::new();
    let mut queue = VecDeque::new();

    for state in states {
        if state.role.is_terminal() {
            productive.insert(state.canonical_path.clone());
            queue.push_back(state.canonical_path.clone());
        }
    }

    while let Some(current) = queue.pop_front() {
        let Some(predecessors) = edges.get(&current) else {
            continue;
        };

        for predecessor in predecessors {
            if productive.insert(predecessor.clone()) {
                queue.push_back(predecessor.clone());
            }
        }
    }

    productive
}

fn reachable(
    start: &CanonicalPath,
    edges: &HashMap<CanonicalPath, Vec<CanonicalPath>>,
) -> HashSet<CanonicalPath> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.clone());
    queue.push_back(start.clone());

    while let Some(current) = queue.pop_front() {
        let Some(nexts) = edges.get(&current) else {
            continue;
        };

        for next in nexts {
            if visited.insert(next.clone()) {
                queue.push_back(next.clone());
            }
        }
    }

    visited
}

pub(crate) fn protocols<'model>(
    states: &'model [WebsocketState],
    transitions: &'model [WebsocketTransition],
) -> Result<Vec<Protocol<'model>>, WebsocketCodegenError> {
    let routes = entry_routes(states)?;

    check_terminal_transitions(states, transitions)?;

    let edges = forward_edges(transitions);
    let reached_sets: Vec<HashSet<CanonicalPath>> = routes
        .iter()
        .map(|route| reachable(&route.entry.canonical_path, &edges))
        .collect();

    let mut membership: HashMap<CanonicalPath, CanonicalPath> = HashMap::new();

    for index in 0..routes.len() {
        let entry = &routes[index].entry.canonical_path;

        for state in &reached_sets[index] {
            if let Some(first) = membership.insert(state.clone(), entry.clone()) {
                return Err(WebsocketCodegenError::AmbiguousProtocolMembership {
                    state: state.to_string(),
                    first: first.to_string(),
                    second: entry.to_string(),
                });
            }
        }
    }

    for state in states {
        if !membership.contains_key(&state.canonical_path) {
            return Err(WebsocketCodegenError::UnreachableState {
                state: state.canonical_path.to_string(),
            });
        }
    }

    check_dead_ends(states, transitions)?;

    let mut assembled = Vec::new();

    for index in 0..routes.len() {
        let route = &routes[index];
        let reached = &reached_sets[index];

        assembled.push(Protocol {
            entry: route.entry.canonical_path.clone(),
            entry_field: route.entry.field.clone(),
            server: route.server.clone(),
            path: route.path.clone(),
            states: states
                .iter()
                .filter(|state| reached.contains(&state.canonical_path))
                .collect(),
            transitions: transitions
                .iter()
                .filter(|transition| reached.contains(&transition.from))
                .collect(),
        });
    }

    Ok(assembled)
}
