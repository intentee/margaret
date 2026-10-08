use std::sync::Arc;

use tokio::time::MissedTickBehavior;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

use crate::stores::note_store::NoteStore;
use crate::system_clock::SystemClock;
use crate::tickers::note_sweep_interval::NOTE_SWEEP_INTERVAL;

#[scheduled_with_tick_timer(interval = NOTE_SWEEP_INTERVAL, behavior = MissedTickBehavior::Delay)]
pub struct NoteSweeper {
    clock: Arc<SystemClock>,
    notes: Arc<NoteStore>,
}

impl NoteSweeper {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(clock: Arc<SystemClock>, notes: Arc<NoteStore>) -> anyhow::Result<Self> {
        Ok(Self { clock, notes })
    }

    /// # Errors
    ///
    /// Returns an error when the expired notes cannot be deleted.
    #[process]
    pub async fn run(&self) -> anyhow::Result<()> {
        self.notes.sweep(self.clock.now()).await?;

        Ok(())
    }
}
