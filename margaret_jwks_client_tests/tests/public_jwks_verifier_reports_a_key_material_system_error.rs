use std::sync::Arc;

use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn public_jwks_verifier_reports_a_key_material_system_error() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = secret
        .current
        .signing
        .sign(&TestClaims {
            exp: 1_700_000_060,
            sub: "subject".to_string(),
        })
        .await
        .expect("the claims sign");
    let mut published = PublicJwks::from(secret);

    published.keys[0].x = "not-base64url".to_string();

    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(published)));

    assert!(matches!(
        PublicJwksVerifier::new(holder).verify::<TestClaims>(&token, test_instant(1_700_000_000)),
        Err(JwksClientError::TokenVerification(_))
    ));
}
