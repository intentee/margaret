use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use margaret_jwks_keygen::jwks_secret_verification_result::JwksSecretVerificationResult;
use margaret_jwks_secret_store::access_token_signing::AccessTokenSigning;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_signer_tests::unix_time::unix_time;

#[derive(Deserialize)]
struct TokenIdentifier {
    jti: Uuid,
}

fn signed_token_identifier(store: &JwksSecretStore) -> Uuid {
    let Ok(AccessTokenSigning::Signed(signed)) =
        store.sign_access_token(&json!({ "name": "demo" }), unix_time(500))
    else {
        panic!("the claims are signed");
    };
    let Ok(JwksSecretVerificationResult::SignedWithCurrent(verified)) =
        store.verify_access_token::<TokenIdentifier>(&signed.signed_claims, unix_time(500))
    else {
        panic!("the signed token carries a token identifier");
    };

    verified.claims.jti
}

#[test]
fn identifies_every_access_token() {
    let store = rolled_store(fresh_p256_secret());

    assert_ne!(
        signed_token_identifier(&store),
        signed_token_identifier(&store)
    );
}
