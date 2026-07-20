use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_websocket::web_socket_session_factory::WebSocketSessionFactory;

use crate::test_session::TestSession;

pub struct RejectingSessionFactory;

#[async_trait]
impl WebSocketSessionFactory for RejectingSessionFactory {
    type Session = TestSession;

    async fn create(&self, _handshake: &Request) -> Result<Arc<TestSession>, Response> {
        Err(Response::text(403, "the websocket session was rejected"))
    }
}
