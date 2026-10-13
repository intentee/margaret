use serde_json::Value;
use serde_json::json;

use margaret_jose_parameters::jwt_type::JwtType;
use margaret_jwks_keygen_tests::far_future_expiry::FAR_FUTURE_EXPIRY;
use margaret_jwks_secret_store::jwks_secret_store::JwksSecretStore;
use margaret_jwks_secret_store_tests::fixture_issuance::fixture_issuance;
use margaret_jwks_secret_store_tests::peer_secrets::PeerSecrets;
use margaret_jwt_verification::attribute_serialized_jwt::attribute_serialized_jwt;
use margaret_jwt_verification::client_assertion_profile::ClientAssertionProfile;
use margaret_jwt_verification::expected_audience::ExpectedAudience;
use margaret_jwt_verification::jwt_expectation::JwtExpectation;
use margaret_jwt_verification::jwt_verification::JwtVerification;
use margaret_registered_claims::numeric_date::NumericDate;

const CLIENT: &str = "portal";
const TOKEN_ENDPOINT: &str = "https://issuer.example/token";

#[test]
fn verifies_an_own_jwt_signed_by_a_peer_one_generation_ahead() {
    let peers = PeerSecrets::one_generation_apart();
    let assertion = peers.rolled.get().current().sign_json(
        &json!({
            "aud": TOKEN_ENDPOINT,
            "exp": FAR_FUTURE_EXPIRY,
            "iss": CLIENT,
            "jti": "assertion",
            "sub": CLIENT,
        }),
        JwtType::ClientAuthentication,
    );
    let attributed = attribute_serialized_jwt(
        &assertion,
        &JwtExpectation {
            audience: ExpectedAudience::Sole(TOKEN_ENDPOINT),
            issuer: CLIENT,
        },
    )
    .continue_value()
    .expect("the assertion is addressed to the token endpoint");

    assert!(matches!(
        JwksSecretStore::create(peers.lagging, fixture_issuance())
            .verify_own_jwt::<Value, ClientAssertionProfile>(&attributed, NumericDate::new(0)),
        JwtVerification::Verified(_)
    ));
}
