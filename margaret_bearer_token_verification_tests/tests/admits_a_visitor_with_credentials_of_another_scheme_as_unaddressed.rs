use std::sync::Arc;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_http::token_admission::TokenAdmission;
use margaret_issuer_key_set::issuer_key_set::IssuerKeySet;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn admits_a_visitor_with_credentials_of_another_scheme_as_unaddressed() {
    let trusted_issuer = TrustedIssuer::create(Arc::new(IssuerKeySet::awaiting()), fixture_trust());

    assert!(matches!(
        admit_access_token(&trusted_issuer, "Basic dXNlcjpwYXNz").await,
        TokenAdmission::Unaddressed
    ));
}
