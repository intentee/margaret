use std::sync::Arc;

use serde_json::Value;
use serde_json::json;

use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_discovery::localhost_discovery;
use margaret_issuer_directory_tests::localhost_oidc_issuer::localhost_oidc_issuer;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_jwt_verification::id_token_profile::IdTokenProfile;

#[tokio::test(flavor = "multi_thread")]
async fn admits_a_token_of_a_discovered_issuer() {
    let key = FixtureRsaKey::load("rsa-kid");
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(200, &localhost_discovery()),
        key_set: json_handler(200, &json!({ "keys": [key.jwk()] })),
    })
    .await;
    let trusted_issuer = localhost_oidc_issuer();
    let snapshot = trusted_issuer.key_set.snapshot();
    let directory =
        PolledDirectory::start(vec![Arc::clone(&trusted_issuer)], issuer.request_client());

    trusted_issuer.key_set.refreshed_since(&snapshot).await;

    let token = key.token(
        &key.header(),
        &json!({
            "aud": "margaret",
            "exp": 9_999_999_999_i64,
            "iat": 1_700_000_000,
            "iss": "https://localhost",
            "role": "builder",
        }),
    );
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) =
        route_bearer_token(&authorization, &[trusted_issuer.as_ref()])
            .expect("the system clock reads as a numeric date")
    else {
        panic!("the token routes to the discovered issuer");
    };
    let admission = routed.admit::<Value, IdTokenProfile>(&trusted_issuer).await;

    directory.stop().await;
    issuer.stop().await;

    let BearerTokenAdmission::Admitted(verified) = admission else {
        panic!("the token of the discovered issuer is admitted");
    };

    assert_eq!(verified.claims["role"], "builder");
}
