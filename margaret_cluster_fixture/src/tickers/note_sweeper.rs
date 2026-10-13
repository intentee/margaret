use std::sync::Arc;

use tokio::time::MissedTickBehavior;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

use crate::constants::NOTE_SWEEP_INTERVAL;
use crate::models::note::Note;
use crate::system_clock::SystemClock;

#[scheduled_with_tick_timer(interval = NOTE_SWEEP_INTERVAL, behavior = MissedTickBehavior::Delay)]
pub struct NoteSweeper {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl NoteSweeper {
    #[must_use]
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> Self {
        Self { clock, database }
    }

    /// # Errors
    ///
    /// Returns an error when the expired notes cannot be deleted.
    #[process]
    pub async fn run(&self) -> anyhow::Result<()> {
        Note::query()
            .expires_at
            .at_most(self.clock.now())
            .delete(self.database.as_ref())
            .await?;

        Ok(())
    }
}
