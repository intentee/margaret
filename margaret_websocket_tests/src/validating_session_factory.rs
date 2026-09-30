use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_http::requirement::Requirement;
use margaret_http_validation::require_input::require_input;
use margaret_validation::validate::validate;
use margaret_websocket_session::web_socket_session_creation_error::WebSocketSessionCreationError;
use margaret_websocket_session::web_socket_session_creation_outcome::WebSocketSessionCreationOutcome;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;
use crate::upgrade_query::UpgradeQuery;

pub struct ValidatingSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for ValidatingSessionFactory {
    type Session = TestSession;

    async fn create(
        &self,
        handshake: &Request,
    ) -> Result<WebSocketSessionCreationOutcome<TestSession>, WebSocketSessionCreationError> {
        if let Requirement::Unmet(continuation) =
            require_input(validate::<UpgradeQuery, _>(&handshake.inputs.query))
        {
            return Ok(WebSocketSessionCreationOutcome::Interrupted(continuation));
        }

        Ok(WebSocketSessionCreationOutcome::Created(Arc::new(
            TestSession::default(),
        )))
    }
}
