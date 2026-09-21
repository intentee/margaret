use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio_util::sync::CancellationToken;

use margaret_websocket_envelope::credit_grant::CreditGrant;

use crate::credit_grant_outcome::CreditGrantOutcome;

#[derive(Clone)]
pub(crate) struct ServedExchange {
    pub(crate) credit: Arc<Semaphore>,
    cancellation_token: CancellationToken,
}

impl ServedExchange {
    pub(crate) fn new(cancellation_token: CancellationToken, window: CreditGrant) -> Self {
        Self {
            cancellation_token,
            credit: Arc::new(Semaphore::new(window.frames())),
        }
    }

    pub(crate) fn end(&self) {
        self.credit.close();
        self.cancellation_token.cancel();
    }

    pub(crate) fn grant(&self, credit: CreditGrant) -> CreditGrantOutcome {
        if self.credit.available_permits() + credit.frames() > CreditGrant::MAXIMUM {
            return CreditGrantOutcome::WindowExceeded;
        }

        self.credit.add_permits(credit.frames());

        CreditGrantOutcome::Granted
    }

    pub(crate) fn is_same_exchange(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.credit, &other.credit)
    }
}
