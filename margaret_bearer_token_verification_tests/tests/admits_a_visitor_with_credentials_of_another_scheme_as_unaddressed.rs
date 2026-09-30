use std::sync::Arc;

use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn admits_a_visitor_with_credentials_of_another_scheme_as_unaddressed() {
    let trusted_issuer = TrustedIssuer::for_oidc_issuer(
        Arc::new(IssuerMetadata::awaiting()),
        Arc::new(fixture_trust()),
    );

    assert!(matches!(
        admit_access_token(&trusted_issuer, "Basic dXNlcjpwYXNz").await,
        BearerTokenAdmission::Unaddressed
    ));
}
