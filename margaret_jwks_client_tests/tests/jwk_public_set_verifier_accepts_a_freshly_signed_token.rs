use std::sync::Arc;

use margaret_jwks_client::jwk_public_set_holder::JwkPublicSetHolder;
use margaret_jwks_client::jwk_public_set_verifier::JwkPublicSetVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn jwk_public_set_verifier_accepts_a_freshly_signed_token() {
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

    let holder = JwkPublicSetHolder::default();

    holder.set(Some(Arc::new(JwkPublicSet::from(secret))));

    let verified = JwkPublicSetVerifier::new(holder)
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("the freshly signed token verifies");

    assert_eq!(verified, claims);
}
