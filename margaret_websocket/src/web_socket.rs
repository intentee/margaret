use serde::Serialize;
use serde_json::Value;

use margaret_websocket_envelope::envelope_error_code::EnvelopeErrorCode;
use margaret_websocket_envelope::outbound_response::OutboundResponse;
use margaret_websocket_envelope::request_id::RequestId;

use crate::exchange_flow_control::ExchangeFlowControl;
use crate::outbound_frames::OutboundFrames;
use crate::web_socket_error::WebSocketError;

#[derive(Clone)]
pub struct WebSocket {
    credit: ExchangeFlowControl,
    frames: OutboundFrames,
}

impl WebSocket {
    pub(crate) fn new(credit: ExchangeFlowControl, frames: OutboundFrames) -> Self {
        Self { credit, frames }
    }

    /// # Errors
    ///
    /// Returns `WebSocketError` propagated from the work it performs.
    pub async fn send<Payload: Serialize>(
        &self,
        response: OutboundResponse<Payload>,
    ) -> Result<(), WebSocketError> {
        self.credit.spend_one_frame().await?;

        self.frames.send_serializable(&response).await
    }

    pub(crate) async fn send_error(
        &self,
        id: RequestId,
        code: EnvelopeErrorCode,
        message: String,
        details: Value,
    ) -> Result<(), WebSocketError> {
        self.frames.send_error(id, code, message, details).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::Semaphore;
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::outbound_response::OutboundResponse;
    use margaret_websocket_envelope::request_id::RequestId;

    use super::ExchangeFlowControl;
    use super::OutboundFrames;
    use super::WebSocket;
    use super::WebSocketError;

    const ONE_FRAME: usize = 1;

    #[tokio::test]
    async fn reports_a_response_the_finished_exchange_can_no_longer_carry() {
        let (sender, _receiver) = mpsc::channel(ONE_FRAME);
        let credit = Arc::new(Semaphore::new(ONE_FRAME));

        credit.close();

        let socket = WebSocket::new(
            ExchangeFlowControl::Metered(credit.clone()),
            OutboundFrames::new(sender),
        );
        let refused = socket
            .send(OutboundResponse {
                id: RequestId::Number(1),
                is_done: true,
                method: "response_chunk",
                payload: "late",
            })
            .await
            .expect_err("a finished exchange carries no response");

        assert!(matches!(
            refused,
            WebSocketError::ExchangeFinished { .. } if credit.is_closed()
        ));
    }
}
