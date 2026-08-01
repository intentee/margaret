use margaret_jwt_claims::expected_claims::ExpectedClaims;

use crate::consumer_audience::CONSUMER_AUDIENCE;
use crate::consumer_issuer::CONSUMER_ISSUER;

#[must_use]
pub fn consumer_expected_claims() -> ExpectedClaims {
    ExpectedClaims {
        audience: CONSUMER_AUDIENCE.to_string(),
        issuer: CONSUMER_ISSUER.to_string(),
    }
}
