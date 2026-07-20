use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::web_socket::WebSocket;
use crate::web_socket_error::WebSocketError;
use crate::web_socket_request_message::WebSocketRequestMessage;

#[async_trait]
pub trait RespondsToWebSocketMessage: Send + Sync {
    type Message: WebSocketRequestMessage + Send;
    type Session: Send + Sync;

    async fn process(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<Self::Session>,
        message: <Self::Message as WebSocketRequestMessage>::Envelope,
        socket: WebSocket,
    ) -> Result<(), WebSocketError>;
}
