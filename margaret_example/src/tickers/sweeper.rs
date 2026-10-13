use std::sync::Arc;

use tokio::time::MissedTickBehavior;

use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::scheduled_with_tick_timer;

use crate::config::deployment_environment::DeploymentEnvironment;
use crate::constants::SWEEP_INTERVAL;
use crate::singletons::metrics::Metrics;

#[scheduled_with_tick_timer(interval = SWEEP_INTERVAL, behavior = MissedTickBehavior::Delay)]
pub struct Sweeper {
    deployment_environment: Arc<DeploymentEnvironment>,
    metrics: Arc<Metrics>,
}

impl Sweeper {
    #[must_use]
    #[constructor]
    pub fn create(
        deployment_environment: Arc<DeploymentEnvironment>,
        metrics: Arc<Metrics>,
    ) -> Self {
        Self {
            deployment_environment,
            metrics,
        }
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn run(&self) -> anyhow::Result<()> {
        if self
            .deployment_environment
            .sweep_budget()
            .is_none_or(|budget| self.metrics.sweeps() < usize::from(budget))
        {
            self.metrics.record_sweep();
        }

        Ok(())
    }
}
