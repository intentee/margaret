use serde_json::json;

use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_token_trust::token_trust::TokenTrust;

#[test]
fn refuses_a_token_addressed_to_two_declarations() {
    let secret = fresh_secret(SigningCurve::P256);
    let trust = fixture_trust();
    let token = secret.current().sign_json(
        &json!({
            "aud": [trust.audience, "elsewhere"],
            "exp": FAR_FUTURE_EXPIRY,
            "iat": 0,
            "iss": trust.issuer,
            "sub": "subject",
        }),
        JwtType::AccessToken,
    );
    let elsewhere = held_trusted_issuer(
        TokenTrust {
            audience: "elsewhere",
            issuer: trust.issuer,
        },
        secret.published_key_set().clone(),
    );
    let addressed = held_trusted_issuer(trust, secret.published_key_set().clone());
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));

    assert!(matches!(
        route_bearer_token(&authorization, &[&addressed, &elsewhere]),
        Ok(BearerTokenRouting::Refused(ResponseContinuation::Done(response)))
            if response.status() == 401
    ));
}
