use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plans::MiddlewarePlans;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;

use crate::built_websocket_plan::BuiltWebSocketPlan;
use crate::message_kind::MessageKind;
use crate::session_handler_plan::SessionHandlerPlan;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_message::WebSocketMessage;
use crate::websocket_handlers::websocket_handlers;
use crate::websocket_messages::websocket_messages;
use crate::websocket_sessions::websocket_sessions;

fn reject_duplicate_response_methods(
    messages: &[WebSocketMessage],
) -> Result<(), WebSocketCodegenError> {
    let mut message_by_method: HashMap<&str, &CanonicalPath> = HashMap::new();

    for message in messages {
        if let MessageKind::Response { method } = &message.kind {
            if let Some(first) = message_by_method.get(method.as_str()) {
                return Err(WebSocketCodegenError::DuplicateResponseMethod {
                    method: method.clone(),
                    first: first.to_string(),
                    second: message.path.to_string(),
                });
            }

            message_by_method.insert(method, &message.path);
        }
    }

    Ok(())
}

pub(crate) fn build_websocket_plan(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    middleware_plans: &MiddlewarePlans,
    registries: &BindingRegistries,
) -> Result<BuiltWebSocketPlan, WebSocketCodegenError> {
    let messages = websocket_messages(index)?;

    reject_duplicate_response_methods(&messages)?;

    let sessions = websocket_sessions(index, bindings, middleware_plans, registries)?;
    let handlers = websocket_handlers(index)?;
    let message_by_path: HashMap<&CanonicalPath, &WebSocketMessage> = messages
        .iter()
        .map(|message| (&message.path, message))
        .collect();
    let mut handled_messages: HashSet<&CanonicalPath> = HashSet::new();
    let mut session_plans: BTreeMap<CanonicalPath, SessionHandlerPlan> = sessions
        .into_iter()
        .map(|session| {
            (
                session.session_path.clone(),
                SessionHandlerPlan::new(session),
            )
        })
        .collect();

    for handler in &handlers {
        let Some(session_plan) = handler
            .session_path
            .as_ref()
            .and_then(|path| session_plans.get_mut(path))
        else {
            return Err(WebSocketCodegenError::HandlerSessionNotASession {
                handler: handler.handler_path.to_string(),
            });
        };
        let Some(message) = handler
            .message_path
            .as_ref()
            .and_then(|path| message_by_path.get(path))
        else {
            return Err(WebSocketCodegenError::HandlerMessageNotAMessage {
                handler: handler.handler_path.to_string(),
            });
        };

        session_plan.bind(handler, message)?;
        handled_messages.insert(&message.path);
    }

    for message in &messages {
        let requires_handler = matches!(
            &message.kind,
            MessageKind::Request { .. } | MessageKind::Notification { .. }
        );

        if requires_handler && !handled_messages.contains(&message.path) {
            return Err(WebSocketCodegenError::MessageWithoutHandler {
                message: message.path.to_string(),
            });
        }
    }

    Ok(BuiltWebSocketPlan {
        messages,
        sessions: session_plans
            .into_values()
            .map(SessionHandlerPlan::finish)
            .collect(),
    })
}
