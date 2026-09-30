use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn refuses_malformed_credentials_as_an_invalid_request() {
    let trusted_issuer = TrustedIssuer::for_oidc_issuer(
        Arc::new(IssuerMetadata::awaiting()),
        Arc::new(fixture_trust()),
    );

    assert!(refused_with_challenge(
        &admit_access_token(&trusted_issuer, "Bearer").await,
        400,
        "Bearer error=\"invalid_request\""
    ));
}
