use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::rolled_secret::rolled_secret;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn verifies_an_access_token_signed_with_the_retired_key() {
    let secret = fresh_secret(SigningCurve::P256);
    let signed = rolled_store(secret.clone())
        .await
        .sign_access_token(&json!({ "name": "demo" }), unix_time(500))
        .expect("the claims are signed");
    let rolled = rolled_store(rolled_secret(&secret)).await;

    assert!(matches!(
        rolled.verify_access_token::<Map<String, Value>>(&signed.signed_claims, unix_time(500)),
        JwtVerification::Verified(verified) if verified.kid.as_ref() == Some(secret.current().kid())
    ));
}
