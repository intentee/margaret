use std::sync::Arc;

use async_trait::async_trait;
use failures as errors;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::chat::chat_echo::ChatEcho;
use crate::chat::chat_say::ChatSay;
use crate::chat::chat_session::ChatSession;
use crate::secrets::Secrets;

#[singleton]
pub struct ChatResponder {
    secrets: Arc<Secrets>,
}

impl ChatResponder {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(secrets: Arc<Secrets>) -> errors::Result<Self> {
        Ok(Self { secrets })
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for ChatResponder {
    type Message = ChatSay;
    type Session = ChatSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<ChatSession>,
        message: StreamingRequestEnvelope<ChatSay>,
        socket: WebSocket,
    ) -> errors::Result<()> {
        let _ = self.secrets.token();

        socket
            .send(message.fin(ChatEcho {
                text: message.message().text.clone(),
            }))
            .await?;

        Ok(())
    }
}
