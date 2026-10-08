use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::FirstTickTiming;
use trzcina::TickContext;
use trzcina::Ticker;

use crate::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use crate::jwks_roller_server_bundle::JwksRollerServerBundle;

pub struct JwksRollerService {
    pub bundle: JwksRollerServerBundle,
}

#[async_trait]
impl Ticker for JwksRollerService {
    fn first_tick_timing(&self) -> FirstTickTiming {
        FirstTickTiming::AfterInterval
    }

    fn tick_interval(&self) -> Duration {
        JWKS_ROLL_INTERVAL
    }

    async fn handle_tick(
        &mut self,
        _cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        self.bundle.roll_and_publish().await?;

        Ok(())
    }
}
