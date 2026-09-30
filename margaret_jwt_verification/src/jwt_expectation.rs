use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

pub struct JwtExpectation<'expectation> {
    pub audience: &'expectation Audience,
    pub issuer: &'expectation IssuerIdentifier,
}
