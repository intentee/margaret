use std::sync::Arc;

use dashmap::DashMap;

use margaret_websocket_envelope::request_id::RequestId;

use crate::exchange_interruption::ExchangeInterruption;
use crate::pending_exchange::PendingExchange;

#[derive(Clone, Default)]
pub(crate) struct PendingResponses {
    exchanges: Arc<DashMap<RequestId, PendingExchange>>,
}

impl PendingResponses {
    pub(crate) fn forget(&self, id: &RequestId) {
        self.exchanges.remove(id);
    }

    pub(crate) fn interrupt_all(&self, interruption: ExchangeInterruption) {
        for exchange in self.exchanges.iter() {
            exchange.value().termination.interrupt(interruption);
        }

        self.exchanges.clear();
    }

    pub(crate) fn peek(&self, id: &RequestId) -> Option<PendingExchange> {
        self.exchanges.get(id).map(|entry| entry.value().clone())
    }

    pub(crate) fn remember(&self, id: RequestId, exchange: PendingExchange) {
        self.exchanges.insert(id, exchange);
    }

    pub(crate) fn take(&self, id: &RequestId) -> Option<PendingExchange> {
        self.exchanges.remove(id).map(|(_, exchange)| exchange)
    }
}
