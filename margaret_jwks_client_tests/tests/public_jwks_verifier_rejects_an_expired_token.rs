use std::sync::Arc;

use margaret_identity_session::claims_rejection::ClaimsRejection;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_audience::TEST_AUDIENCE;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_client_tests::test_issuer::TEST_ISSUER;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;

#[tokio::test]
async fn public_jwks_verifier_rejects_an_expired_token() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let now = test_instant(1_700_000_000);
    let claims = TestClaims {
        aud: TEST_AUDIENCE.to_string(),
        exp: 1_699_999_999,
        iss: TEST_ISSUER.to_string(),
        nbf: 0,
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

    let verification = PublicJwksVerifier::new(holder)
        .verify::<TestClaims>(&token, now)
        .expect("the published jwks is usable");

    assert!(matches!(
        verification,
        AccessTokenVerification::Rejected(ClaimsRejection::Expired)
    ));
}
