use std::sync::Arc;

use serde_json::json;

use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::token_admission::TokenAdmission;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer;
use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_jwks_endpoint_issuer::LOCALHOST_JWKS_ENDPOINT_ISSUER;
use margaret_issuer_directory_tests::polled_fixture::PolledFixture;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;

#[tokio::test(flavor = "multi_thread")]
async fn admits_a_token_of_a_margaret_issuer_publishing_its_signing_keys() {
    let secret = fresh_secret(JWKS_CURVE);
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(404, &json!({})),
        key_set: Arc::new(StaticHandler {
            body: serde_json::to_vec(secret.public_jwks()).expect("the public key set serializes"),
            content_type: "application/jwk-set+json",
            status: 200,
        }),
    })
    .await;
    let polled = PolledFixture::published(JwksEndpointIssuer {
        issuer: fixture_trust().issuer,
        jwks_uri: LOCALHOST_JWKS_ENDPOINT_ISSUER.jwks_uri,
    });

    polled.first_poll(issuer.request_client()).await;

    let trusted_issuer = polled.trusted(fixture_trust());

    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let token = claims.signed_by(secret.current());
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) = route_bearer_token(&authorization, &[&trusted_issuer])
        .expect("the system clock reads as a numeric date")
    else {
        panic!("the token routes to the margaret issuer");
    };
    let TokenAdmission::Admitted(verified) = routed
        .admit::<TestClaims, AccessTokenProfile>(&trusted_issuer)
        .await
    else {
        panic!("the token of the margaret issuer is admitted");
    };

    assert_eq!(verified.claims, claims);

    issuer.stop().await;
}
