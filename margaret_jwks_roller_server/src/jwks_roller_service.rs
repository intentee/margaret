use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use trzcina::TickContext;
use trzcina::Ticker;

use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_roller::jwks_secret_storage::JwksSecretStorage;

use crate::jwks_document_holder::JwksDocumentHolder;
use crate::jwks_roll_interval::JWKS_ROLL_INTERVAL;
use crate::roll_and_publish::roll_and_publish;

pub struct JwksRollerService {
    jwks_document_holder: JwksDocumentHolder,
    jwks_secret_holder: JwksSecretHolder,
    storage: Arc<dyn JwksSecretStorage>,
}

impl JwksRollerService {
    #[must_use]
    pub fn new(
        jwks_document_holder: JwksDocumentHolder,
        jwks_secret_holder: JwksSecretHolder,
        storage: Arc<dyn JwksSecretStorage>,
    ) -> Self {
        Self {
            jwks_document_holder,
            jwks_secret_holder,
            storage,
        }
    }
}

#[async_trait]
impl Ticker for JwksRollerService {
    fn tick_interval(&self) -> Duration {
        JWKS_ROLL_INTERVAL
    }

    async fn handle_tick(
        &mut self,
        _cancellation_token: CancellationToken,
        _tick_context: TickContext,
    ) -> Result<()> {
        roll_and_publish(
            self.storage.as_ref(),
            &self.jwks_secret_holder,
            &self.jwks_document_holder,
        )?;

        Ok(())
    }
}
