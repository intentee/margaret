use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_jwt_verification::verify_serialized_jwt::verify_serialized_jwt;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::numeric_date::NumericDate;

use crate::test_claims::TestClaims;

#[must_use]
pub fn verified_token(
    key_set: &VerificationKeySet,
    token: &str,
) -> JwtVerification<TestClaims, AccessTokenProfile> {
    verify_serialized_jwt(
        key_set,
        token,
        &fixture_trust().expectation(),
        NumericDate::new(0),
    )
}
