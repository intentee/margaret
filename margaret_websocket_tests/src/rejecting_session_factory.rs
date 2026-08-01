use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct RejectingSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for RejectingSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        _handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        Ok(WebSocketSessionCreationOutcome::Interrupted(
            Response::text(403, "the websocket session was rejected").into(),
        ))
    }
}
