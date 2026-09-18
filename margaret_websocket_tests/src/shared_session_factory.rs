use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct SharedSessionFactory {
    session: Arc<TestSession>,
}

impl SharedSessionFactory {
    #[must_use]
    pub fn new(session: Arc<TestSession>) -> Self {
        Self { session }
    }
}

#[async_trait]
impl WebSocketSessionFactory for SharedSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        _handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        Ok(WebSocketSessionCreationOutcome::Created(
            self.session.clone(),
        ))
    }
}
