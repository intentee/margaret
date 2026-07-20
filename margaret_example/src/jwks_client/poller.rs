use std::convert::Infallible;
use std::sync::Arc;

use log::error;
use log::info;
use reqwest::Client;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwks_client::fetch_jwk_public_set::fetch_jwk_public_set;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::well_known_jwks_url::well_known_jwks_url;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::scheduled_with_tick_timer;

#[scheduled_with_tick_timer(
    interval = margaret_jwks_client::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY,
    behavior = tokio::time::MissedTickBehavior::Delay
)]
pub struct Poller {
    endpoint: Arc<dyn ProvidesEndpoint>,
    http_client: Client,
}

impl Poller {
    #[constructor]
    #[must_use]
    pub fn create(#[endpoint_provider(jwks)] endpoint: Arc<dyn ProvidesEndpoint>) -> Self {
        Self {
            endpoint,
            http_client: Client::new(),
        }
    }

    #[process]
    pub async fn run(&self) -> Result<(), Infallible> {
        match self.poll().await {
            Ok(_) => info!("polled the internal jwks document"),
            Err(error) => error!("unable to poll the internal jwks document: {error}"),
        }

        Ok(())
    }

    async fn poll(&self) -> Result<JwkPublicSet, JwksClientError> {
        let endpoint = self.endpoint.resolve().await?;
        let jwks_url = well_known_jwks_url(&endpoint.url)?;

        fetch_jwk_public_set(&self.http_client, jwks_url).await
    }
}
