use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenTrust {
    pub audience: Audience,
    pub issuer: IssuerIdentifier,
}

impl TokenTrust {
    #[must_use]
    pub fn expectation(&self) -> JwtExpectation<'_> {
        JwtExpectation {
            audience: ExpectedAudience::One(&self.audience),
            issuer: &self.issuer,
        }
    }
}
