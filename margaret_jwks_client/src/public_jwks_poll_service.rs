use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
use reqwest::Client;
use reqwest::Response;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwks_keygen::public_jwks::PublicJwks;

use crate::jwks_client_error::JwksClientError;
use crate::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use crate::jwks_poll_interval_before_ready::JWKS_POLL_INTERVAL_BEFORE_READY;
use crate::public_jwks_holder::PublicJwksHolder;

pub struct PublicJwksPollService {
    endpoint_provider: Arc<dyn ProvidesEndpoint>,
    http_client: Client,
    public_jwks_holder: PublicJwksHolder,
}

impl PublicJwksPollService {
    #[must_use]
    pub fn new(
        public_jwks_holder: PublicJwksHolder,
        endpoint_provider: Arc<dyn ProvidesEndpoint>,
        http_client: Client,
    ) -> Self {
        Self {
            endpoint_provider,
            http_client,
            public_jwks_holder,
        }
    }

    pub async fn fetch_public_jwks(&self) -> Result<PublicJwks, JwksClientError> {
        let jwks_url = self
            .endpoint_provider
            .provide()
            .await
            .map_err(JwksClientError::EndpointResolution)?;

        self.http_client
            .get(jwks_url)
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(JwksClientError::DocumentFetch)?
            .json::<PublicJwks>()
            .await
            .map_err(JwksClientError::DocumentFetch)
    }
}

#[async_trait]
impl Ticker for PublicJwksPollService {
    fn tick_interval(&self) -> Duration {
        JWKS_POLL_INTERVAL_BEFORE_READY
    }

    async fn handle_tick(
        &mut self,
        cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        match self.fetch_public_jwks().await {
            Ok(public_jwks) => self.public_jwks_holder.set(Some(Arc::new(public_jwks))),
            Err(error) => error!("Unable to fetch the jwks document from the issuer: {error}"),
        }

        if self.public_jwks_holder.is_ready() {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => {}
                () = sleep(JWKS_POLL_INTERVAL_AFTER_READY) => {}
            }
        }

        Ok(())
    }
}
