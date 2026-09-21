use tokio::sync::mpsc::Receiver;

use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_outcome::ExchangeOutcome;
use crate::exchange_registry::ExchangeRegistry;
use crate::exchange_termination::ExchangeTermination;

pub(crate) struct OpenExchange {
    id: RequestId,
    registry: ExchangeRegistry,
    receiver: Receiver<ServerSentFrame>,
    termination: ExchangeTermination,
}

impl OpenExchange {
    pub(crate) fn new(
        id: RequestId,
        registry: ExchangeRegistry,
        receiver: Receiver<ServerSentFrame>,
        termination: ExchangeTermination,
    ) -> Self {
        Self {
            id,
            registry,
            receiver,
            termination,
        }
    }

    pub(crate) fn outcome(&self) -> ExchangeOutcome {
        self.termination.outcome()
    }

    pub(crate) async fn receive(&mut self) -> Option<ServerSentFrame> {
        self.receiver.recv().await
    }
}

impl Drop for OpenExchange {
    fn drop(&mut self) {
        self.registry.forget(&self.id);
    }
}
