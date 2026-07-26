use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket::request_envelope::RequestEnvelope;
use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::web_socket::WebSocket;

use crate::ping_message::PingMessage;
use crate::response_chunk::ResponseChunk;
use crate::test_session::TestSession;

pub struct PingHandler;

#[async_trait]
impl RespondsToWebSocketMessage for PingHandler {
    type Message = PingMessage;
    type Session = TestSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        _session: Arc<TestSession>,
        message: RequestEnvelope<PingMessage>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket
            .send(message.response(ResponseChunk {
                text: format!("pong {}", message.message().label),
            }))
            .await?;

        Ok(())
    }
}
