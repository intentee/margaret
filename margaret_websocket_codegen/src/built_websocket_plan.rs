use crate::session_plan::SessionPlan;
use crate::web_socket_message::WebSocketMessage;

pub(crate) struct BuiltWebSocketPlan {
    pub(crate) messages: Vec<WebSocketMessage>,
    pub(crate) sessions: Vec<SessionPlan>,
}
