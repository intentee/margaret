use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::token_admission::TokenAdmission;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_token_trust::token_trust::TokenTrust;

#[tokio::test]
async fn admits_a_token_through_the_declaration_of_its_audience() {
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let addressed = fixture_trust();
    let elsewhere = held_trusted_issuer(
        TokenTrust {
            audience: "elsewhere",
            issuer: addressed.issuer,
        },
        secret.key_set().clone(),
    );
    let audience = held_trusted_issuer(addressed, secret.key_set().clone());
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) =
        route_bearer_token(&authorization, &[&elsewhere, &audience])
            .expect("the system clock reads as a numeric date")
    else {
        panic!("the token routes to the declaration of its audience");
    };

    assert!(matches!(
        routed
            .admit::<TestClaims, AccessTokenProfile>(&elsewhere)
            .await,
        TokenAdmission::Unaddressed
    ));
    assert!(matches!(
        routed
            .admit::<TestClaims, AccessTokenProfile>(&audience)
            .await,
        TokenAdmission::Admitted(_)
    ));
}
