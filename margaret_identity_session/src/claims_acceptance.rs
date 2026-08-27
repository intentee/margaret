use crate::claims_rejection::ClaimsRejection;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ClaimsAcceptance {
    Accepted,
    Rejected(ClaimsRejection),
}
