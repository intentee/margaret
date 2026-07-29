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

    /// # Errors
    ///
    /// Returns `WebSocketError` propagated from the work it performs.
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

    use super::WebSocket;
    use crate::outbound_response::OutboundResponse;
    use crate::request_id::RequestId;

    #[tokio::test]
    async fn reports_a_closed_outbound_channel() {
        let (sender, receiver) = mpsc::channel(1);
        drop(receiver);
        let socket = WebSocket::new(sender);
        let response = OutboundResponse {
            id: RequestId::Number(1),
            is_done: true,
            method: "complete",
            payload: (),
        };
        let error = socket
            .send(response)
            .await
            .expect_err("the closed channel rejects the frame");

        assert!(
            error
                .to_string()
                .starts_with("the websocket connection is closed")
        );
    }
}
