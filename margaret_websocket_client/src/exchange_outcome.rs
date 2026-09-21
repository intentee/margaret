use crate::exchange_interruption::ExchangeInterruption;

pub(crate) enum ExchangeOutcome {
    Completed,
    Interrupted(ExchangeInterruption),
}
