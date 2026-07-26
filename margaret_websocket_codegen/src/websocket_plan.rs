use crate::session_plan::SessionPlan;
use crate::websocket_message::WebSocketMessage;

pub(crate) struct WebSocketPlan {
    pub(crate) messages: Vec<WebSocketMessage>,
    pub(crate) sessions: Vec<SessionPlan>,
}
