use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::indexed_trait_impl::IndexedTraitImpl;

use crate::discovered_handler::DiscoveredHandler;
use crate::handler_kind::HandlerKind;
use crate::websocket_codegen_error::WebSocketCodegenError;

fn responds_to_message_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "websocket".to_string(),
        "responds_to_web_socket_message".to_string(),
        "RespondsToWebSocketMessage".to_string(),
    ])
}

fn responds_to_notification_path() -> CanonicalPath {
    CanonicalPath::new(vec![
        "margaret".to_string(),
        "framework".to_string(),
        "websocket".to_string(),
        "responds_to_web_socket_notification".to_string(),
        "RespondsToWebSocketNotification".to_string(),
    ])
}

fn handler_kind(index: &AttributeIndex, trait_impl: &IndexedTraitImpl) -> Option<HandlerKind> {
    let trait_path =
        index.resolve_module_path(trait_impl.module_path(), trait_impl.trait_path())?;

    if trait_path == responds_to_message_path() {
        Some(HandlerKind::Request)
    } else if trait_path == responds_to_notification_path() {
        Some(HandlerKind::Notification)
    } else {
        None
    }
}

fn associated(
    index: &AttributeIndex,
    trait_impl: &IndexedTraitImpl,
    name: &str,
    handler: &str,
) -> Result<Option<CanonicalPath>, WebSocketCodegenError> {
    let associated_type = trait_impl.associated_type(name).ok_or_else(|| {
        WebSocketCodegenError::HandlerMissingAssociatedType {
            handler: handler.to_string(),
            associated_type: name.to_string(),
        }
    })?;

    Ok(index.resolve_module_type(trait_impl.module_path(), associated_type.ty()))
}

pub(crate) fn websocket_handlers(
    index: &AttributeIndex,
) -> Result<Vec<DiscoveredHandler>, WebSocketCodegenError> {
    let singleton_selector = AttributeSelector::from_marker("singleton");
    let mut handlers = Vec::new();

    for item in index.items() {
        for trait_impl in item.trait_impls() {
            let Some(kind) = handler_kind(index, trait_impl) else {
                continue;
            };
            let handler = item.canonical_path().to_string();

            if !item.has_attribute(&singleton_selector) {
                return Err(WebSocketCodegenError::HandlerNotSingleton { handler });
            }

            let session_path = associated(index, trait_impl, "Session", &handler)?;
            let message_path = associated(index, trait_impl, "Message", &handler)?;

            handlers.push(DiscoveredHandler {
                handler_path: item.canonical_path().clone(),
                kind,
                message_path,
                session_path,
            });
        }
    }

    handlers.sort_by_key(|handler| handler.handler_path.to_string());

    Ok(handlers)
}
