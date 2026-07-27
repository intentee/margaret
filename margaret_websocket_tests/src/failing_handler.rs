use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret_websocket::web_socket::WebSocket;

use crate::conversation_message::ConversationMessage;
use crate::failing_chunk::FailingChunk;
use crate::test_session::TestSession;

pub struct FailingHandler;

#[async_trait]
impl RespondsToWebSocketMessage for FailingHandler {
    type Message = ConversationMessage;
    type Session = TestSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<TestSession>,
        message: StreamingRequestEnvelope<ConversationMessage>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket.send(message.chunk(FailingChunk)).await?;

        Ok(())
    }
}
