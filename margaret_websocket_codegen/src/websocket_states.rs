use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::matched_attribute::MatchedAttribute;

use crate::state_role::StateRole;
use crate::websocket_codegen_error::WebsocketCodegenError;
use crate::websocket_state::WebsocketState;

fn state_role(
    matched: &MatchedAttribute<'_>,
    state: &str,
) -> Result<StateRole, WebsocketCodegenError> {
    let arguments = matched.args()?;
    let server = arguments.string("server")?;
    let path = arguments.string("path")?;
    let terminal = arguments
        .positional_path(0)
        .is_some_and(|positional| positional.is_ident("terminal"));

    match (server, path, terminal) {
        (Some(server), Some(path), false) => Ok(StateRole::Entry { server, path }),
        (Some(_), Some(_), true) => Err(WebsocketCodegenError::ConflictingStateRole {
            state: state.to_owned(),
        }),
        (Some(_), None, _) => Err(WebsocketCodegenError::EntryMissingPath {
            state: state.to_owned(),
        }),
        (None, Some(_), _) => Err(WebsocketCodegenError::EntryMissingServer {
            state: state.to_owned(),
        }),
        (None, None, true) => Ok(StateRole::Terminal),
        (None, None, false) => Ok(StateRole::Intermediate),
    }
}

pub(crate) fn websocket_states(
    index: &AttributeIndex,
) -> Result<Vec<WebsocketState>, WebsocketCodegenError> {
    let selector = AttributeSelector::from_marker("websocket_state");
    let mut states = Vec::new();
    let mut seen = HashSet::new();

    for matched in index.select(&selector) {
        let canonical_path = matched.item().canonical_path().clone();
        let state = canonical_path.to_string();

        let Some(identifier) = index.struct_identifier(&canonical_path) else {
            return Err(WebsocketCodegenError::StateNotAStruct { state });
        };
        let field = identifier.field().to_owned();
        let variant = identifier.type_name().to_owned();

        if !seen.insert(canonical_path.clone()) {
            return Err(WebsocketCodegenError::DuplicateState { state });
        }

        let role = state_role(&matched, &state)?;

        states.push(WebsocketState {
            canonical_path,
            field,
            role,
            variant,
        });
    }

    Ok(states)
}
