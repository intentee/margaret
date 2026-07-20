use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::websocket_codegen_error::WebsocketCodegenError;
use crate::websocket_internal_event::WebsocketInternalEvent;

pub(crate) fn websocket_internal_events(
    index: &AttributeIndex,
) -> Result<Vec<WebsocketInternalEvent>, WebsocketCodegenError> {
    let selector = AttributeSelector::from_marker("websocket_internal_event");
    let mut events = Vec::new();
    let mut seen = HashSet::new();

    for matched in index.select(&selector) {
        let canonical_path = matched.item().canonical_path().clone();
        let event = canonical_path.to_string();

        let Some(identifier) = index.struct_identifier(&canonical_path) else {
            return Err(WebsocketCodegenError::InternalEventNotAStruct { event });
        };
        let variant = identifier.type_name().to_owned();

        if !seen.insert(canonical_path.clone()) {
            return Err(WebsocketCodegenError::DuplicateInternalEvent { event });
        }

        events.push(WebsocketInternalEvent {
            canonical_path,
            variant,
        });
    }

    Ok(events)
}
