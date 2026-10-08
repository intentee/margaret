use serde_json::json;

use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[tokio::test]
async fn signs_access_tokens_for_the_access_token_lifetime() {
    let signed = rolled_store(fresh_p256_secret())
        .await
        .sign_access_token(&json!({ "name": "demo" }), unix_time(500))
        .expect("the claims are signed");

    assert_eq!(signed.exp, 500 + i64::from(ACCESS_TOKEN_LIFETIME_SECS));
}
