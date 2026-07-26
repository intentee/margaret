use std::sync::Arc;

use async_trait::async_trait;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::request_envelope::RequestEnvelope;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::web_socket::WebSocket;
use tokio_util::sync::CancellationToken;

use crate::beta_session::BetaSession;
use crate::conversation_message::ConversationMessage;

#[singleton]
pub struct OnConversationMessage;

impl OnConversationMessage {
    #[constructor]
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for OnConversationMessage {
    type Message = ConversationMessage;
    type Session = BetaSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<BetaSession>,
        _message: RequestEnvelope<ConversationMessage>,
        _socket: WebSocket,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
