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
async fn public_jwks_verifier_rejects_an_expired_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let now = test_instant(1_700_000_000);
    let claims = TestClaims {
        exp: 1_699_999_999,
        sub: "subject".to_string(),
    };
    let token = secret
        .current
        .signing
        .sign(&claims)
        .await
        .expect("the claims sign");

    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(PublicJwks::from(secret))));

    let outcome = PublicJwksVerifier::new(holder)
        .verify::<TestClaims>(&token, now)
        .expect("an expired token is an expected validation outcome");

    assert_eq!(
        outcome,
        PublicTokenVerification::Rejected(TokenRejection::Expired)
    );
}
