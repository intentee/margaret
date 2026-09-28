use std::sync::Arc;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceBundle as _;

use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_jwks_client::jwks_client::JwksClient;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwks_keygen::jwks_secret::JwksSecret;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::jwks_roller_server_bundle::JwksRollerServerBundle;
use margaret_jwks_roller_server::jwks_roller_server_bundle_params::JwksRollerServerBundleParams;
use margaret_jwt_verification_tests::fixture_trust::fixture_trust;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

pub struct PolledJwksIssuer {
    cancellation_token: CancellationToken,
    jwks_server: RunningFixtureServer,
    poll_task: JoinHandle<anyhow::Result<()>>,
    roll_task: JoinHandle<anyhow::Result<()>>,
    pub secret: Arc<JwksSecret>,
    pub verifier: Arc<BearerTokenVerifier>,
}

impl PolledJwksIssuer {
    /// # Panics
    ///
    /// Panics when the issuer cannot roll its secret, or the client cannot poll its document.
    pub async fn start() -> Self {
        let fixture = TlsFixture::generate();
        let server_bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
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
        let roll_token = cancellation_token.clone();
        let roll_task = tokio::spawn(async move { roll_service.run(roll_token).await });

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
                vec![MethodHandler::anonymous("GET", public_jwks_handler)],
            )],
        )
        .await;
        let client_builder = fixture_client_builder(&fixture.certificate_authority);
        let jwks_client = JwksClient::create(
            Arc::new(StaticEndpoint::new(
                fixture.url(jwks_server.port(), WELL_KNOWN_JWKS_PATH),
            )),
            Arc::new(fixture_trust()),
        );
        let verifier = jwks_client.verifier();
        let mut jwks_subscription = jwks_client.subscribe();
        let poll_token = cancellation_token.clone();
        let poll_task = tokio::spawn(async move {
            jwks_client
                .run_with_client_builder(client_builder, poll_token)
                .await
        });

        assert_eq!(
            jwks_subscription
                .wait_until_present(&cancellation_token)
                .await,
            SyncHolderPresence::Present
        );

        Self {
            cancellation_token,
            jwks_server,
            poll_task,
            roll_task,
            secret: jwks_secret_holder.get().expect("the first roll published"),
            verifier,
        }
    }

    /// # Panics
    ///
    /// Panics when the roll or poll service does not shut down cleanly.
    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.roll_task
            .await
            .expect("the roll task joins")
            .expect("the roll service shuts down cleanly");
        self.poll_task
            .await
            .expect("the poll task joins")
            .expect("the poll service shuts down cleanly");
        self.jwks_server.stop().await;
    }
}
