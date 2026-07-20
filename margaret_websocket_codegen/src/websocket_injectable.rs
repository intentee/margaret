use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;

use crate::transition_trigger::TransitionTrigger;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WebsocketInjectable {
    Emit,
    Envelope,
    EventValue,
    Facts,
    FromState,
    Spawner,
}

fn connection_facts_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_websocket".to_owned(),
        "connection_facts".to_owned(),
        "ConnectionFacts".to_owned(),
    ])
}

fn emit_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_websocket".to_owned(),
        "emit".to_owned(),
        "Emit".to_owned(),
    ])
}

fn envelope_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_websocket".to_owned(),
        "envelope".to_owned(),
        "Envelope".to_owned(),
    ])
}

fn spawner_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret_websocket".to_owned(),
        "activity_spawner".to_owned(),
        "ActivitySpawner".to_owned(),
    ])
}

pub(crate) fn classify_parameter(
    index: &AttributeIndex,
    item: &IndexedItem,
    from: &CanonicalPath,
    trigger: &TransitionTrigger,
    declared: &Type,
) -> Option<WebsocketInjectable> {
    let resolved = index.resolve_item_type(item, declared)?;
    let is_reference = matches!(declared, Type::Reference(_));

    if !is_reference && resolved == *from {
        return Some(WebsocketInjectable::FromState);
    }

    if is_reference && resolved == emit_path() {
        return Some(WebsocketInjectable::Emit);
    }

    if is_reference && resolved == connection_facts_path() {
        return Some(WebsocketInjectable::Facts);
    }

    if is_reference && resolved == spawner_path() {
        return Some(WebsocketInjectable::Spawner);
    }

    match trigger {
        TransitionTrigger::Message { .. } => {
            if !is_reference && resolved == envelope_path() {
                return Some(WebsocketInjectable::Envelope);
            }
        }
        TransitionTrigger::InternalEvent { canonical_path, .. } => {
            if !is_reference && resolved == *canonical_path {
                return Some(WebsocketInjectable::EventValue);
            }
        }
    }

    None
}
