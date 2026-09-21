use tokio::sync::mpsc::Receiver;

use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_outcome::ExchangeOutcome;
use crate::exchange_termination::ExchangeTermination;
use crate::pending_responses::PendingResponses;

pub(crate) struct OpenExchange {
    id: RequestId,
    pending: PendingResponses,
    receiver: Receiver<ServerSentFrame>,
    termination: ExchangeTermination,
}

impl OpenExchange {
    pub(crate) fn new(
        id: RequestId,
        pending: PendingResponses,
        receiver: Receiver<ServerSentFrame>,
        termination: ExchangeTermination,
    ) -> Self {
        Self {
            id,
            pending,
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
        self.pending.forget(&self.id);
    }
}
