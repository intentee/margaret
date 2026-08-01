use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;
use reqwest::ClientBuilder;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;

use margaret_jwks_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwks_keygen::public_jwks::PublicJwks;
use margaret_jwt_claims::expected_claims::ExpectedClaims;
use margaret_jwt_claims::provides_expected_claims::ProvidesExpectedClaims;
use margaret_sync_holder::sync_holder_subscription::SyncHolderSubscription;

use crate::public_jwks_holder::PublicJwksHolder;
use crate::public_jwks_poll_service::PublicJwksPollService;
use crate::public_jwks_verifier::PublicJwksVerifier;

pub struct JwksClient {
    endpoint_provider: Arc<dyn ProvidesEndpoint>,
    expected_claims: ExpectedClaims,
    public_jwks_holder: PublicJwksHolder,
}

impl JwksClient {
    #[must_use]
    pub fn create<TIssuer: ProvidesEndpoint + ProvidesExpectedClaims + 'static>(
        issuer: Arc<TIssuer>,
    ) -> Self {
        Self {
            expected_claims: issuer.expected_claims(),
            endpoint_provider: issuer,
            public_jwks_holder: PublicJwksHolder::default(),
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
        let http_client = client_builder.build()?;
        let poll_service = PublicJwksPollService {
            endpoint_provider: self.endpoint_provider.clone(),
            http_client,
            public_jwks_holder: self.public_jwks_holder.clone(),
        };

        Box::new(poll_service).run(cancellation_token).await
    }

    #[must_use]
    pub fn subscribe(&self) -> SyncHolderSubscription<Arc<PublicJwks>> {
        self.public_jwks_holder.subscribe()
    }

    #[must_use]
    pub fn verifier(&self) -> Arc<PublicJwksVerifier> {
        Arc::new(PublicJwksVerifier::new(
            self.expected_claims.clone(),
            self.public_jwks_holder.clone(),
        ))
    }
}
