use std::sync::Arc;

use async_trait::async_trait;
use hyper::upgrade::OnUpgrade;
use tokio_util::sync::CancellationToken;

use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait WebSocketUpgrade: Send + Sync {
    async fn upgrade(
        self: Arc<Self>,
        handshake: &Request,
        on_upgrade: OnUpgrade,
        cancellation_token: CancellationToken,
    ) -> ResponseContinuation;
}
