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

        let permit = self
            .sender
            .reserve()
            .await
            .map_err(|source| WebSocketError::Send { source })?;

        if terminates_request && !self.claim_answer() {
            return Ok(());
        }

        permit.send(Message::text(text));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use futures_util::FutureExt;
    use serde_json::Value;
    use tokio::sync::mpsc;

    use super::WebSocket;
    use crate::outbound_response::OutboundResponse;
    use crate::request_id::RequestId;

    fn chunk_response() -> OutboundResponse<Value> {
        OutboundResponse {
            id: RequestId::Number(1),
            is_done: false,
            method: "test",
            payload: Value::Null,
        }
    }

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

    #[tokio::test]
    async fn a_cancelled_terminal_send_does_not_consume_the_answer_slot() {
        let (sender, mut receiver) = mpsc::channel(1);
        let socket = WebSocket::new(sender);

        socket
            .send(chunk_response())
            .await
            .expect("the chunk fills the single channel slot");

        assert!(
            socket.send(terminal_response()).now_or_never().is_none(),
            "the terminal send parks on a full channel, then is dropped",
        );

        receiver
            .recv()
            .await
            .expect("the chunk is drained, freeing the slot");

        socket
            .send(terminal_response())
            .await
            .expect("a fresh terminal is delivered after the cancelled send");

        assert!(receiver.try_recv().is_ok());
    }

    #[tokio::test]
    async fn reports_a_send_error_when_the_connection_is_closed() {
        let (sender, receiver) = mpsc::channel(1);
        let socket = WebSocket::new(sender);

        drop(receiver);

        let error = socket
            .send(terminal_response())
            .await
            .expect_err("the send fails once the connection is closed");

        assert_eq!(
            error.to_string(),
            "the websocket connection is closed and can no longer accept outbound frames: channel closed",
        );
    }
}
