use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

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
    answered: Arc<AtomicBool>,
    sender: Sender<Message>,
}

impl WebSocket {
    pub(crate) fn new(sender: Sender<Message>) -> Self {
        Self {
            answered: Arc::new(AtomicBool::new(false)),
            sender,
        }
    }

    pub async fn send<Payload: Serialize>(
        &self,
        response: OutboundResponse<Payload>,
    ) -> Result<(), WebSocketError> {
        self.send_frame(response.is_done, &response).await
    }

    pub(crate) fn request_scope(&self) -> Self {
        Self {
            answered: Arc::new(AtomicBool::new(false)),
            sender: self.sender.clone(),
        }
    }

    pub(crate) async fn send_error(
        &self,
        id: RequestId,
        code: EnvelopeErrorCode,
        message: String,
        details: Value,
    ) -> Result<(), WebSocketError> {
        self.send_frame(
            true,
            &OutboundError {
                error: EnvelopeError {
                    code,
                    details,
                    message,
                },
                id,
            },
        )
        .await
    }

    fn claim_answer(&self) -> bool {
        self.answered
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }

    async fn send_frame<WireFrame: Serialize>(
        &self,
        terminates_request: bool,
        frame: &WireFrame,
    ) -> Result<(), WebSocketError> {
        let text = serde_json::to_string(frame)
            .map_err(|source| WebSocketError::SerializeResponse { source })?;

        if terminates_request && !self.claim_answer() {
            return Ok(());
        }

        self.sender
            .send(Message::text(text))
            .await
            .map_err(|source| WebSocketError::Send { source })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use tokio::sync::mpsc;

    use super::WebSocket;
    use crate::outbound_response::OutboundResponse;
    use crate::request_id::RequestId;

    fn terminal_response() -> OutboundResponse<Value> {
        OutboundResponse {
            id: RequestId::Number(1),
            is_done: true,
            method: "test",
            payload: Value::Null,
        }
    }

    #[tokio::test]
    async fn suppresses_a_duplicate_terminal_response() {
        let (sender, mut receiver) = mpsc::channel(4);
        let socket = WebSocket::new(sender);

        socket
            .send(terminal_response())
            .await
            .expect("the first terminal response is sent");
        socket
            .send(terminal_response())
            .await
            .expect("the duplicate terminal response is suppressed without error");

        assert!(receiver.try_recv().is_ok());
        assert!(receiver.try_recv().is_err());
    }
}
