use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::notification_envelope::NotificationEnvelope;
use crate::web_socket::WebSocket;

#[async_trait]
pub trait RespondsToWebSocketNotification: Send + Sync {
    type Message: Send;
    type Session: Send + Sync;

    async fn process(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<Self::Session>,
        message: NotificationEnvelope<Self::Message>,
        socket: WebSocket,
    ) -> anyhow::Result<()>;
}
