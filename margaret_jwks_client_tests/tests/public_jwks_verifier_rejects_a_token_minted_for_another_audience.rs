use std::sync::Arc;

use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client::public_jwks_holder::PublicJwksHolder;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_expected_claims::test_expected_claims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_client_tests::test_issuer::TEST_ISSUER;
use margaret_jwks_keygen::curve::Curve;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwt_claims::audience::Audience;

#[tokio::test]
async fn public_jwks_verifier_rejects_a_token_minted_for_another_audience() {
    let secret = JwksSecret::fresh(Curve::P256).expect("a fresh secret");
    let token = secret
        .current
        .signing
        .sign(&TestClaims {
            aud: Audience::Many(vec!["another-service".to_string()]),
            exp: 1_700_000_060,
            iss: TEST_ISSUER.to_string(),
            sub: "subject".to_string(),
        })
        .await
        .expect("the claims sign");
    let holder = PublicJwksHolder::default();

    holder.set(Some(Arc::new(PublicJwks::from(secret))));

    let verification = PublicJwksVerifier::new(test_expected_claims(), holder)
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("the published jwks is usable");

    assert!(matches!(
        verification,
        AccessTokenVerification::AudienceMismatch
    ));
}
