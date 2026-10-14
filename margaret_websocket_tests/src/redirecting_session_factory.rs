use async_trait::async_trait;

use margaret_http::redirect::Redirect;
use margaret_http::request::Request;
use margaret_websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct RedirectingSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for RedirectingSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        _handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        Ok(WebSocketSessionCreationOutcome::Interrupted(
            Redirect::see_other("http://localhost/sign-in".to_string()).into_response(),
        ))
    }
}
