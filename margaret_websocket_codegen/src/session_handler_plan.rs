use std::collections::HashMap;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::name_allocator::NameAllocator;

use crate::discovered_handler::DiscoveredHandler;
use crate::handler_binding::HandlerBinding;
use crate::handler_kind::HandlerKind;
use crate::message_kind::MessageKind;
use crate::session_plan::SessionPlan;
use crate::web_socket_codegen_error::WebSocketCodegenError;
use crate::web_socket_message::WebSocketMessage;
use crate::web_socket_session::WebSocketSession;

fn handler_method<'message>(
    handler_kind: &HandlerKind,
    message: &'message WebSocketMessage,
    handler: &str,
) -> Result<&'message str, WebSocketCodegenError> {
    match (handler_kind, &message.kind) {
        (HandlerKind::Request, MessageKind::Request { method, .. })
        | (HandlerKind::Notification, MessageKind::Notification { method }) => Ok(method),
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

pub(crate) struct SessionHandlerPlan {
    dispatch_idents: NameAllocator,
    handler_by_method: HashMap<String, (CanonicalPath, String)>,
    notification_handlers: Vec<HandlerBinding>,
    request_handlers: Vec<HandlerBinding>,
    session: WebSocketSession,
}

impl SessionHandlerPlan {
    pub(crate) fn new(session: WebSocketSession) -> Self {
        Self {
            dispatch_idents: NameAllocator::new(),
            handler_by_method: HashMap::new(),
            notification_handlers: Vec::new(),
            request_handlers: Vec::new(),
            session,
        }
    }

    pub(crate) fn bind(
        &mut self,
        handler: &DiscoveredHandler,
        message: &WebSocketMessage,
    ) -> Result<(), WebSocketCodegenError> {
        let handler_name = handler.handler_path.to_string();
        let method = handler_method(&handler.kind, message, &handler_name)?;

        if let Some((first_message, first_handler)) = self.handler_by_method.get(method) {
            if first_message == &message.path {
                return Err(WebSocketCodegenError::DuplicateHandlerForMessage {
                    message: message.path.to_string(),
                    first: first_handler.clone(),
                    second: handler_name,
                });
            }

            return Err(WebSocketCodegenError::DuplicateMethodInSession {
                session: self.session.session_path.to_string(),
                method: method.to_string(),
                first: first_handler.clone(),
                second: handler_name,
            });
        }

        let method = method.to_string();
        let dispatch_ident = format!(
            "{}Dispatch",
            self.dispatch_idents.allocate(&method).type_name()
        );
        let binding = HandlerBinding {
            dispatch_ident,
            handler_field: handler.handler_field.clone(),
            handler_path: handler.handler_path.clone(),
            message_path: message.path.clone(),
        };

        self.handler_by_method
            .insert(method, (message.path.clone(), handler_name));

        match handler.kind {
            HandlerKind::Notification => self.notification_handlers.push(binding),
            HandlerKind::Request => self.request_handlers.push(binding),
        }

        Ok(())
    }

    pub(crate) fn finish(self) -> SessionPlan {
        SessionPlan {
            notification_handlers: self.notification_handlers,
            request_handlers: self.request_handlers,
            session: self.session,
        }
    }
}
