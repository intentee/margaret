use std::collections::HashMap;
use std::collections::HashSet;

use quote::ToTokens;
use syn::ReturnType;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;

use crate::emitted_message::EmittedMessage;
use crate::transition_trigger::TransitionTrigger;
use crate::websocket_codegen_error::WebsocketCodegenError;
use crate::websocket_injectable::classify_parameter;
use crate::websocket_internal_event::WebsocketInternalEvent;
use crate::websocket_message::WebsocketMessage;
use crate::websocket_state::WebsocketState;
use crate::websocket_transition::WebsocketTransition;

#[derive(Eq, Hash, PartialEq)]
struct DispatchKey {
    from: CanonicalPath,
    on: CanonicalPath,
}

fn classify_trigger(
    resolved: CanonicalPath,
    message_by_path: &HashMap<CanonicalPath, &WebsocketMessage>,
    event_by_path: &HashMap<CanonicalPath, &WebsocketInternalEvent>,
) -> Option<TransitionTrigger> {
    if let Some(message) = message_by_path.get(&resolved) {
        return Some(TransitionTrigger::Message {
            method: message.method.clone(),
            variant: message.variant.clone(),
            canonical_path: resolved,
        });
    }

    if let Some(event) = event_by_path.get(&resolved) {
        return Some(TransitionTrigger::InternalEvent {
            variant: event.variant.clone(),
            canonical_path: resolved,
        });
    }

    None
}

fn is_singleton(item: &IndexedItem) -> bool {
    let selector = AttributeSelector::from_marker("singleton");

    item.attributes()
        .iter()
        .any(|attribute| selector.matches(attribute.path()))
}

fn resolve_next(
    index: &AttributeIndex,
    item: &IndexedItem,
    method: &IndexedMethod,
    state_paths: &HashSet<CanonicalPath>,
    transition: &str,
) -> Result<CanonicalPath, WebsocketCodegenError> {
    let ReturnType::Type(_, declared) = &method.signature().output else {
        return Err(WebsocketCodegenError::TransitionMissingReturnState {
            transition: transition.to_owned(),
        });
    };

    index
        .resolve_item_type(item, declared)
        .filter(|resolved| state_paths.contains(resolved))
        .ok_or_else(|| WebsocketCodegenError::NextNotAState {
            transition: transition.to_owned(),
            written: declared.to_token_stream().to_string(),
        })
}

pub(crate) fn websocket_transitions(
    index: &AttributeIndex,
    states: &[WebsocketState],
    messages: &[WebsocketMessage],
    events: &[WebsocketInternalEvent],
) -> Result<Vec<WebsocketTransition>, WebsocketCodegenError> {
    let state_paths: HashSet<CanonicalPath> = states
        .iter()
        .map(|state| state.canonical_path.clone())
        .collect();
    let message_by_path: HashMap<CanonicalPath, &WebsocketMessage> = messages
        .iter()
        .map(|message| (message.canonical_path.clone(), message))
        .collect();
    let event_by_path: HashMap<CanonicalPath, &WebsocketInternalEvent> = events
        .iter()
        .map(|event| (event.canonical_path.clone(), event))
        .collect();

    let selector = AttributeSelector::from_marker("websocket_transition");
    let mut transitions = Vec::new();
    let mut seen = HashMap::new();

    for matched in index.select(&selector) {
        let item = matched.item();
        let canonical_path = item.canonical_path().clone();
        let transition = canonical_path.to_string();

        let Some(identifier) = index.struct_identifier(&canonical_path) else {
            return Err(WebsocketCodegenError::TransitionNotAStruct { transition });
        };
        let field = identifier.field().to_owned();
        let type_name = item.identifier().to_owned();

        if !is_singleton(item) {
            return Err(WebsocketCodegenError::TransitionNotSingleton { transition });
        }

        let arguments = matched.args()?;
        let from_path =
            arguments
                .path("from")?
                .ok_or_else(|| WebsocketCodegenError::MissingTransitionFrom {
                    transition: transition.clone(),
                })?;
        let on_path =
            arguments
                .path("on")?
                .ok_or_else(|| WebsocketCodegenError::MissingTransitionOn {
                    transition: transition.clone(),
                })?;

        let from = index
            .resolve_item_path(item, &from_path)
            .filter(|resolved| state_paths.contains(resolved))
            .ok_or_else(|| WebsocketCodegenError::FromNotAState {
                transition: transition.clone(),
                written: from_path.to_token_stream().to_string(),
            })?;

        let trigger = index
            .resolve_item_path(item, &on_path)
            .and_then(|resolved| classify_trigger(resolved, &message_by_path, &event_by_path))
            .ok_or_else(|| WebsocketCodegenError::OnNotAMessageOrEvent {
                transition: transition.clone(),
                written: on_path.to_token_stream().to_string(),
            })?;

        let mut emits = Vec::new();

        for emit_path in arguments.call_paths("emits")? {
            let message = index
                .resolve_item_path(item, &emit_path)
                .and_then(|resolved| message_by_path.get(&resolved))
                .ok_or_else(|| WebsocketCodegenError::EmitsNotAMessage {
                    transition: transition.clone(),
                    written: emit_path.to_token_stream().to_string(),
                })?;

            emits.push(EmittedMessage {
                canonical_path: message.canonical_path.clone(),
                method: message.method.clone(),
                variant: message.variant.clone(),
            });
        }

        let method = process_method(item)?;
        let next = resolve_next(index, item, method, &state_paths, &transition)?;

        let mut bindings = Vec::new();

        for parameter in parameters(method.signature()) {
            let binding = classify_parameter(index, item, &from, &trigger, parameter.declared)
                .ok_or_else(|| WebsocketCodegenError::UnclassifiableTransitionParameter {
                    transition: transition.clone(),
                    written: parameter.declared.to_token_stream().to_string(),
                })?;

            bindings.push(binding);
        }

        let key = DispatchKey {
            from: from.clone(),
            on: trigger.canonical_path().clone(),
        };

        if let Some(first) = seen.insert(key, canonical_path.clone()) {
            return Err(WebsocketCodegenError::DuplicateTransition {
                from: from.to_string(),
                on: trigger.canonical_path().to_string(),
                first: first.to_string(),
                second: transition,
            });
        }

        transitions.push(WebsocketTransition {
            transition: canonical_path,
            field,
            type_name,
            process_method: method.identifier().to_owned(),
            from,
            trigger,
            emits,
            next,
            bindings,
        });
    }

    Ok(transitions)
}
