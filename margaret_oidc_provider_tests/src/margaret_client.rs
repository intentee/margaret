use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_issuer_directory::discovered_issuer::DiscoveredIssuer;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_directory_tests::polled_fixture::PolledFixture;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_roller_server::jwks_roller::JwksRoller;
use margaret_jwks_secret_store_tests::fixture_roller::fixture_roller;
use margaret_oauth_vocabulary::client_secret::ClientSecret;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::provider_fixture::ProviderFixture;

const PORTAL_CLIENT_ID: &str = "portal";

pub struct MargaretClient {
    pub directory: PolledDirectory,
    pub roller: Arc<JwksRoller>,
    pub server: Arc<AuthorizationServerClient>,
}

impl MargaretClient {
    pub async fn of_portal(fixture: &ProviderFixture) -> Self {
        let roller = Arc::clone(&fixture.clients.asserting(PORTAL_CLIENT_ID).roller);

        Self::discovering(
            || fixture.issuer_request_client(),
            TokenTrust {
                audience: "artifacts",
                issuer: fixture.issuer,
            },
            Arc::clone(&roller),
            |request_client, metadata, trusted_issuer| {
                AuthorizationServerClient::with_private_key_jwt(
                    request_client,
                    metadata,
                    trusted_issuer,
                    PORTAL_CLIENT_ID,
                    roller,
                )
            },
        )
        .await
    }

    pub async fn signing_in(
        request_client: impl Fn() -> IssuerRequestClient,
        trust: TokenTrust,
        client_id: &'static str,
        client_secret: ClientSecret,
    ) -> Self {
        Self::discovering(
            request_client,
            trust,
            fixture_roller().await,
            |request_client, metadata, trusted_issuer| {
                AuthorizationServerClient::with_client_secret_basic(
                    request_client,
                    metadata,
                    trusted_issuer,
                    client_id,
                    client_secret,
                )
            },
        )
        .await
    }

    /// # Panics
    ///
    /// Panics when the callback is not a url.
    #[must_use]
    pub fn sign_in_flow(&self, callback: &str, scopes: &[&str]) -> SignInFlow {
        SignInFlow::create(
            Arc::clone(&self.server),
            Arc::clone(&self.roller),
            callback.to_string(),
            scopes,
        )
        .expect("the callback is a url")
    }

    pub async fn stop(self) {
        self.directory.stop().await;
    }

    async fn discovering(
        request_client: impl Fn() -> IssuerRequestClient,
        trust: TokenTrust,
        roller: Arc<JwksRoller>,
        server: impl FnOnce(
            Arc<IssuerRequestClient>,
            Arc<IssuerMetadata>,
            Arc<TrustedIssuer>,
        ) -> AuthorizationServerClient,
    ) -> Self {
        let metadata = Arc::new(IssuerMetadata::awaiting());
        let polled = PolledFixture::discovered(
            DiscoveredIssuer {
                discovery_url: String::leak(
                    oidc_discovery_url(
                        &trust
                            .issuer
                            .parse()
                            .expect("the trusted issuer is an https url"),
                    )
                    .to_string(),
                ),
                issuer: trust.issuer,
            },
            Arc::clone(&metadata),
        );
        let snapshot = polled.key_set.snapshot();
        let directory = PolledDirectory::start(vec![Arc::clone(&polled.polled)], request_client());

        polled.key_set.refreshed_since(&snapshot).await;

        Self {
            directory,
            roller,
            server: Arc::new(server(
                Arc::new(request_client()),
                metadata,
                Arc::new(polled.trusted(trust)),
            )),
        }
    }
}
