use tokio::sync::mpsc::UnboundedSender;

use crate::exchange_signal::ExchangeSignal;
use crate::exchange_signal_report::ExchangeSignalReport;

/// Where an exchange reports what its consumer did, for the connection to turn into the frame the
/// peer expects. A connection that has stopped reading carries nothing, which is an outcome the
/// exchange answers for, not a failure of the report itself.
#[derive(Clone)]
pub(crate) struct ExchangeSignals {
    reporter: UnboundedSender<ExchangeSignal>,
}

impl ExchangeSignals {
    pub(crate) fn new(reporter: UnboundedSender<ExchangeSignal>) -> Self {
        Self { reporter }
    }

    pub(crate) fn report(&self, signal: ExchangeSignal) -> ExchangeSignalReport {
        match self.reporter.send(signal) {
            Ok(()) => ExchangeSignalReport::Reported,
            Err(_) => ExchangeSignalReport::ConnectionStoppedReading,
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::request_id::RequestId;

    use super::ExchangeSignal;
    use super::ExchangeSignalReport;
    use super::ExchangeSignals;

    #[test]
    fn reports_a_signal_to_a_connection_that_still_reads() {
        let (reporter, _signal_queue) = mpsc::unbounded_channel();

        assert_eq!(
            ExchangeSignals::new(reporter).report(ExchangeSignal::Consumed(RequestId::Number(1))),
            ExchangeSignalReport::Reported
        );
    }

    #[test]
    fn answers_for_a_connection_that_has_stopped_reading() {
        let (reporter, signal_queue) = mpsc::unbounded_channel();

        drop(signal_queue);

        assert_eq!(
            ExchangeSignals::new(reporter).report(ExchangeSignal::Cancelled(RequestId::Number(1))),
            ExchangeSignalReport::ConnectionStoppedReading
        );
    }
}
