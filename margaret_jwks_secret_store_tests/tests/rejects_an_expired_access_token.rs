use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwt_verification::claims_rejection::ClaimsRejection;
use margaret_jwt_verification::jwt_rejection::JwtRejection;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn rejects_an_expired_access_token() {
    let store = rolled_store(fresh_p256_secret()).await;
    let signed = store
        .sign_access_token(&json!({ "name": "demo" }), unix_time(500))
        .expect("the claims are signed");

    assert!(matches!(
        store.verify_access_token::<Map<String, Value>>(
            &signed.signed_claims,
            unix_time(500 + i64::from(ACCESS_TOKEN_LIFETIME_SECS))
        ),
        JwksSecretVerificationResult::Rejected(JwtRejection::Claims(
            ClaimsRejection::Expired { .. }
        ))
    ));
}
