use std::sync::Arc;

use margaret_http::response_continuation::ResponseContinuation;

pub enum WebSocketSessionCreationOutcome<Session> {
    Created(Arc<Session>),
    Interrupted(ResponseContinuation),
}
