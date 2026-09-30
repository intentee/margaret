#[derive(Debug, Eq, PartialEq)]
pub enum RefreshRevocation {
    ForeignClient,
    Revoked,
    Unknown,
}
