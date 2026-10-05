use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::expected_audience::ExpectedAudience;

pub struct JwtExpectation<'expectation> {
    pub audience: ExpectedAudience<'expectation>,
    pub issuer: &'expectation IssuerIdentifier,
}
