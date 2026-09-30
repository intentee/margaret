use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jws_verification::jws_rejection::JwsRejection;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn jwks_secret_rejects_a_token_of_another_secret() -> Result<()> {
    let trust = fixture_trust();
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())?;
    let token = claims.signed_by(
        JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())?.current(),
    );

    assert!(matches!(
        secret.verify_jwt::<TestClaims, AccessTokenProfile>(
            &token,
            &JwtExpectation {
                audience: &trust.audience,
                issuer: &trust.issuer
            },
            NumericDate::new(0)
        ),
        JwksSecretVerificationResult::Rejected(JwtRejection::Jws(
            JwsRejection::UnknownKeyId { .. }
        ))
    ));

    Ok(())
}
