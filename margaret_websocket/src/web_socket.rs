use serde::Serialize;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

use crate::envelope_error::EnvelopeError;
use crate::envelope_error_code::EnvelopeErrorCode;
use crate::outbound_error::OutboundError;
use crate::outbound_response::OutboundResponse;
use crate::request_id::RequestId;
use crate::web_socket_error::WebSocketError;

#[derive(Clone)]
pub struct WebSocket {
    sender: Sender<Message>,
}

impl WebSocket {
    pub(crate) fn new(sender: Sender<Message>) -> Self {
        Self { sender }
    }

    pub async fn send<Payload: Serialize>(
        &self,
        response: OutboundResponse<Payload>,
    ) -> Result<(), WebSocketError> {
        self.send_serializable(&response).await
    }

    pub(crate) async fn send_error(
        &self,
        id: RequestId,
        code: EnvelopeErrorCode,
        message: String,
        details: Value,
    ) -> Result<(), WebSocketError> {
        self.send_serializable(&OutboundError {
            error: EnvelopeError {
                code,
                details,
                message,
            },
            id,
        })
        .await
    }

    async fn send_serializable<WireFrame: Serialize>(
        &self,
        frame: &WireFrame,
    ) -> Result<(), WebSocketError> {
        let text = serde_json::to_string(frame)
            .map_err(|source| WebSocketError::SerializeResponse { source })?;

        self.sender
            .send(Message::text(text))
            .await
            .map_err(|source| WebSocketError::Send { source })
    }
}
