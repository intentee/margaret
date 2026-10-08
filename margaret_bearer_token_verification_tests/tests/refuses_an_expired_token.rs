use serde_json::json;

use margaret_bearer_token_verification_tests::admit_access_token::admit_access_token;
use margaret_bearer_token_verification_tests::held_trusted_issuer::held_trusted_issuer;
use margaret_bearer_token_verification_tests::refused_with_challenge::refused_with_challenge;
use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test]
async fn refuses_an_expired_token() {
    let trust = fixture_trust();
    let secret = fresh_secret(SigningCurve::P256);
    let token = secret.current().sign_json(
        &json!({
            "aud": trust.audience,
            "exp": 1,
            "iat": 0,
            "iss": trust.issuer,
            "sub": "subject",
        }),
        JwtType::AccessToken,
    );
    let trusted_issuer = held_trusted_issuer(trust, secret.published_key_set().clone());

    assert!(refused_with_challenge(
        &admit_access_token(&trusted_issuer, &format!("Bearer {token}")).await,
        401,
        "Bearer error=\"invalid_token\""
    ));
}
