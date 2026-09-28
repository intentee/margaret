use anyhow::Result;

use margaret_jose_parameters::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen::persisted_jwks_secret::PersistedJwksSecret;
use margaret_jwks_keygen::previous_key::PreviousKey;
use margaret_jwks_keygen::signs_claims::SignsClaims;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::test_claims::TestClaims;

#[tokio::test]
async fn persisted_jwks_secret_restores_the_keys_it_persisted() -> Result<()> {
    let claims = TestClaims {
        exp: FAR_FUTURE_EXPIRY,
        sub: "subject".to_string(),
    };
    let secret = JwksSecret::fresh(Curve::P256)?.rotate()?;
    let current_token = secret.current().sign(&claims).await?;
    let PreviousKey::Retired(retired) = secret.previous() else {
        panic!("a rotated secret retires its previous key");
    };
    let retired_token = retired.sign(&claims).await?;
    let document = serde_json::to_vec(&PersistedJwksSecret::from_secret(&secret))?;
    let restored = serde_json::from_slice::<PersistedJwksSecret>(&document)?.into_secret()?;

    assert!(matches!(
        restored.verify_any::<TestClaims>(&current_token),
        JwksSecretVerificationResult::SignedWithCurrent(_)
    ));
    assert!(matches!(
        restored.verify_any::<TestClaims>(&retired_token),
        JwksSecretVerificationResult::SignedWithPrevious(_)
    ));

    Ok(())
}
