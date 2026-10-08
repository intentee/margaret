use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test(start_paused = true)]
async fn refuses_a_token_signed_by_a_key_of_a_recently_fetched_set_without_refetching() {
    let published = fresh_secret(SigningCurve::P256);
    let unpublished = fresh_secret(SigningCurve::P256);
    let trusted_issuer =
        held_trusted_issuer(fixture_trust(), published.published_key_set().clone());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(unpublished.current());

    assert!(refused_with_challenge(
        &admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
