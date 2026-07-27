use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::routes::public::sessions::storyboard::messages::conversation_message::ConversationMessage;
use crate::routes::public::sessions::storyboard::messages::response_chunk::ResponseChunk;
use crate::routes::public::sessions::storyboard::storyboard_session::StoryboardSession;

#[singleton]
pub struct Storyboard;

impl Storyboard {
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
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
    ) -> anyhow::Result<()> {
        let prompt = message.message().prompt.clone();

        session.record(prompt.clone()).await;

        let turn_count = session.turn_count().await;
        let viewer = session.viewer_name().unwrap_or("a guest");

        socket
            .send(message.chunk(ResponseChunk {
                text: format!(
                    "{} {} for {viewer} storyboard '{}' about '{}' turn {turn_count}: {prompt}",
                    session.board_url(),
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
