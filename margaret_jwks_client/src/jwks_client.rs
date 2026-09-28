use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use reqwest::ClientBuilder;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

use crate::jwks_client_error::JwksClientError;
use crate::public_jwks_poll_service::PublicJwksPollService;
use crate::public_jwks_verifier::PublicJwksVerifier;
use crate::verification_key_set_holder::VerificationKeySetHolder;

pub struct JwksClient {
    endpoint_provider: Arc<dyn ProvidesEndpoint>,
    verification_key_set_holder: VerificationKeySetHolder,
}

impl JwksClient {
    #[must_use]
    pub fn create(endpoint_provider: Arc<dyn ProvidesEndpoint>) -> Self {
        Self {
            endpoint_provider,
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
        let issuer_document_client = IssuerDocumentClient::build(client_builder)
            .map_err(|source| JwksClientError::ClientBuild { source })?;
        let poll_service = PublicJwksPollService {
            endpoint_provider: self.endpoint_provider.clone(),
            issuer_document_client,
            verification_key_set_holder: self.verification_key_set_holder.clone(),
        };

        Box::new(poll_service).run(cancellation_token).await
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<VerificationKeySet>> {
        self.verification_key_set_holder.subscribe()
    }

    #[must_use]
    pub fn verifier(&self) -> Arc<PublicJwksVerifier> {
        Arc::new(PublicJwksVerifier::new(
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

    use super::JwksClient;

    fn client() -> JwksClient {
        let endpoint = StaticEndpoint::new(
            Url::parse("https://issuer.invalid/.well-known/jwks.json").expect("the url parses"),
        );

        JwksClient::create(Arc::new(endpoint))
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
