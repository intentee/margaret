#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ExchangeSignalReport {
    ConnectionStoppedReading,
    Reported,
}
