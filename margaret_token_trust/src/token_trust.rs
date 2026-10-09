use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenTrust {
    pub audience: &'static str,
    pub issuer: &'static str,
}

impl TokenTrust {
    #[must_use]
    pub fn expectation(&self) -> JwtExpectation<'static> {
        JwtExpectation {
            audience: ExpectedAudience::One(self.audience),
            issuer: self.issuer,
        }
    }
}
