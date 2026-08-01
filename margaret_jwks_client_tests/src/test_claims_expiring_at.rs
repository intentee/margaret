use margaret_jwt_claims::audience::Audience;

use crate::test_audience::TEST_AUDIENCE;
use crate::test_claims::TestClaims;
use crate::test_issuer::TEST_ISSUER;

#[must_use]
pub fn test_claims_expiring_at(exp: i64) -> TestClaims {
    TestClaims {
        aud: Audience::Many(vec![TEST_AUDIENCE.to_string()]),
        exp,
        iss: TEST_ISSUER.to_string(),
        sub: "subject".to_string(),
    }
}
