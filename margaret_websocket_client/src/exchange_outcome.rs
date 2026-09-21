use crate::exchange_interruption::ExchangeInterruption;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ExchangeOutcome {
    Completed,
    Interrupted(ExchangeInterruption),
}
