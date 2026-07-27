use std::sync::Arc;

use async_trait::async_trait;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;
use tokio_util::sync::CancellationToken;

#[singleton]
pub struct BoardResponder;

impl BoardResponder {
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for BoardResponder {
    type Message = crate::board::board_prompt::BoardPrompt;
    type Session = crate::board::board_session::BoardSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<crate::board::board_session::BoardSession>,
        message: StreamingRequestEnvelope<crate::board::board_prompt::BoardPrompt>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket
            .send(message.fin(crate::board::board_chunk::BoardChunk {
                text: format!(
                    "{} {}: {}",
                    session.reader_name().unwrap_or("guest"),
                    session.topic(),
                    message.message().prompt,
                ),
            }))
            .await?;

        Ok(())
    }
}
