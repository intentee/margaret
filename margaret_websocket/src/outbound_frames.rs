use serde::Serialize;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_envelope::envelope_error::EnvelopeError;
use margaret_websocket_envelope::envelope_error_code::EnvelopeErrorCode;
use margaret_websocket_envelope::outbound_error::OutboundError;
use margaret_websocket_envelope::request_id::RequestId;

use crate::web_socket_error::WebSocketError;

#[derive(Clone)]
pub(crate) struct OutboundFrames {
    sender: Sender<Message>,
}

impl OutboundFrames {
    pub(crate) fn new(sender: Sender<Message>) -> Self {
        Self { sender }
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

    pub(crate) async fn send_serializable<WireFrame: Serialize>(
        &self,
        frame: &WireFrame,
    ) -> Result<(), WebSocketError> {
        self.send_serialization(serde_json::to_string(frame)).await
    }

    async fn send_serialization(
        &self,
        serialized: Result<String, serde_json::Error>,
    ) -> Result<(), WebSocketError> {
        let text = serialized.map_err(|source| WebSocketError::SerializeResponse { source })?;

        self.sender
            .send(Message::text(text))
            .await
            .map_err(|source| WebSocketError::Send { source })
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::outbound_response::OutboundResponse;
    use margaret_websocket_envelope::request_id::RequestId;

    use super::OutboundFrames;
    use super::WebSocketError;

    const ONE_FRAME: usize = 1;

    #[tokio::test]
    async fn reports_a_frame_the_closed_connection_can_no_longer_drain() {
        let (sender, receiver) = mpsc::channel(ONE_FRAME);

        drop(receiver);

        let refused = OutboundFrames::new(sender)
            .send_serializable(&OutboundResponse {
                id: RequestId::Number(1),
                is_done: true,
                method: "response_chunk",
                payload: "unreachable",
            })
            .await
            .expect_err("a closed connection drains nothing");

        assert!(matches!(
            refused,
            WebSocketError::Send { ref source } if source.0.is_text()
        ));
    }
}
