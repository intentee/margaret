use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
use reqwest::Client;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;

use margaret_endpoint::provides_endpoint::ProvidesEndpoint;
use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;

use crate::fetch_jwk_public_set::fetch_jwk_public_set;
use crate::jwk_public_set_holder::JwkPublicSetHolder;
use crate::jwks_client_error::JwksClientError;
use crate::jwks_poll_interval_after_ready::JWKS_POLL_INTERVAL_AFTER_READY;
use crate::jwks_poll_interval_before_ready::JWKS_POLL_INTERVAL_BEFORE_READY;
use crate::well_known_jwks_url::well_known_jwks_url;

pub struct JwkPublicSetPollService {
    pub endpoint: Arc<dyn ProvidesEndpoint>,
    pub http_client: Client,
    pub jwk_public_set_holder: JwkPublicSetHolder,
}

impl JwkPublicSetPollService {
    pub async fn poll(&self) -> Result<JwkPublicSet, JwksClientError> {
        let endpoint = self.endpoint.resolve().await?;
        let jwks_url = well_known_jwks_url(&endpoint.url)?;

        fetch_jwk_public_set(&self.http_client, jwks_url).await
    }
}

#[async_trait]
impl Ticker for JwkPublicSetPollService {
    fn tick_interval(&self) -> Duration {
        JWKS_POLL_INTERVAL_BEFORE_READY
    }

    async fn handle_tick(
        &mut self,
        cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        match self.poll().await {
            Ok(jwk_public_set) => self
                .jwk_public_set_holder
                .set(Some(Arc::new(jwk_public_set))),
            Err(error) => error!("Unable to fetch the jwks document from the issuer: {error}"),
        }

        if self.jwk_public_set_holder.is_ready() {
            tokio::select! {
                biased;
                () = cancellation_token.cancelled() => {}
                () = sleep(JWKS_POLL_INTERVAL_AFTER_READY) => {}
            }
        }

        Ok(())
    }
}
