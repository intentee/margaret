use margaret_registered_claims::audience::Audience;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::type_header_expectation::TypeHeaderExpectation;

pub struct JwtExpectation<'expectation> {
    pub audience: &'expectation Audience,
    pub issuer: &'expectation IssuerIdentifier,
    pub token_type: TypeHeaderExpectation,
}
