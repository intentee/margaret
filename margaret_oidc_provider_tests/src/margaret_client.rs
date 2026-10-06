use std::sync::Arc;

use margaret_authorization_server_client::authorization_server_client::AuthorizationServerClient;
use margaret_authorization_server_client_tests::oauth_client_declaration::OAuthClientDeclaration;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_metadata::issuer_metadata::IssuerMetadata;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_secret_store_tests::rolled_store::rolled_store;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_oauth_client::client_authentication::ClientAuthentication;
use margaret_oauth_client::oauth_client::OAuthClient;
use margaret_oidc_sign_in::sign_in_flow::SignInFlow;
use margaret_token_signer_tests::fresh_p256_secret::fresh_p256_secret;
use margaret_token_trust::token_trust::TokenTrust;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

use crate::portal_secret::PORTAL_SECRET;
use crate::provider_fixture::ProviderFixture;

pub struct MargaretClient {
    pub directory: PolledDirectory,
    pub server: Arc<AuthorizationServerClient>,
}

impl MargaretClient {
    /// # Panics
    ///
    /// Panics when the portal trust or credentials are malformed.
    pub async fn of_portal(fixture: &ProviderFixture) -> Self {
        Self::signing_in(
            || fixture.issuer_request_client(),
            TokenTrust {
                audience: "artifacts".parse().expect("the resource is an audience"),
                issuer: "https://localhost"
                    .parse()
                    .expect("the provider issuer is an https url"),
            },
            OAuthClient {
                authentication: ClientAuthentication::ClientSecretBasic(
                    PORTAL_SECRET
                        .parse()
                        .expect("the portal secret is not empty"),
                ),
                client_id: "portal".parse().expect("the portal identifier is visible"),
            },
        )
        .await
    }

    pub async fn signing_in(
        request_client: impl Fn() -> IssuerRequestClient,
        trust: TokenTrust,
        client: OAuthClient,
    ) -> Self {
        let metadata = Arc::new(IssuerMetadata::awaiting());
        let trusted_issuer = Arc::new(TrustedIssuer::for_oidc_issuer(
            Arc::clone(&metadata),
            Arc::new(TokenTrustDeclaration { trust }),
        ));
        let snapshot = trusted_issuer.key_set.snapshot();
        let directory = PolledDirectory::start(vec![Arc::clone(&trusted_issuer)], request_client());

        trusted_issuer.key_set.refreshed_since(&snapshot).await;

        Self {
            directory,
            server: Arc::new(AuthorizationServerClient::create(
                Arc::new(request_client()),
                metadata,
                trusted_issuer,
                Arc::new(OAuthClientDeclaration { client }),
            )),
        }
    }

    #[must_use]
    pub fn sign_in_flow(&self) -> SignInFlow {
        SignInFlow::create(
            Arc::clone(&self.server),
            Arc::new(rolled_store(fresh_p256_secret())),
        )
    }

    pub async fn stop(self) {
        self.directory.stop().await;
    }
}
