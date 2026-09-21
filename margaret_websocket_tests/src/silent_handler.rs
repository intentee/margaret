use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret_websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret_websocket::web_socket::WebSocket;
use margaret_websocket_envelope::request_envelope::RequestEnvelope;

use crate::silent_message::SilentMessage;
use crate::test_session::TestSession;

/// Holds its exchange open without ever spending its window, so a test can observe the credit
/// ledger and the exchange's cancellation on their own.
pub struct SilentHandler;

#[async_trait]
impl RespondsToWebSocketMessage for SilentHandler {
    type Message = SilentMessage;
    type Session = TestSession;

    async fn process(
        &self,
        cancellation_token: CancellationToken,
        _session: Arc<TestSession>,
        _message: RequestEnvelope<SilentMessage>,
        _socket: WebSocket,
    ) -> anyhow::Result<()> {
        cancellation_token.cancelled().await;

        Ok(())
    }
}
