use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test]
async fn refuses_an_access_token_under_the_id_token_profile() {
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let trusted_issuer = held_trusted_issuer(fixture_trust(), secret.key_set().clone());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) = route_bearer_token(&authorization, &[&trusted_issuer])
        .expect("the system clock reads as a numeric date")
    else {
        panic!("the token routes to its issuer");
    };

    let admission: BearerTokenAdmission<TestClaims, IdTokenProfile> =
        routed.admit(&trusted_issuer).await;

    assert!(refused_with_challenge(
        &admission,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
