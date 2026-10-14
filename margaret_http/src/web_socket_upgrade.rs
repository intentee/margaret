use std::sync::Arc;

use async_trait::async_trait;
use hyper::upgrade::OnUpgrade;
use tokio_util::sync::CancellationToken;

use crate::request::Request;
use crate::response::Response;
use crate::web_socket_driver_sender::WebSocketDriverSender;

#[async_trait]
pub trait WebSocketUpgrade: Send + Sync {
    async fn upgrade(
        self: Arc<Self>,
        handshake: &Request,
        on_upgrade: OnUpgrade,
        cancellation_token: CancellationToken,
        driver_sender: WebSocketDriverSender,
    ) -> Response;
}
