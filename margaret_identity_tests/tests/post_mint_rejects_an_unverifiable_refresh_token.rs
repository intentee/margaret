use std::sync::Arc;

use margaret_identity::forms::mint_request::MintRequest;
use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity::routes::internal::post_mint::PostMint;
use margaret_identity_tests::fixed_clock::FixedClock;
use margaret_identity_tests::peer::peer;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_token_signer_tests::unix_time::unix_time;
use margaret_validation::validation_result::ValidationResult;

#[tokio::test]
async fn post_mint_rejects_an_unverifiable_refresh_token() {
    let clock = Arc::new(FixedClock::new(unix_time(1_000)));
    let store = Arc::new(SigningKeyStore::create());
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret is generated");
    store.holder().set(Some(Arc::new(secret)));
    let route = PostMint::create(clock, store);

    let response = route
        .respond(
            &peer(),
            ValidationResult::Valid(MintRequest {
                refresh_token: "not.a.valid.token".to_string(),
            }),
        )
        .await;

    assert_eq!(response.status(), 401);
}
