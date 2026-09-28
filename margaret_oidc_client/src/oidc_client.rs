use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use reqwest::ClientBuilder;
use tokio_util::sync::CancellationToken;
use url::Url;

use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_key_set_poll::poll_key_set::poll_key_set;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::discovery_key_set_locator::DiscoveryKeySetLocator;

pub struct OidcClient {
    discovery_url: Url,
    token_trust: Arc<dyn DeclaresTokenTrust>,
    verification_key_set_holder: VerificationKeySetHolder,
}

impl OidcClient {
    #[must_use]
    pub fn create(token_trust: Arc<dyn DeclaresTokenTrust>) -> Self {
        Self {
            discovery_url: oidc_discovery_url(&token_trust.token_trust().issuer),
            token_trust,
            verification_key_set_holder: VerificationKeySetHolder::default(),
        }
    }

    /// # Errors
    ///
    /// Returns an error when the issuer document client cannot be built.
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<()> {
        self.run_with_client_builder(Client::builder(), cancellation_token)
            .await
    }

    /// # Errors
    ///
    /// Returns an error when the issuer document client cannot be built.
    pub async fn run_with_client_builder(
        &self,
        client_builder: ClientBuilder,
        cancellation_token: CancellationToken,
    ) -> Result<()> {
        poll_key_set(
            DiscoveryKeySetLocator {
                discovery_url: self.discovery_url.clone(),
                token_trust: self.token_trust.clone(),
            },
            self.verification_key_set_holder.clone(),
            client_builder,
            cancellation_token,
        )
        .await
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<VerificationKeySet>> {
        self.verification_key_set_holder.subscribe()
    }

    #[must_use]
    pub fn verifier(&self) -> Arc<BearerTokenVerifier> {
        Arc::new(BearerTokenVerifier::new_for_id_tokens(
            self.token_trust.clone(),
            self.verification_key_set_holder.clone(),
        ))
    }
}
