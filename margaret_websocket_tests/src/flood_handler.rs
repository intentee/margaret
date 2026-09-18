use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::web_socket::WebSocket;
use margaret_websocket_envelope::streaming_request_envelope::StreamingRequestEnvelope;

use crate::conversation_message::ConversationMessage;
use crate::response_chunk::ResponseChunk;
use crate::test_session::TestSession;

pub struct FloodHandler;

#[async_trait]
impl RespondsToWebSocketMessage for FloodHandler {
    type Message = ConversationMessage;
    type Session = TestSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<TestSession>,
        message: StreamingRequestEnvelope<ConversationMessage>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        loop {
            socket
                .send(message.chunk(ResponseChunk {
                    text: "flood".to_string(),
                }))
                .await?;
        }
    }
}
