use std::sync::Arc;

use margaret_identity::forms::mint_request::MintRequest;
use margaret_identity::routes::internal::post_mint::PostMint;
use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity_tests::fixed_clock::FixedClock;
use margaret_identity_tests::peer::peer;
use margaret_token_signer_tests::unix_time::unix_time;
use margaret_validation::validation_result::ValidationResult;

#[tokio::test]
async fn post_mint_returns_503_when_the_keys_are_not_ready() {
    let clock = Arc::new(FixedClock::new(unix_time(1_000)));
    let store = Arc::new(SigningKeyStore::create());
    let route = PostMint::create(clock, store);

    let response = route
        .respond(
            &peer(),
            ValidationResult::Valid(MintRequest {
                refresh_token: "a.refresh.token".to_string(),
            }),
        )
        .await;

    assert_eq!(response.status(), 503);
}
