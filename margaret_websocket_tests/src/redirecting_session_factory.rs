use async_trait::async_trait;

use margaret_http::get_target::GetTarget;
use margaret_http::request::Request;
use margaret_http::route_reference::RouteReference;
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
            RouteReference::<GetTarget>::new("http://localhost/sign-in".to_string())
                .see_other()
                .into(),
        ))
    }
}
