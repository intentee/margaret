use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_a_token_that_is_not_a_jws() {
    let trusted_issuer = TrustedIssuer::for_oidc_issuer(
        Arc::new(IssuerMetadata::awaiting()),
        Arc::new(fixture_trust()),
    );

    assert!(refused_with_challenge(
        &admit_access_token(&trusted_issuer, "Bearer not-a-jws").await,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
