use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::token_admission::TokenAdmission;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test]
async fn admits_a_visitor_without_bearer_credentials_as_unaddressed() {
    let secret = fresh_secret(SigningCurve::P256);
    let trusted_issuer = held_trusted_issuer(fixture_trust(), secret.published_key_set().clone());
    let authorization = RequestAuthorization::parse(None);
    let BearerTokenRouting::Routed(routed) = route_bearer_token(&authorization, &[&trusted_issuer])
        .expect("the system clock reads as a numeric date")
    else {
        panic!("an absent credential routes");
    };

    assert!(matches!(
        routed
            .admit::<TestClaims, AccessTokenProfile>(&trusted_issuer)
            .await,
        TokenAdmission::Unaddressed
    ));
}
