use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;

use crate::access_token_audience::ACCESS_TOKEN_AUDIENCE;
use crate::access_token_issuer::ACCESS_TOKEN_ISSUER;

#[must_use]
pub fn access_token_expectation() -> JwtExpectation<'static> {
    JwtExpectation {
        audience: ExpectedAudience::One(ACCESS_TOKEN_AUDIENCE),
        issuer: ACCESS_TOKEN_ISSUER,
    }
}
