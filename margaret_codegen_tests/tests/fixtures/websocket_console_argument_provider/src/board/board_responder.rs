use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;

use crate::board::board_chunk::BoardChunk;
use crate::board::board_prompt::BoardPrompt;
use crate::board::board_session::BoardSession;

#[singleton]
pub struct BoardResponder;

impl BoardResponder {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }
}

#[async_trait]
impl RespondsToWebSocketMessage for BoardResponder {
    type Message = BoardPrompt;
    type Session = BoardSession;

    async fn process(
        &self,
        _cancellation_token: CancellationToken,
        session: Arc<BoardSession>,
        message: StreamingRequestEnvelope<BoardPrompt>,
        socket: WebSocket,
    ) -> anyhow::Result<()> {
        socket
            .send(message.fin(BoardChunk {
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
