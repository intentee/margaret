use serde_json::Value;
use serde_json::json;

use margaret_bearer_token_verification::bearer_token_verification::BearerTokenVerification;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_oidc_client_tests::discovered_issuer::DiscoveredIssuer;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_client_tests::signed_id_token::SignedIdToken;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test(flavor = "multi_thread")]
async fn oidc_client_verifies_an_rs256_token_of_the_discovered_issuer() {
    let key = FixtureRsaKey::load("rsa-kid");
    let issuer = DiscoveredIssuer::start(&key).await;
    let token = SignedIdToken {
        audience: json!("margaret"),
        exp: 9_999_999_999,
        typ: "JWT",
    }
    .signed_by(&key, &localhost_trust());

    let BearerTokenVerification::Verified(identity) = issuer.verifier.verify::<Value>(
        &RequestAuthorization::parse(Some(&format!("Bearer {token}"))),
        NumericDate::new(1_700_000_000),
    ) else {
        panic!("the token of the discovered issuer verifies");
    };

    assert_eq!(identity.claims["role"], "builder");

    issuer.stop().await;
}
