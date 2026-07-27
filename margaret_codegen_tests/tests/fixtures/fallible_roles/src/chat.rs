use std::sync::Arc;

use async_trait::async_trait;
use failures as errors;
use margaret::framework::macros::build_for_session;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::websocket_message;
use margaret::framework::macros::websocket_session;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;
use serde::Deserialize;
use serde::Serialize;
use tokio_util::sync::CancellationToken;
use validator::Validate;

use super::secrets::Secrets;

#[websocket_message(request, method = "chat_say", response = stream)]
#[derive(Deserialize, Validate)]
pub struct ChatSay {
    #[validate(length(min = 1))]
    pub text: String,
}

#[websocket_message(response, method = "chat_echo")]
#[derive(Serialize)]
pub struct ChatEcho {
    pub text: String,
}

#[websocket_session(path = "/chat", server = "public")]
pub struct ChatSession;

impl ChatSession {
    #[build_for_session]
    pub fn assemble() -> errors::Result<Self> {
        Ok(Self)
    }
}

#[singleton]
pub struct ChatResponder {
    secrets: Arc<Secrets>,
}

impl ChatResponder {
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
