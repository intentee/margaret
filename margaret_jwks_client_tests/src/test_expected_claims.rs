use margaret_jwt_claims::expected_claims::ExpectedClaims;

use crate::test_audience::TEST_AUDIENCE;
use crate::test_issuer::TEST_ISSUER;

#[must_use]
pub fn test_expected_claims() -> ExpectedClaims {
    ExpectedClaims {
        audience: TEST_AUDIENCE.to_string(),
        issuer: TEST_ISSUER.to_string(),
    }
}
