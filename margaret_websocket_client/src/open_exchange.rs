use tokio::sync::mpsc::UnboundedReceiver;

use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_envelope::server_sent_frame::ServerSentFrame;

use crate::exchange_interruption::ExchangeInterruption;
use crate::exchange_outcome::ExchangeOutcome;
use crate::exchange_registry::ExchangeRegistry;
use crate::exchange_signal::ExchangeSignal;
use crate::exchange_signal_report::ExchangeSignalReport;
use crate::exchange_signals::ExchangeSignals;
use crate::exchange_termination::ExchangeTermination;
use crate::granted_credit::GrantedCredit;

pub(crate) struct OpenExchange {
    granted: GrantedCredit,
    id: RequestId,
    receiver: UnboundedReceiver<ServerSentFrame>,
    registry: ExchangeRegistry,
    signals: ExchangeSignals,
    termination: ExchangeTermination,
}

impl OpenExchange {
    pub(crate) fn new(
        granted: GrantedCredit,
        id: RequestId,
        receiver: UnboundedReceiver<ServerSentFrame>,
        registry: ExchangeRegistry,
        signals: ExchangeSignals,
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
        match self.signals.report(signal) {
            ExchangeSignalReport::ConnectionStoppedReading => self
                .termination
                .interrupt(ExchangeInterruption::ConnectionDropped),
            ExchangeSignalReport::Reported => {}
        }
    }
}

impl Drop for OpenExchange {
    fn drop(&mut self) {
        self.registry.forget(&self.id);
        self.report(ExchangeSignal::Cancelled(self.id.clone()));
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::request_id::RequestId;

    use super::ExchangeInterruption;
    use super::ExchangeOutcome;
    use super::ExchangeRegistry;
    use super::ExchangeSignals;
    use super::ExchangeTermination;
    use super::GrantedCredit;
    use super::OpenExchange;
    use crate::response_credit_window::RESPONSE_CREDIT_WINDOW;

    fn exchange(signals: ExchangeSignals) -> OpenExchange {
        let (_sender, receiver) = mpsc::unbounded_channel();

        OpenExchange::new(
            GrantedCredit::new(RESPONSE_CREDIT_WINDOW),
            RequestId::Number(1),
            receiver,
            ExchangeRegistry::default(),
            signals,
            ExchangeTermination::default(),
        )
    }

    #[test]
    fn reports_a_consumed_frame_to_a_connection_that_still_reads() {
        let (reporter, _signal_queue) = mpsc::unbounded_channel();
        let exchange = exchange(ExchangeSignals::new(reporter));

        exchange.report_consumed();

        assert_eq!(exchange.outcome(), ExchangeOutcome::Completed);
    }

    #[test]
    fn ends_an_exchange_the_connection_can_no_longer_report_for() {
        let (reporter, signal_queue) = mpsc::unbounded_channel();

        drop(signal_queue);

        let exchange = exchange(ExchangeSignals::new(reporter));

        exchange.report_consumed();

        assert_eq!(
            exchange.outcome(),
            ExchangeOutcome::Interrupted(ExchangeInterruption::ConnectionDropped)
        );
    }
}
