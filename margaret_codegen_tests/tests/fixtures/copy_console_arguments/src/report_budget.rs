use std::num::NonZeroU32;
use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::budget::Budget;
use crate::connection_limits::ConnectionLimits;

#[singleton]
#[console_command(name = "report-budget", description = "Reports the configured budget")]
pub struct ReportBudget {
    budget: Budget,
    limits: Arc<ConnectionLimits>,
    max_connections: NonZeroU32,
    spare_connections: Option<NonZeroU32>,
}

impl ReportBudget {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        limits: Arc<ConnectionLimits>,
        #[console_argument(from = "budget")] budget: Budget,
        #[console_argument(from = "max-connections")] max_connections: NonZeroU32,
        #[console_argument(from = "spare-connections")] spare_connections: Option<NonZeroU32>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            budget,
            limits,
            max_connections,
            spare_connections,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok({
            println!(
                "{} {} {} {}",
                self.budget.0,
                self.limits.budget.0,
                self.max_connections,
                self.spare_connections.map_or(0, NonZeroU32::get)
            );

            CommandOutcome::Succeeded
        })
    }
}
