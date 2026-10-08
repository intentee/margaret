use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn verifies_an_access_token_signed_with_the_retired_key() {
    let secret = fresh_p256_secret();
    let signed = rolled_store(secret.clone())
        .await
        .sign_access_token(&json!({ "name": "demo" }), unix_time(500))
        .expect("the claims are signed");
    let rotated = rolled_store(
        secret
            .rotate(&FixtureRsaSigningKeys::default())
            .expect("the signing secret rotates"),
    )
    .await;

    assert!(matches!(
        rotated.verify_access_token::<Map<String, Value>>(&signed.signed_claims, unix_time(500)),
        JwksSecretVerificationResult::SignedWithPrevious(_)
    ));
}
