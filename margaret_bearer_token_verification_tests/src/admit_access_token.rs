use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

/// # Panics
///
/// Panics when the system clock cannot be read as a numeric date.
pub async fn admit_access_token(
    trusted_issuer: &TrustedIssuer,
    authorization: &str,
) -> BearerTokenAdmission<TestClaims, AccessTokenProfile> {
    let authorization = RequestAuthorization::parse(Some(authorization));

    match route_bearer_token(&authorization, &[trusted_issuer])
        .expect("the system clock reads as a numeric date")
    {
        BearerTokenRouting::Refused(continuation) => BearerTokenAdmission::Refused(continuation),
        BearerTokenRouting::Routed(routed) => routed.admit(trusted_issuer).await,
    }
}
