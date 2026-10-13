use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::cookie_changes::CookieChanges;
use margaret_http::request::Request;
use margaret_websocket_session::created_web_socket_session::CreatedWebSocketSession;
use margaret_websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct TestSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for TestSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        _handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        Ok(WebSocketSessionCreationOutcome::Created(
            CreatedWebSocketSession {
                cookie_changes: CookieChanges {
                    cookies: Vec::new(),
                },
                session: Arc::new(TestSession::default()),
            },
        ))
    }
}
