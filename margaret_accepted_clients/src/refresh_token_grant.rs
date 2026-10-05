#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshTokenGrant {
    Granted,
    Withheld,
}
