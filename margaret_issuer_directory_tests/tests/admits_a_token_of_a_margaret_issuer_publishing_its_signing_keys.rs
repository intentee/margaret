use std::sync::Arc;

use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::route_entry::RouteEntry;
use margaret_http::token_admission::TokenAdmission;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer;
use margaret_issuer_directory_tests::polled_fixture::PolledFixture;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_keygen_tests::fresh_secret::fresh_secret;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::jwks_curve::JWKS_CURVE;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test(flavor = "multi_thread")]
async fn admits_a_token_of_a_margaret_issuer_publishing_its_signing_keys() {
    let fixture = TlsFixture::generate();
    let secret = fresh_secret(JWKS_CURVE);
    let jwks_server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            WELL_KNOWN_JWKS_PATH,
            vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(StaticHandler {
                    body: serde_json::to_vec(secret.public_jwks())
                        .expect("the public key set serializes"),
                    content_type: "application/jwk-set+json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let polled = PolledFixture::published(JwksEndpointIssuer {
        issuer: fixture_trust().issuer,
        jwks_uri: String::leak(
            fixture
                .url(jwks_server.port(), WELL_KNOWN_JWKS_PATH)
                .to_string(),
        ),
    });

    polled
        .first_poll(
            IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
                .expect("the fixture request client builds"),
        )
        .await;

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

    jwks_server.stop().await;
}
