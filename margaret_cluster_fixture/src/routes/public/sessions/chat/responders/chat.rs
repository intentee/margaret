use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::routes::public::sessions::chat::chat_session::ChatSession;
use crate::routes::public::sessions::chat::messages::post_message::PostMessage;
use crate::routes::public::sessions::chat::messages::posted_message::PostedMessage;
use crate::stores::message_store::MessageStore;

#[singleton]
pub struct Chat {
    messages: Arc<MessageStore>,
}

impl Chat {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(messages: Arc<MessageStore>) -> anyhow::Result<Self> {
        Ok(Self { messages })
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for Chat {
    type Message = PostMessage;
    type Session = ChatSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<ChatSession>,
        message: StreamingRequestEnvelope<PostMessage>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        let posted = self.messages.post(&message.message().body).await?;

        socket
            .send(message.fin(PostedMessage { id: posted.id }))
            .await?;

        Ok(())
    }
}
