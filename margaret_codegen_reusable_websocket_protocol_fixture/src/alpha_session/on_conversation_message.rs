use std::sync::Arc;

use async_trait::async_trait;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::request_envelope::RequestEnvelope;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::web_socket::WebSocket;
use margaret::framework::websocket::web_socket_error::WebSocketError;
use tokio_util::sync::CancellationToken;

use crate::alpha_session::AlphaSession;
use crate::conversation_message::ConversationMessage;

#[singleton]
pub struct OnConversationMessage;

impl OnConversationMessage {
    #[constructor]
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for OnConversationMessage {
    type Message = ConversationMessage;
    type Session = AlphaSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<AlphaSession>,
        _message: RequestEnvelope<ConversationMessage>,
        _socket: WebSocket,
    ) -> Result<(), WebSocketError> {
        Ok(())
    }
}
