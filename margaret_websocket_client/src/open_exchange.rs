use tokio::sync::mpsc::Receiver;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_outcome::ExchangeOutcome;
use crate::exchange_registry::ExchangeRegistry;
use crate::exchange_termination::ExchangeTermination;
use crate::outbound_frames::OutboundFrames;

/// One consumed response frame frees exactly one frame of the window.
const REPLENISHED_BY_ONE_FRAME: usize = 1;

pub(crate) struct OpenExchange {
    frames: OutboundFrames,
    id: RequestId,
    receiver: Receiver<ServerSentFrame>,
    registry: ExchangeRegistry,
    termination: ExchangeTermination,
}

impl OpenExchange {
    pub(crate) fn new(
        frames: OutboundFrames,
        id: RequestId,
        receiver: Receiver<ServerSentFrame>,
        registry: ExchangeRegistry,
        termination: ExchangeTermination,
    ) -> Self {
        Self {
            frames,
            id,
            receiver,
            registry,
            termination,
        }
    }

    pub(crate) fn outcome(&self) -> ExchangeOutcome {
        self.termination.outcome()
    }

    pub(crate) async fn receive(&mut self) -> Option<ServerSentFrame> {
        self.receiver.recv().await
    }

    pub(crate) async fn replenish(&self) {
        if let Err(error) = self
            .frames
            .send_serialization(serde_json::to_string(&ClientSentFrame::<()>::Credit {
                credit: REPLENISHED_BY_ONE_FRAME,
                id: self.id.clone(),
            }))
            .await
        {
            eprintln!(
                "margaret_websocket_client: the exchange window could not be replenished: {error}"
            );
        }
    }
}

impl Drop for OpenExchange {
    fn drop(&mut self) {
        self.registry.forget(&self.id);
    }
}
