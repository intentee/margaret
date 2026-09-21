use std::sync::Arc;

use dashmap::DashMap;

use margaret_websocket_envelope::request_id::RequestId;

use crate::exchange_interruption::ExchangeInterruption;
use crate::exchange_outcome::ExchangeOutcome;
use crate::exchange_registration::ExchangeRegistration;
use crate::exchange_termination::ExchangeTermination;
use crate::pending_exchange::PendingExchange;

#[derive(Clone, Default)]
pub(crate) struct ExchangeRegistry {
    connection: ExchangeTermination,
    exchanges: Arc<DashMap<RequestId, PendingExchange>>,
}

impl ExchangeRegistry {
    pub(crate) fn forget(&self, id: &RequestId) {
        self.exchanges.remove(id);
    }

    pub(crate) fn interrupt_all(&self, interruption: ExchangeInterruption) {
        self.connection.interrupt(interruption);

        for exchange in self.exchanges.iter() {
            exchange.value().termination.interrupt(interruption);
        }

        self.exchanges.clear();
    }

    pub(crate) fn outcome(&self) -> ExchangeOutcome {
        self.connection.outcome()
    }

    pub(crate) fn peek(&self, id: &RequestId) -> Option<PendingExchange> {
        self.exchanges.get(id).map(|entry| entry.value().clone())
    }

    pub(crate) fn register(
        &self,
        id: &RequestId,
        exchange: PendingExchange,
    ) -> ExchangeRegistration {
        self.exchanges.insert(id.clone(), exchange);

        match self.connection.outcome() {
            ExchangeOutcome::Completed => ExchangeRegistration::Registered,
            ExchangeOutcome::Interrupted(interruption) => {
                self.exchanges.remove(id);

                ExchangeRegistration::Refused(interruption)
            }
        }
    }

    pub(crate) fn take(&self, id: &RequestId) -> Option<PendingExchange> {
        self.exchanges.remove(id).map(|(_, exchange)| exchange)
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::mpsc;

    use margaret_websocket_envelope::request_id::RequestId;

    use super::ExchangeRegistry;
    use crate::exchange_interruption::ExchangeInterruption;
    use crate::exchange_registration::ExchangeRegistration;
    use crate::exchange_termination::ExchangeTermination;
    use crate::pending_exchange::PendingExchange;
    use crate::response_backlog_limit::RESPONSE_BACKLOG_LIMIT;

    fn exchange() -> PendingExchange {
        let (sender, receiver) = mpsc::channel(RESPONSE_BACKLOG_LIMIT);

        drop(receiver);

        PendingExchange {
            sender,
            termination: ExchangeTermination::default(),
        }
    }

    #[test]
    fn registers_an_exchange_while_the_connection_still_reads() {
        let registry = ExchangeRegistry::default();
        let id = RequestId::Number(1);

        assert!(matches!(
            registry.register(&id, exchange()),
            ExchangeRegistration::Registered
        ));
        assert!(registry.peek(&id).is_some());
    }

    #[test]
    fn refuses_an_exchange_once_the_connection_has_stopped_reading() {
        let registry = ExchangeRegistry::default();
        let id = RequestId::Number(1);

        registry.interrupt_all(ExchangeInterruption::PeerSentUnreadableFrame);

        assert!(matches!(
            registry.register(&id, exchange()),
            ExchangeRegistration::Refused(ExchangeInterruption::PeerSentUnreadableFrame)
        ));
        assert!(registry.peek(&id).is_none());
    }
}
