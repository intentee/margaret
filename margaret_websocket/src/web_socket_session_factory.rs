use std::sync::Arc;

use async_trait::async_trait;

use margaret_http::request::Request;
use margaret_http::response_continuation::ResponseContinuation;

#[async_trait]
pub trait WebSocketSessionFactory: Send + Sync {
    type Session;

    async fn create(&self, handshake: &Request)
    -> Result<Arc<Self::Session>, ResponseContinuation>;
}
