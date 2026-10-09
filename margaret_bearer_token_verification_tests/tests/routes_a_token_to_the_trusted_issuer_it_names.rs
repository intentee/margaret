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
use margaret_token_trust::token_trust::TokenTrust;

#[tokio::test]
async fn routes_a_token_to_the_trusted_issuer_it_names() {
    let secret = fresh_secret(SigningCurve::P256);
    let other = held_trusted_issuer(
        TokenTrust {
            audience: "margaret",
            issuer: "https://other.example",
        },
        secret.published_key_set().clone(),
    );
    let named = held_trusted_issuer(fixture_trust(), secret.published_key_set().clone());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) = route_bearer_token(&authorization, &[&other, &named])
        .expect("the system clock reads as a numeric date")
    else {
        panic!("the token routes to the issuer it names");
    };

    assert!(matches!(
        routed.admit::<TestClaims, AccessTokenProfile>(&other).await,
        TokenAdmission::Unaddressed
    ));
    assert!(matches!(
        routed.admit::<TestClaims, AccessTokenProfile>(&named).await,
        TokenAdmission::Admitted(_)
    ));
}
