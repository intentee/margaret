use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::database::database::Database;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::stores::alice::ALICE;
use crate::stores::alice_session::ALICE_SESSION;
use crate::stores::cluster_store_error::ClusterStoreError;
use crate::system_clock::SystemClock;

const SEED_SESSION: &str = "INSERT INTO user_sessions (id, authenticated_at, user_id) \
     VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING";

const SEED_USER: &str = "INSERT INTO users (id, name) VALUES ($1, $2) ON CONFLICT (id) DO NOTHING";

#[singleton]
#[console_command(
    name = "seed",
    description = "Seeds the cluster database with its user and session"
)]
pub struct Seed {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl Seed {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
    }

    /// # Errors
    ///
    /// Returns `ClusterStoreError` when the user or the session cannot be stored.
    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        let mut client = self
            .database
            .client()
            .await
            .map_err(ClusterStoreError::Unavailable)?;
        let transaction = client
            .transaction()
            .await
            .map_err(ClusterStoreError::Seed)?;

        transaction
            .execute(SEED_USER, &[&ALICE, &"alice"])
            .await
            .map_err(ClusterStoreError::Seed)?;
        transaction
            .execute(SEED_SESSION, &[&ALICE_SESSION, &self.clock.now(), &ALICE])
            .await
            .map_err(ClusterStoreError::Seed)?;
        transaction
            .commit()
            .await
            .map_err(ClusterStoreError::Seed)?;

        Ok(CommandOutcome::Succeeded)
    }
}
