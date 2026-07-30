use std::sync::Arc;

use async_trait::async_trait;
use failures as errors;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;
use tokio_util::sync::CancellationToken;

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
    type Message = crate::chat::chat_say::ChatSay;
    type Session = crate::chat::chat_session::ChatSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<crate::chat::chat_session::ChatSession>,
        message: StreamingRequestEnvelope<crate::chat::chat_say::ChatSay>,
        socket: WebSocket,
    ) -> errors::Result<()> {
        let _ = self.secrets.token();

        socket
            .send(message.fin(crate::chat::chat_echo::ChatEcho {
                text: message.message().text.clone(),
            }))
            .await?;

        Ok(())
    }
}
