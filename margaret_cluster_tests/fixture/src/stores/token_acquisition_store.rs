use std::sync::Arc;

use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

use crate::models::token_acquisition_outcome::TokenAcquisitionOutcome;
use crate::stores::cluster_store_error::ClusterStoreError;
use crate::system_clock::SystemClock;

const RECORD_TOKEN_ACQUISITION: &str =
    "INSERT INTO token_acquisitions (attempted_at, outcome) VALUES ($1, $2)";

#[singleton]
pub struct TokenAcquisitionStore {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl TokenAcquisitionStore {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the acquisition cannot be recorded.
    pub async fn record(&self, outcome: &TokenAcquisitionOutcome) -> Result<(), ClusterStoreError> {
        self.database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?
            .execute(
                RECORD_TOKEN_ACQUISITION,
                &[&self.clock.now(), &outcome.stored()],
            )
            .await
            .map_err(ClusterStoreError::RecordTokenAcquisition)
            .map(|_recorded| ())
    }
}
