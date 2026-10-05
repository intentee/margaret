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
use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwks_keygen_tests::fixture_rsa_signing_keys::FixtureRsaSigningKeys;
use margaret_jwks_keygen_tests::test_claims::TestClaims;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jwt_verification::access_token_profile::AccessTokenProfile;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_route_method::route_method::RouteMethod;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(flavor = "multi_thread")]
async fn admits_a_token_of_a_rolling_margaret_issuer() {
    let fixture = TlsFixture::generate();
    let server_bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(MemoryJwksSecretStorage),
    })
    .expect("the first secret is rolled and published");
    let jwks_server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            WELL_KNOWN_JWKS_PATH,
            vec![MethodHandler::head(
                RouteMethod::Get,
                Arc::new(StaticHandler {
                    body: server_bundle.jwks_document_holder().get().to_vec(),
                    content_type: "application/jwk-set+json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let trusted_issuer = Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(StaticEndpoint::new(
            fixture.url(jwks_server.port(), WELL_KNOWN_JWKS_PATH),
        )),
        Arc::new(TokenTrustDeclaration {
            trust: fixture_trust(),
        }),
    ));

    first_poll(
        &trusted_issuer,
        IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
            .expect("the fixture request client builds"),
    )
    .await;

    let claims = TestClaims {
        sub: "subject".to_string(),
    };
    let token = claims.signed_by(server_bundle.jwks_secret_holder().get().current());
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) =
        route_bearer_token(&authorization, &[trusted_issuer.as_ref()])
            .expect("the system clock reads as a numeric date")
    else {
        panic!("the token routes to the rolling issuer");
    };
    let TokenAdmission::Admitted(verified) = routed
        .admit::<TestClaims, AccessTokenProfile>(&trusted_issuer)
        .await
    else {
        panic!("the token of the rolling issuer is admitted");
    };

    assert_eq!(verified.claims, claims);

    jwks_server.stop().await;
}
