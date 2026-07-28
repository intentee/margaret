use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret_websocket::web_socket::WebSocket;

use crate::conversation_message::ConversationMessage;
use crate::response_chunk::ResponseChunk;
use crate::test_session::TestSession;

pub struct CleanupHandler;

#[async_trait]
impl RespondsToWebSocketMessage for CleanupHandler {
    type Message = ConversationMessage;
    type Session = TestSession;

    async fn process(
        &self,
        cancellation_token: CancellationToken,
        session: Arc<TestSession>,
        message: StreamingRequestEnvelope<ConversationMessage>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket
            .send(message.chunk(ResponseChunk {
                text: "working".to_string(),
            }))
            .await?;

        cancellation_token.cancelled().await;

        session
            .cleanups
            .lock()
            .expect("the test session records cleanups")
            .push(message.message().prompt.clone());

        Ok(())
    }
}
