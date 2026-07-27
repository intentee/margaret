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
use margaret_jwks_keygen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn public_jwks_verifier_rejects_a_token_signed_by_an_unrelated_key() {
    let mut published = JwksSecret::fresh(Curve::P256).expect("a published secret");
    let stranger = JwksSecret::fresh(Curve::P256).expect("an unrelated secret");
    let claims = TestClaims {
        exp: 1_700_000_060,
        sub: "subject".to_string(),
    };
    let token = stranger
        .current
        .signing
        .sign(&claims)
        .await
        .expect("the claims sign");
    published.current.public.kid = stranger.current.public.kid.clone();

    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(PublicJwks::from(published))));

    let outcome = PublicJwksVerifier::new(holder)
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("a foreign token is an expected validation outcome");

    assert_eq!(
        outcome,
        PublicTokenVerification::Rejected(TokenRejection::Invalid)
    );
}
