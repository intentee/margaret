use margaret_http::response_continuation::ResponseContinuation;

use crate::created_web_socket_session::CreatedWebSocketSession;

pub enum WebSocketSessionCreationOutcome<Session> {
    Created(CreatedWebSocketSession<Session>),
    Interrupted(ResponseContinuation),
}
