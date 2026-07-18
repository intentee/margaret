use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
use reqwest::Client;
use reqwest::Response;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;
use url::Url;

use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;

use crate::jwk_public_set_holder::JwkPublicSetHolder;
use crate::jwks_client_error::JwksClientError;
use crate::jwks_poll_interval::JWKS_POLL_INTERVAL;

pub struct JwkPublicSetPollService {
    pub http_client: Client,
    pub jwk_public_set_holder: JwkPublicSetHolder,
    pub jwks_url: Url,
}

impl JwkPublicSetPollService {
    pub async fn fetch_jwk_public_set(&self) -> Result<JwkPublicSet, JwksClientError> {
        self.http_client
            .get(self.jwks_url.clone())
            .send()
            .await
            .and_then(Response::error_for_status)
            .map_err(JwksClientError::DocumentFetch)?
            .json::<JwkPublicSet>()
            .await
            .map_err(JwksClientError::DocumentFetch)
    }
}

#[async_trait]
impl Ticker for JwkPublicSetPollService {
    fn tick_interval(&self) -> Duration {
        JWKS_POLL_INTERVAL
    }

    fn tick_time_limit(&self) -> Option<Duration> {
        Some(JWKS_POLL_INTERVAL)
    }

    async fn handle_tick(
        &mut self,
        _cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        match self.fetch_jwk_public_set().await {
            Ok(jwk_public_set) => self
                .jwk_public_set_holder
                .set(Some(Arc::new(jwk_public_set))),
            Err(error) => error!("Unable to fetch the jwks document from the issuer: {error}"),
        }

        Ok(())
    }
}
