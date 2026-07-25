use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use reqwest::ClientBuilder;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;

use crate::public_jwks_holder::PublicJwksHolder;
use crate::public_jwks_poll_service::PublicJwksPollService;
use crate::public_jwks_verifier::PublicJwksVerifier;

pub struct JwksClient {
    endpoint_provider: Arc<dyn ProvidesEndpoint>,
    public_jwks_holder: PublicJwksHolder,
}

impl JwksClient {
    #[must_use]
    pub fn create(endpoint_provider: Arc<dyn ProvidesEndpoint>) -> Self {
        Self {
            endpoint_provider,
            public_jwks_holder: PublicJwksHolder::default(),
        }
    }

    pub async fn run(&self, cancellation_token: CancellationToken) -> Result<()> {
        self.run_with_client_builder(Client::builder(), cancellation_token)
            .await
    }

    pub async fn run_with_client_builder(
        &self,
        client_builder: ClientBuilder,
        cancellation_token: CancellationToken,
    ) -> Result<()> {
        let http_client = client_builder.build()?;
        let poll_service = PublicJwksPollService {
            endpoint_provider: self.endpoint_provider.clone(),
            http_client,
            public_jwks_holder: self.public_jwks_holder.clone(),
        };

        Box::new(poll_service).run(cancellation_token).await
    }

    #[must_use]
    pub fn verifier(&self) -> Arc<PublicJwksVerifier> {
        Arc::new(PublicJwksVerifier::new(self.public_jwks_holder.clone()))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use reqwest::Client;
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
        let broken_builder = Client::builder().use_preconfigured_tls(0u8);

        assert!(
            client()
                .run_with_client_builder(broken_builder, CancellationToken::new())
                .await
                .is_err()
        );
    }
}
