use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_websocket::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct FailingSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for FailingSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        _handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        Err(WebSocketSessionCreationError::consumer(anyhow::anyhow!(
            "secret database endpoint"
        )))
    }
}
