use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::type_header_expectation::TypeHeaderExpectation;
use margaret_registered_claims::numeric_date::NumericDate;

#[test]
fn persisted_jwks_secret_restores_the_keys_it_persisted() -> Result<()> {
    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?.rotate()?;
    let current_token = claims.signed_by(secret.current());
    let PreviousKey::Retired(retired) = secret.previous() else {
        panic!("a rotated secret retires its previous key");
    };
    let retired_token = claims.signed_by(retired);
    let document = serde_json::to_vec(&PersistedJwksSecret::from_secret(&secret))?;
    let restored = serde_json::from_slice::<PersistedJwksSecret>(&document)?.into_secret()?;

    assert!(matches!(
        restored.verify_jwt::<TestClaims>(
            &current_token,
            TypeHeaderExpectation::Required(JwtType::AccessToken),
            NumericDate::new(0)
        ),
        JwksSecretVerificationResult::SignedWithCurrent(_)
    ));
    assert!(matches!(
        restored.verify_jwt::<TestClaims>(
            &retired_token,
            TypeHeaderExpectation::Required(JwtType::AccessToken),
            NumericDate::new(0)
        ),
        JwksSecretVerificationResult::SignedWithPrevious(_)
    ));

    Ok(())
}
