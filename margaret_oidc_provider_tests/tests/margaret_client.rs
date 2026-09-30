use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::oauth_client::OAuthClient;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::portal_secret::PORTAL_SECRET;
use crate::provider_fixture::ProviderFixture;

pub struct MargaretClient {
    pub directory: PolledDirectory,
    pub server: Arc<AuthorizationServerClient>,
}

impl MargaretClient {
    pub async fn of_portal(fixture: &ProviderFixture) -> Self {
        let metadata = Arc::new(IssuerMetadata::awaiting());
        let trusted_issuer = Arc::new(TrustedIssuer::for_oidc_issuer(
            Arc::clone(&metadata),
            Arc::new(TokenTrust {
                audience: "artifacts".parse().expect("the resource is an audience"),
                issuer: "https://localhost"
                    .parse()
                    .expect("the provider issuer is an https url"),
            }),
        ));
        let snapshot = trusted_issuer.key_set.snapshot();
        let directory = PolledDirectory::start(
            vec![Arc::clone(&trusted_issuer)],
            fixture.issuer_request_client(),
        );

        trusted_issuer.key_set.refreshed_since(&snapshot).await;

        Self {
            directory,
            server: Arc::new(AuthorizationServerClient::create(
                Arc::new(fixture.issuer_request_client()),
                metadata,
                trusted_issuer,
                Arc::new(OAuthClient {
                    authentication: ClientAuthentication::ClientSecretBasic(
                        PORTAL_SECRET
                            .parse()
                            .expect("the portal secret is not empty"),
                    ),
                    client_id: "portal".parse().expect("the portal identifier is visible"),
                }),
            )),
        }
    }

    pub async fn stop(self) {
        self.directory.stop().await;
    }
}
