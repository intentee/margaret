use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret_websocket::web_socket::WebSocket;
use margaret_websocket::web_socket_error::WebSocketError;

use crate::conversation_message::ConversationMessage;
use crate::response_chunk::ResponseChunk;
use crate::storyboard_complete::StoryboardComplete;
use crate::test_session::TestSession;

pub struct StoryboardHandler;

#[async_trait]
impl RespondsToWebSocketMessage for StoryboardHandler {
    type Message = ConversationMessage;
    type Session = TestSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<TestSession>,
        message: StreamingRequestEnvelope<ConversationMessage>,
        socket: WebSocket,
    ) -> Result<(), WebSocketError> {
        socket
            .send(message.chunk(ResponseChunk {
                text: format!("thinking about {}", message.message().prompt),
            }))
            .await?;
        socket
            .send(message.fin(StoryboardComplete {
                summary: "done".to_string(),
            }))
            .await?;

        Ok(())
    }
}
