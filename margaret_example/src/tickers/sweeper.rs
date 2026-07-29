use std::sync::Arc;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

use crate::metrics::Metrics;

#[scheduled_with_tick_timer(
    interval = crate::sweep_interval::SWEEP_INTERVAL,
    behavior = tokio::time::MissedTickBehavior::Delay
)]
pub struct Sweeper {
    metrics: Arc<Metrics>,
}

impl Sweeper {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(metrics: Arc<Metrics>) -> anyhow::Result<Self> {
        Ok(Self { metrics })
    }

    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn run(&self) -> anyhow::Result<()> {
        self.metrics.record_sweep();

        Ok(())
    }
}
