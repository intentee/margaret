use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_key_set::held_key_set;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(start_paused = true)]
async fn refuses_a_key_that_the_refetched_set_still_lacks() {
    let published = fresh_secret(SigningCurve::P256);
    let unpublished = fresh_secret(SigningCurve::P256);
    let key_set = held_key_set(published.published_key_set().clone());
    let trusted_issuer = TrustedIssuer::polled(Arc::clone(&key_set), fixture_trust());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(unpublished.current());

    let authorization = format!("Bearer {token}");

    tokio::time::advance(ISSUER_FETCH_SPACING).await;

    let (admission, ()) =
        tokio::join!(admit_access_token(&trusted_issuer, &authorization), async {
            key_set.refresh_requested().await;
            key_set.start_fetch();
            key_set.hold(Arc::new(published.published_key_set().clone()));
        });

    assert!(refused_with_challenge(
        &admission,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
