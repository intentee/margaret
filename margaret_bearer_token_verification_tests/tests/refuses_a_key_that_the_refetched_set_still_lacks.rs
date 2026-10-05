use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test(start_paused = true)]
async fn refuses_a_key_that_the_refetched_set_still_lacks() {
    let published = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a published secret");
    let unpublished = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("an unpublished secret");
    let trusted_issuer = held_trusted_issuer(fixture_trust(), published.key_set().clone());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(unpublished.current());

    let authorization = format!("Bearer {token}");

    tokio::time::advance(ISSUER_FETCH_SPACING).await;

    let (admission, ()) =
        tokio::join!(admit_access_token(&trusted_issuer, &authorization), async {
            trusted_issuer.key_set.refresh_requested().await;
            trusted_issuer.key_set.start_fetch();
            trusted_issuer
                .key_set
                .hold(Arc::new(published.key_set().clone()));
        });

    assert!(refused_with_challenge(
        &admission,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
