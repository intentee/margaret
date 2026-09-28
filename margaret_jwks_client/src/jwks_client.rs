use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use reqwest::ClientBuilder;
use tokio_util::sync::CancellationToken;

use margaret_bearer_token_verification::bearer_token_verifier::BearerTokenVerifier;
use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_key_set_poll::poll_key_set::poll_key_set;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;
use margaret_token_trust::declares_token_trust::DeclaresTokenTrust;

use crate::endpoint_key_set_locator::EndpointKeySetLocator;

pub struct JwksClient {
    endpoint_provider: Arc<dyn ProvidesEndpoint>,
    token_trust: Arc<dyn DeclaresTokenTrust>,
    verification_key_set_holder: VerificationKeySetHolder,
}

impl JwksClient {
    #[must_use]
    pub fn create(
        endpoint_provider: Arc<dyn ProvidesEndpoint>,
        token_trust: Arc<dyn DeclaresTokenTrust>,
    ) -> Self {
        Self {
            endpoint_provider,
            token_trust,
            verification_key_set_holder: VerificationKeySetHolder::default(),
        }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<()> {
        self.run_with_client_builder(Client::builder(), cancellation_token)
            .await
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn run_with_client_builder(
        &self,
        client_builder: ClientBuilder,
        cancellation_token: CancellationToken,
    ) -> Result<()> {
        poll_key_set(
            EndpointKeySetLocator {
                endpoint_provider: self.endpoint_provider.clone(),
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
        Arc::new(BearerTokenVerifier::new_for_access_tokens(
            self.token_trust.clone(),
            self.verification_key_set_holder.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use reqwest::Client;
    use reqwest::tls::Version;
    use tokio_util::sync::CancellationToken;
    use url::Url;

    use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
    use margaret_token_trust::token_trust::TokenTrust;

    use super::JwksClient;

    fn client() -> JwksClient {
        let endpoint = StaticEndpoint::new(
            Url::parse("https://issuer.invalid/.well-known/jwks.json").expect("the url parses"),
        );
        let token_trust = TokenTrust {
            audience: "margaret".parse().expect("the audience is not empty"),
            issuer: "https://issuer.invalid"
                .parse()
                .expect("the issuer is an https url"),
        };

        JwksClient::create(Arc::new(endpoint), Arc::new(token_trust))
    }

    #[tokio::test]
    async fn returns_when_cancelled_before_the_first_poll() {
        let cancellation_token = CancellationToken::new();
        cancellation_token.cancel();

        client()
            .run(cancellation_token)
            .await
            .expect("a cancelled client shuts down cleanly");
    }

    #[tokio::test]
    async fn reports_a_client_builder_that_cannot_be_built() {
        let broken_builder = Client::builder().min_tls_version(Version::TLS_1_3);

        assert!(
            client()
                .run_with_client_builder(broken_builder, CancellationToken::new())
                .await
                .is_err()
        );
    }
}
