#[derive(Debug, Eq, PartialEq)]
pub(crate) enum CreditGrantOutcome {
    ExchangeIsOver,
    Granted,
    WindowExceeded,
}
