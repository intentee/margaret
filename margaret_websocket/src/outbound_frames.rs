use serde::Serialize;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_envelope::envelope_error::EnvelopeError;
use margaret_websocket_envelope::envelope_error_code::EnvelopeErrorCode;
use margaret_websocket_envelope::outbound_error::OutboundError;
use margaret_websocket_envelope::request_id::RequestId;

use crate::serialize_frame::serialize_frame;
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

    pub(crate) async fn send_frame(&self, frame: String) -> Result<(), WebSocketError> {
        self.sender
            .send(Message::text(frame))
            .await
            .map_err(|source| WebSocketError::Send { source })
    }

    pub(crate) async fn send_serializable<WireFrame: Serialize>(
        &self,
        frame: &WireFrame,
    ) -> Result<(), WebSocketError> {
        self.send_frame(serialize_frame(frame)?).await
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde::Serializer;
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::outbound_response::OutboundResponse;
    use margaret_websocket_envelope::request_id::RequestId;

    use super::OutboundFrames;
    use super::WebSocketError;

    const ONE_FRAME: usize = 1;

    struct Unserializable;

    impl Serialize for Unserializable {
        fn serialize<Target>(&self, _target: Target) -> Result<Target::Ok, Target::Error>
        where
            Target: Serializer,
        {
            Err(serde::ser::Error::custom("this frame never serializes"))
        }
    }

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

    #[tokio::test]
    async fn keeps_a_frame_that_cannot_be_written_out_of_the_connection() {
        let (sender, mut receiver) = mpsc::channel(ONE_FRAME);
        let refused = OutboundFrames::new(sender)
            .send_serializable(&Unserializable)
            .await
            .expect_err("a frame that never serializes never reaches the connection");

        assert!(matches!(
            refused,
            WebSocketError::SerializeResponse { ref source } if source.is_data()
        ));
        assert!(receiver.try_recv().is_err());
    }
}
