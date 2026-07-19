use std::sync::Arc;

use margaret_jwks_client::jwk_public_set_holder::JwkPublicSetHolder;
use margaret_jwks_client::jwk_public_set_verifier::JwkPublicSetVerifier;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_key_gen::curve::Curve;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_jwks_key_gen::jwks_secret::JwksSecret;
use margaret_jwks_key_gen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn jwk_public_set_verifier_rejects_an_expired_token() {
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

    let holder = JwkPublicSetHolder::default();

    holder.set(Some(Arc::new(JwkPublicSet::from(secret))));

    let Err(error) = JwkPublicSetVerifier::new(holder).verify::<TestClaims>(&token, now) else {
        panic!("an expired token never verifies even when its signature is valid");
    };

    assert!(matches!(error, JwksClientError::TokenExpired));
    assert_eq!(
        error.to_string(),
        "the token expired before it was verified"
    );
}
