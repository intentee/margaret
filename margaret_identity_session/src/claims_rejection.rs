#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClaimsRejection {
    Expired,
    NotYetValid,
    UnexpectedAudience,
    UnexpectedIssuer,
}
