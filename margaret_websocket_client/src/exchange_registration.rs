use crate::exchange_interruption::ExchangeInterruption;

pub(crate) enum ExchangeRegistration {
    Refused(ExchangeInterruption),
    Registered,
}
