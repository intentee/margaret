use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;

use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_outcome::ExchangeOutcome;
use crate::exchange_registry::ExchangeRegistry;
use crate::exchange_signal::ExchangeSignal;
use crate::exchange_termination::ExchangeTermination;
use crate::granted_credit::GrantedCredit;

pub(crate) struct OpenExchange {
    granted: GrantedCredit,
    id: RequestId,
    receiver: UnboundedReceiver<ServerSentFrame>,
    registry: ExchangeRegistry,
    signals: UnboundedSender<ExchangeSignal>,
    termination: ExchangeTermination,
}

impl OpenExchange {
    pub(crate) fn new(
        granted: GrantedCredit,
        id: RequestId,
        receiver: UnboundedReceiver<ServerSentFrame>,
        registry: ExchangeRegistry,
        signals: UnboundedSender<ExchangeSignal>,
        termination: ExchangeTermination,
    ) -> Self {
        Self {
            granted,
            id,
            receiver,
            registry,
            signals,
            termination,
        }
    }

    pub(crate) fn outcome(&self) -> ExchangeOutcome {
        self.termination.outcome()
    }

    pub(crate) async fn receive(&mut self) -> Option<ServerSentFrame> {
        self.receiver.recv().await
    }

    pub(crate) fn report_consumed(&self) {
        self.granted.grant_one_frame();
        self.report(ExchangeSignal::Consumed(self.id.clone()));
    }

    fn report(&self, signal: ExchangeSignal) {
        drop(self.signals.send(signal));
    }
}

impl Drop for OpenExchange {
    fn drop(&mut self) {
        self.registry.forget(&self.id);
        self.report(ExchangeSignal::Cancelled(self.id.clone()));
    }
}
