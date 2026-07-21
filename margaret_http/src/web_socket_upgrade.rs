use std::sync::Arc;

use async_trait::async_trait;
use hyper::upgrade::OnUpgrade;
use tokio_util::sync::CancellationToken;

use crate::forward_targets::ForwardTargets;
use crate::request::Request;
use crate::response::Response;

#[async_trait]
pub trait WebSocketUpgrade: Send + Sync {
    async fn upgrade(
        self: Arc<Self>,
        handshake: Request,
        on_upgrade: OnUpgrade,
        cancellation_token: CancellationToken,
        forward_targets: &Arc<ForwardTargets>,
    ) -> Response;
}
