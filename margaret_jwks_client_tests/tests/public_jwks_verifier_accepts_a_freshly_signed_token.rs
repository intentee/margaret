use std::sync::Arc;

use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn public_jwks_verifier_accepts_a_freshly_signed_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let claims = TestClaims {
        exp: 1_700_000_060,
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

    let verified = PublicJwksVerifier::new(holder)
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("the freshly signed token verifies");

    assert_eq!(verified, claims);
}
