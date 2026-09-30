use anyhow::Result;

use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn persisted_jwks_secret_restores_the_keys_it_persisted() -> Result<()> {
    let trust = fixture_trust();
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())?
        .rotate(&FixtureRsaSigningKeys::default())?;
    let current_token = claims.signed_by(secret.current());
    let PreviousKey::Retired(retired) = secret.previous() else {
        panic!("a rotated secret retires its previous key");
    };
    let retired_token = claims.signed_by(retired);
    let document = serde_json::to_vec(&PersistedJwksSecret::from_secret(&secret))?;
    let restored = serde_json::from_slice::<PersistedJwksSecret>(&document)?
        .into_secret(&FixtureRsaSigningKeys::default())?;

    assert!(matches!(
        restored.verify_jwt::<TestClaims, AccessTokenProfile>(
            &current_token,
            &JwtExpectation {
                audience: &trust.audience,
                issuer: &trust.issuer
            },
            NumericDate::new(0)
        ),
        JwksSecretVerificationResult::SignedWithCurrent(_)
    ));
    assert!(matches!(
        restored.verify_jwt::<TestClaims, AccessTokenProfile>(
            &retired_token,
            &JwtExpectation {
                audience: &trust.audience,
                issuer: &trust.issuer
            },
            NumericDate::new(0)
        ),
        JwksSecretVerificationResult::SignedWithPrevious(_)
    ));

    Ok(())
}
