#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrantOperation {
    FindRefreshToken,
    HoldPendingAuthorization,
    IssueCode,
    OpenRefreshFamily,
    RedeemCode,
    RevokeRefreshFamily,
    RotateRefreshToken,
    TakePendingAuthorization,
}
