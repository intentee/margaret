use std::sync::Arc;

use tokio::time::MissedTickBehavior;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

use crate::models::note::Note;
use crate::system_clock::SystemClock;
use crate::tickers::note_sweep_interval::NOTE_SWEEP_INTERVAL;

#[scheduled_with_tick_timer(interval = NOTE_SWEEP_INTERVAL, behavior = MissedTickBehavior::Delay)]
pub struct NoteSweeper {
    clock: Arc<SystemClock>,
    database: Arc<Database>,
}

impl NoteSweeper {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { clock, database })
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
