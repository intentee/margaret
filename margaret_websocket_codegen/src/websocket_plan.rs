use std::collections::HashMap;
use std::mem::take;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::name_allocator::NameAllocator;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_plan::MiddlewarePlan;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;

use crate::handler_binding::HandlerBinding;
use crate::handler_kind::HandlerKind;
use crate::message_kind::MessageKind;
use crate::session_plan::SessionPlan;
use crate::websocket_codegen_error::WebSocketCodegenError;
use crate::websocket_handlers::websocket_handlers;
use crate::websocket_message::WebSocketMessage;
use crate::websocket_messages::websocket_messages;
use crate::websocket_sessions::websocket_sessions;

pub(crate) struct WebSocketPlan {
    pub(crate) messages: Vec<WebSocketMessage>,
    pub(crate) sessions: Vec<SessionPlan>,
}

fn handler_method(
    handler_kind: &HandlerKind,
    message: &WebSocketMessage,
    handler: &str,
) -> Result<String, WebSocketCodegenError> {
    match (handler_kind, &message.kind) {
        (HandlerKind::Request, MessageKind::Request { method, .. }) => Ok(method.clone()),
        (HandlerKind::Notification, MessageKind::Notification { method }) => Ok(method.clone()),
        (HandlerKind::Request, MessageKind::Notification { .. }) => {
            Err(WebSocketCodegenError::HandlerMessageNotARequest {
                handler: handler.to_string(),
                message: message.path.to_string(),
            })
        }
        (HandlerKind::Notification, MessageKind::Request { .. }) => {
            Err(WebSocketCodegenError::HandlerMessageNotANotification {
                handler: handler.to_string(),
                message: message.path.to_string(),
            })
        }
        (_, MessageKind::Response { .. }) => Err(WebSocketCodegenError::HandlerMessageIsResponse {
            handler: handler.to_string(),
        }),
    }
}

fn reject_duplicate_response_methods(
    messages: &[WebSocketMessage],
) -> Result<(), WebSocketCodegenError> {
    let mut seen: HashMap<String, String> = HashMap::new();

    for message in messages {
        if let MessageKind::Response { method } = &message.kind {
            if let Some(first) = seen.get(method) {
                return Err(WebSocketCodegenError::DuplicateResponseMethod {
                    method: method.clone(),
                    first: first.clone(),
                    second: message.path.to_string(),
                });
            }

            seen.insert(method.clone(), message.path.to_string());
        }
    }

    Ok(())
}

pub(crate) fn websocket_plan(
    index: &AttributeIndex,
    bindings: &ContainerBindings,
    middleware_plans: &[MiddlewarePlan],
    registries: &BindingRegistries,
) -> Result<WebSocketPlan, WebSocketCodegenError> {
    let messages = websocket_messages(index)?;

    reject_duplicate_response_methods(&messages)?;

    let sessions = websocket_sessions(index, bindings, middleware_plans, registries)?;
    let handlers = websocket_handlers(index)?;

    let message_by_path: HashMap<String, &WebSocketMessage> = messages
        .iter()
        .map(|message| (message.path.to_string(), message))
        .collect();
    let session_index: HashMap<String, usize> = sessions
        .iter()
        .enumerate()
        .map(|(position, session)| (session.session_path.to_string(), position))
        .collect();

    let mut request_handlers: Vec<Vec<HandlerBinding>> =
        sessions.iter().map(|_| Vec::new()).collect();
    let mut notification_handlers: Vec<Vec<HandlerBinding>> =
        sessions.iter().map(|_| Vec::new()).collect();
    let mut message_handler: HashMap<String, String> = HashMap::new();
    let mut session_methods: HashMap<usize, HashMap<String, String>> = HashMap::new();
    let mut dispatch_idents: HashMap<usize, NameAllocator> = HashMap::new();

    for handler in &handlers {
        let handler_name = handler.handler_path.to_string();

        let Some(&session_index) = handler
            .session_path
            .as_ref()
            .and_then(|path| session_index.get(&path.to_string()))
        else {
            return Err(WebSocketCodegenError::HandlerSessionNotASession {
                handler: handler_name,
            });
        };
        let Some(message) = handler
            .message_path
            .as_ref()
            .and_then(|path| message_by_path.get(&path.to_string()))
        else {
            return Err(WebSocketCodegenError::HandlerMessageNotAMessage {
                handler: handler_name,
            });
        };

        let message_name = message.path.to_string();

        if let Some(first) = message_handler.get(&message_name) {
            return Err(WebSocketCodegenError::DuplicateHandlerForMessage {
                message: message_name,
                first: first.clone(),
                second: handler_name,
            });
        }

        let method = handler_method(&handler.kind, message, &handler_name)?;
        let methods = session_methods.entry(session_index).or_default();

        if let Some(first) = methods.get(&method) {
            return Err(WebSocketCodegenError::DuplicateMethodInSession {
                session: sessions[session_index].session_path.to_string(),
                method,
                first: first.clone(),
                second: handler_name,
            });
        }

        methods.insert(method.clone(), handler_name.clone());
        message_handler.insert(message_name, handler_name);

        let dispatch_ident = format!(
            "{}Dispatch",
            dispatch_idents
                .entry(session_index)
                .or_default()
                .allocate(&method)
                .type_name()
        );

        let binding = HandlerBinding {
            dispatch_ident,
            handler_field: handler.handler_field.clone(),
            handler_path: handler.handler_path.clone(),
            method,
        };

        match handler.kind {
            HandlerKind::Request => request_handlers[session_index].push(binding),
            HandlerKind::Notification => notification_handlers[session_index].push(binding),
        }
    }

    for message in &messages {
        let requires_handler = match &message.kind {
            MessageKind::Request { .. } | MessageKind::Notification { .. } => true,
            MessageKind::Response { .. } => false,
        };

        if requires_handler && !message_handler.contains_key(&message.path.to_string()) {
            return Err(WebSocketCodegenError::MessageWithoutHandler {
                message: message.path.to_string(),
            });
        }
    }

    let session_plans = sessions
        .into_iter()
        .enumerate()
        .map(|(position, session)| SessionPlan {
            notification_handlers: take(&mut notification_handlers[position]),
            request_handlers: take(&mut request_handlers[position]),
            session,
        })
        .collect();

    Ok(WebSocketPlan {
        messages,
        sessions: session_plans,
    })
}
