use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::envelopes_web_socket_request::EnvelopesWebSocketRequest;

use crate::web_socket::WebSocket;

#[async_trait]
pub trait RespondsToWebSocketMessage: Send + Sync {
    type Message: EnvelopesWebSocketRequest + Send;
    type Session: Send + Sync;

    async fn process(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<Self::Session>,
        message: <Self::Message as EnvelopesWebSocketRequest>::Envelope,
        socket: WebSocket,
    ) -> anyhow::Result<()>;
}
