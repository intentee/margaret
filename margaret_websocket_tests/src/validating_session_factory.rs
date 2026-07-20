use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http_validation::request_input::RequestInput;
use margaret_http_validation::require_input::require_input;
use margaret_websocket::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;
use crate::upgrade_query::UpgradeQuery;

pub struct ValidatingSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for ValidatingSessionFactory {
    type Session = TestSession;

    async fn create(&self, handshake: &Request) -> Result<Arc<TestSession>, Response> {
        require_input::<UpgradeQuery>(handshake, RequestInput::Query)?;

        Ok(Arc::new(TestSession::default()))
    }
}
