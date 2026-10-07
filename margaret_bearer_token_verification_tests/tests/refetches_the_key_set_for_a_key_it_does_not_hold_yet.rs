use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_key_set::held_key_set;
use margaret_http::token_admission::TokenAdmission;
use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(start_paused = true)]
async fn refetches_the_key_set_for_a_key_it_does_not_hold_yet() {
    let rotated_out = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a rotated-out secret");
    let rotated_in = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a rotated-in secret");
    let key_set = held_key_set(rotated_out.key_set().clone());
    let trusted_issuer = TrustedIssuer::create(Arc::clone(&key_set), fixture_trust());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(rotated_in.current());

    let authorization = format!("Bearer {token}");

    tokio::time::advance(ISSUER_FETCH_SPACING).await;

    let (admission, ()) =
        tokio::join!(admit_access_token(&trusted_issuer, &authorization), async {
            key_set.refresh_requested().await;
            key_set.start_fetch();
            key_set.hold(Arc::new(rotated_in.key_set().clone()));
        });

    assert!(matches!(admission, TokenAdmission::Admitted(_)));
}
