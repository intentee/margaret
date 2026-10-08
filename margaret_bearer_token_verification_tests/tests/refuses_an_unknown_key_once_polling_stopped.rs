use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_key_set::held_key_set;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(start_paused = true)]
async fn refuses_an_unknown_key_once_polling_stopped() {
    let published = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a published secret");
    let unpublished = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("an unpublished secret");
    let key_set = held_key_set(published.key_set().clone());
    let trusted_issuer = TrustedIssuer::polled(Arc::clone(&key_set), fixture_trust());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(unpublished.current());

    tokio::time::advance(ISSUER_FETCH_SPACING).await;
    key_set.stop_polling();

    assert!(refused_with_challenge(
        &admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
