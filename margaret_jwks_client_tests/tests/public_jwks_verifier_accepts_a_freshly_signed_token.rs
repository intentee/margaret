use std::sync::Arc;

use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_claims_expiring_at::test_claims_expiring_at;
use margaret_jwks_client_tests::test_expected_claims::test_expected_claims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn public_jwks_verifier_accepts_a_freshly_signed_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let claims = test_claims_expiring_at(1_700_000_060);
    let token = secret
        .current
        .signing
        .sign(&claims)
        .await
        .expect("the claims sign");

    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(PublicJwks::from(secret))));

    let verification = PublicJwksVerifier::new(test_expected_claims(), holder)
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("the freshly signed token verifies");

    let AccessTokenVerification::Verified(verified) = verification else {
        panic!("the freshly signed token verifies");
    };

    assert_eq!(verified, claims);
}
