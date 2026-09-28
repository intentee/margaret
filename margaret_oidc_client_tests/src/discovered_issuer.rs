use std::sync::Arc;

use serde_json::json;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_oidc_client::oidc_client::OidcClient;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

use crate::fixture_issuer_routes::FixtureIssuerRoutes;
use crate::localhost_trust::localhost_trust;
use crate::running_fixture_issuer::RunningFixtureIssuer;

pub struct DiscoveredIssuer {
    cancellation_token: CancellationToken,
    issuer: RunningFixtureIssuer,
    poll_task: JoinHandle<anyhow::Result<()>>,
    pub verifier: Arc<BearerTokenVerifier>,
}

impl DiscoveredIssuer {
    /// # Panics
    ///
    /// Panics when the client cannot discover the key set of the issuer.
    pub async fn start(key: &FixtureRsaKey) -> Self {
        let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
            discovery: Arc::new(StaticHandler {
                body: br#"{"issuer":"https://localhost","jwks_uri":"https://localhost/jwks"}"#
                    .to_vec(),
                content_type: "application/json",
                status: 200,
            }),
            key_set: Arc::new(StaticHandler {
                body: json!({ "keys": [key.jwk()] }).to_string().into_bytes(),
                content_type: "application/json",
                status: 200,
            }),
        })
        .await;
        let client = OidcClient::create(Arc::new(localhost_trust()));
        let verifier = client.verifier();
        let mut subscription = client.subscribe();
        let cancellation_token = CancellationToken::new();
        let poll_token = cancellation_token.clone();
        let client_builder = issuer.client_builder();
        let poll_task = tokio::spawn(async move {
            client
                .run_with_client_builder(client_builder, poll_token)
                .await
        });

        assert_eq!(
            subscription.wait_until_present(&cancellation_token).await,
            SyncHolderPresence::Present
        );

        Self {
            cancellation_token,
            issuer,
            poll_task,
            verifier,
        }
    }

    /// # Panics
    ///
    /// Panics when the client does not shut down cleanly.
    pub async fn stop(self) {
        self.cancellation_token.cancel();
        self.poll_task
            .await
            .expect("the poll task joins")
            .expect("the oidc client shuts down cleanly");
        self.issuer.stop().await;
    }
}
