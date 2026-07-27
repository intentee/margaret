use crate::handler_binding::HandlerBinding;
use crate::web_socket_session::WebSocketSession;

pub(crate) struct SessionPlan {
    pub(crate) notification_handlers: Vec<HandlerBinding>,
    pub(crate) request_handlers: Vec<HandlerBinding>,
    pub(crate) session: WebSocketSession,
}
