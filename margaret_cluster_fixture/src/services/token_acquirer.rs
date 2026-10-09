use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::client_credentials::acquired_token::AcquiredToken;
use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::service;

use crate::margaret::models::models_token_acquisition_token_acquisition::draft::Draft;
use crate::margaret::oauth_clients::cluster::resources::notes::ResourceCredentials;
use crate::models::token_acquisition::TokenAcquisition;
use crate::models::token_acquisition_outcome::TokenAcquisitionOutcome;
use crate::system_clock::SystemClock;

#[service]
pub struct TokenAcquirer {
    clock: Arc<SystemClock>,
    credentials: Arc<ResourceCredentials>,
    database: Arc<Database>,
}

impl TokenAcquirer {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        clock: Arc<SystemClock>,
        credentials: Arc<ResourceCredentials>,
        database: Arc<Database>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            clock,
            credentials,
            database,
        })
    }

    /// # Errors
    ///
    /// Returns an error when the acquisition cannot be recorded.
    #[process]
    pub async fn run(&self, cancellation_token: CancellationToken) -> anyhow::Result<()> {
        let outcome = match self.credentials.access_token().await {
            AcquiredToken::Acquired(_) => TokenAcquisitionOutcome::Acquired,
            AcquiredToken::Refused(_) => TokenAcquisitionOutcome::Refused,
            AcquiredToken::Unavailable(_) => TokenAcquisitionOutcome::Unavailable,
        };

        TokenAcquisition::create(Draft {
            attempted_at: self.clock.now(),
            outcome,
        })
        .run(self.database.as_ref())
        .await?;
        cancellation_token.cancelled().await;

        Ok(())
    }
}
