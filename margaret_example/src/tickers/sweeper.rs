use std::convert::Infallible;
use std::sync::Arc;

use margaret_macros::constructor;
use margaret_macros::runner;
use margaret_macros::ticker;

use crate::metrics::Metrics;

#[ticker(
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

    #[runner]
    pub async fn run(&self) -> Result<(), Infallible> {
        self.metrics.record_sweep();

        Ok(())
    }
}
