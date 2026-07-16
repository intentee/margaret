use std::sync::Arc;

use zeroize::Zeroizing;

use margaret_identity::forms::mint_request::MintRequest;
use margaret_identity::stores::signing_key_store::SigningKeyStore;
use margaret_identity::routes::internal::post_mint::PostMint;
use margaret_identity_tests::fixed_clock::FixedClock;
use margaret_identity_tests::peer::peer;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_token_signer_tests::refresh_claims::refresh_claims;
use margaret_token_signer_tests::sign_refresh_token::sign_refresh_token;
use margaret_token_signer_tests::unix_time::unix_time;
use margaret_validation::validation_result::ValidationResult;

#[tokio::test]
async fn post_mint_reports_a_signing_failure() {
    let mut secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret is generated");
    let refresh_token = sign_refresh_token(&secret.current.signing, &refresh_claims(10_000)).await;
    secret.current.signing.pem = Zeroizing::new("not a valid pkcs8 pem".to_string());
    let store = Arc::new(SigningKeyStore::create());
    store.holder().set(Some(Arc::new(secret)));
    let clock = Arc::new(FixedClock::new(unix_time(1_000)));
    let route = PostMint::create(clock, store);

    let response = route
        .respond(
            &peer(),
            ValidationResult::Valid(MintRequest { refresh_token }),
        )
        .await;

    assert_eq!(response.status(), 500);
}
