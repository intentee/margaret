use std::sync::Arc;

use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity::routes::internal::post_mint::PostMint;
use margaret_identity_tests::fixed_clock::FixedClock;
use margaret_identity_tests::peer::peer;
use margaret_token_signer_tests::unix_time::unix_time;
use margaret_validation::malformation::Malformation;
use margaret_validation::validation_result::ValidationResult;

#[tokio::test]
async fn post_mint_rejects_a_malformed_request() {
    let clock = Arc::new(FixedClock::new(unix_time(1_000)));
    let store = Arc::new(SigningKeyStore::create());
    let route = PostMint::create(clock, store);

    let response = route
        .respond(
            &peer(),
            ValidationResult::Malformed(Malformation::Unreadable),
        )
        .await;

    assert_eq!(response.status(), 400);
}
