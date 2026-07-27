use std::sync::Arc;

use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client::public_token_verification::PublicTokenVerification;
use margaret_jwks_client::token_rejection::TokenRejection;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;

#[test]
fn public_jwks_verifier_rejects_a_malformed_token() {
    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(PublicJwks::from(
        JwksSecret::fresh(Curve::P256).expect("a fresh secret"),
    ))));

    let outcome = PublicJwksVerifier::new(holder)
        .verify::<TestClaims>("not-a-token", test_instant(1_700_000_000))
        .expect("malformed user input is an expected outcome");

    assert_eq!(
        outcome,
        PublicTokenVerification::Rejected(TokenRejection::Invalid)
    );
}
