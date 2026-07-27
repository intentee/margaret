use async_trait::async_trait;

use margaret_http::request::Request;

use crate::web_socket_session_creation_error::WebSocketSessionCreationError;
use crate::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;

#[async_trait]
pub trait WebSocketSessionFactory: Send + Sync {
    type Session;

    async fn create(
        &self,
        handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<Self::Session>, WebSocketSessionCreationError>;
}
