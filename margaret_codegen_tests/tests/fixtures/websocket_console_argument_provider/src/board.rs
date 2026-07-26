use std::sync::Arc;

use async_trait::async_trait;
use margaret::framework::macros::build_for_session;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::macros::websocket_message;
use margaret::framework::macros::websocket_session;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;
use margaret::framework::websocket::streaming_request_envelope::StreamingRequestEnvelope;
use margaret::framework::websocket::web_socket::WebSocket;
use serde::Deserialize;
use serde::Serialize;
use tokio_util::sync::CancellationToken;
use validator::Validate;

use crate::reader::Reader;

#[websocket_message(request, method = "board_prompt", response = stream)]
#[derive(Deserialize, Validate)]
pub struct BoardPrompt {
    #[validate(length(min = 1))]
    pub prompt: String,
}

#[websocket_message(response, method = "board_chunk")]
#[derive(Serialize)]
pub struct BoardChunk {
    pub text: String,
}

#[websocket_session(path = "/board/{topic}", server = "public")]
pub struct BoardSession {
    reader: Option<Reader>,
    topic: String,
}

impl BoardSession {
    #[build_for_session]
    #[must_use]
    pub fn assemble(
        #[route_parameter(from = "topic")] topic: String,
        #[authenticated_user] reader: Option<Reader>,
    ) -> Self {
        Self { reader, topic }
    }

    #[must_use]
    pub fn reader_name(&self) -> Option<&str> {
        self.reader.as_ref().map(|reader| reader.name.as_str())
    }

    #[must_use]
    pub fn topic(&self) -> &str {
        &self.topic
    }
}

#[singleton]
pub struct BoardResponder;

impl BoardResponder {
    #[constructor]
    #[must_use]
    pub fn create() -> Self {
        Self
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
