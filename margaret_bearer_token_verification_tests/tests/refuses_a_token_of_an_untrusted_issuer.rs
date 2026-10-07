use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_token_trust::token_trust::TokenTrust;

#[test]
fn refuses_a_token_of_an_untrusted_issuer() {
    let secret = JwksSecret::fresh(SigningCurve::P256, &FixtureRsaSigningKeys::default())
        .expect("a fresh secret");
    let trusted_issuer = held_trusted_issuer(
        TokenTrust {
            audience: "margaret",
            issuer: "https://other.example",
        },
        secret.key_set().clone(),
    );
    let token = TestClaims {
        sub: "subject".to_string(),
    }
    .signed_by(secret.current());
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));

    assert!(matches!(
        route_bearer_token(&authorization, &[&trusted_issuer]),
        Ok(BearerTokenRouting::Refused(ResponseContinuation::Done(response)))
            if response.status() == 401
    ));
}
