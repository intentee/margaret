use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_http::require_peer_spiffe_id::require_peer_spiffe_id;
use margaret_websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct VerifiedPeerSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for VerifiedPeerSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        Ok(match require_peer_spiffe_id(handshake) {
            Ok(_peer) => WebSocketSessionCreationOutcome::Created(Arc::new(TestSession::default())),
            Err(response) => WebSocketSessionCreationOutcome::Interrupted(response.into()),
        })
    }
}
