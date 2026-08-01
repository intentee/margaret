use crate::expected_claims::ExpectedClaims;

pub trait ProvidesExpectedClaims: Send + Sync {
    fn expected_claims(&self) -> ExpectedClaims;
}
