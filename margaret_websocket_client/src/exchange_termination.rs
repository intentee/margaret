use std::sync::Arc;
use std::sync::OnceLock;

use crate::exchange_interruption::ExchangeInterruption;
use crate::exchange_outcome::ExchangeOutcome;

#[derive(Clone, Debug, Default)]
pub(crate) struct ExchangeTermination {
    interruption: Arc<OnceLock<ExchangeInterruption>>,
}

impl ExchangeTermination {
    pub(crate) fn interrupt(&self, interruption: ExchangeInterruption) {
        self.interruption.get_or_init(|| interruption);
    }

    pub(crate) fn outcome(&self) -> ExchangeOutcome {
        match self.interruption.get() {
            Some(interruption) => ExchangeOutcome::Interrupted(*interruption),
            None => ExchangeOutcome::Completed,
        }
    }
}
