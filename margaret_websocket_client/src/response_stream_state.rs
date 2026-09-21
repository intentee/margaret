#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ResponseStreamState {
    Ended,
    Open,
}
