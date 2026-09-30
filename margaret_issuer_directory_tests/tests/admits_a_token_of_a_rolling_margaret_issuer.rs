use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use trzcina::ServiceBundle as _;

use margaret_bearer_token_verification::bearer_token_admission::BearerTokenAdmission;
use margaret_bearer_token_verification::bearer_token_routing::BearerTokenRouting;
use margaret_bearer_token_verification::route_bearer_token::route_bearer_token;
use margaret_http::method_handler::MethodHandler;
use margaret_http::request_authorization::RequestAuthorization;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
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
use margaret_route_method::route_method::RouteMethod;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(flavor = "multi_thread")]
async fn admits_a_token_of_a_rolling_margaret_issuer() {
    let fixture = TlsFixture::generate();
    let server_bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        rsa_keys: Arc::new(FixtureRsaSigningKeys::default()),
        storage: Arc::new(MemoryJwksSecretStorage),
    });
    let public_jwks_handler = server_bundle.public_jwks_handler();
    let jwks_document_holder = server_bundle.jwks_document_holder();
    let jwks_secret_holder = server_bundle.jwks_secret_holder();
    let roll_service = server_bundle
        .services()
        .await
        .expect("the server bundle exposes services")
        .pop()
        .expect("the server bundle exposes the roll service");
    let cancellation_token = CancellationToken::new();
    let roll_task = tokio::spawn(roll_service.run(cancellation_token.clone()));

    assert_eq!(
        jwks_document_holder
            .subscribe()
            .wait_until_present(&cancellation_token)
            .await,
        SyncHolderPresence::Present
    );

    let jwks_server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            WELL_KNOWN_JWKS_PATH,
            vec![MethodHandler::anonymous(
                RouteMethod::Get,
                public_jwks_handler,
            )],
        )],
    )
    .await;
    let trusted_issuer = Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(StaticEndpoint::new(
            fixture.url(jwks_server.port(), WELL_KNOWN_JWKS_PATH),
        )),
        Arc::new(fixture_trust()),
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
    let token = claims.signed_by(
        jwks_secret_holder
            .get()
            .expect("the first roll published")
            .current(),
    );
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let BearerTokenRouting::Routed(routed) =
        route_bearer_token(&authorization, &[trusted_issuer.as_ref()])
            .expect("the system clock reads as a numeric date")
    else {
        panic!("the token routes to the rolling issuer");
    };
    let BearerTokenAdmission::Admitted(verified) = routed
        .admit::<TestClaims, AccessTokenProfile>(&trusted_issuer)
        .await
    else {
        panic!("the token of the rolling issuer is admitted");
    };

    assert_eq!(verified.claims, claims);

    cancellation_token.cancel();
    roll_task
        .await
        .expect("the roll task joins")
        .expect("the roll service shuts down cleanly");
    jwks_server.stop().await;
}
