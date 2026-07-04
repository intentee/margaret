use std::convert::Infallible;
use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::scheduled_with_tick_timer;

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
    pub fn create(metrics: Arc<Metrics>) -> Self {
        Self { metrics }
    }

    #[process]
    pub async fn run(&self) -> Result<(), Infallible> {
        self.metrics.record_sweep();

        Ok(())
    }
}
