use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_macros::constructor;
use margaret_macros::singleton;
use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret_websocket::web_socket::WebSocket;
use margaret_websocket::web_socket_error::WebSocketError;

use crate::routes::public::sessions::storyboard::StoryboardSession;
use crate::routes::public::sessions::storyboard::messages::conversation_message::ConversationMessage;
use crate::routes::public::sessions::storyboard::messages::response_chunk::ResponseChunk;

#[singleton]
pub struct Storyboard;

impl Storyboard {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for Storyboard {
    type Message = ConversationMessage;
    type Session = StoryboardSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<StoryboardSession>,
        message: StreamingRequestEnvelope<ConversationMessage>,
        socket: WebSocket,
    ) -> Result<(), WebSocketError> {
        let prompt = message.message().prompt.clone();

        session.record(prompt.clone()).await;

        let turn_count = session.turn_count().await;

        socket
            .send(message.chunk(ResponseChunk {
                text: format!(
                    "{} {} storyboard '{}' about '{}' turn {turn_count}: {prompt}",
                    session.welcome(),
                    session.greeting(),
                    session.topic(),
                    session.article_title(),
                ),
            }))
            .await?;
        socket
            .send(message.fin(ResponseChunk {
                text: "storyboard complete".to_string(),
            }))
            .await?;

        Ok(())
    }
}
