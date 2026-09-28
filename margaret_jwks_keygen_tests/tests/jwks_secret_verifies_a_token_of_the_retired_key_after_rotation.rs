use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn jwks_secret_verifies_a_token_of_the_retired_key_after_rotation() -> Result<()> {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?;
    let token = claims.signed_by(secret.current());
    let rotated = secret.rotate()?;

    assert!(matches!(
        rotated.verify_jwt::<TestClaims>(&token, TypeHeaderExpectation::Required(JwtType::AccessToken), NumericDate::new(0)),
        JwksSecretVerificationResult::SignedWithPrevious(verified) if verified.claims == claims
    ));

    Ok(())
}
