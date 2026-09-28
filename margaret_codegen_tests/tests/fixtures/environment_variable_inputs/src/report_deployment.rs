use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::deployment::Deployment;

#[singleton]
#[console_command(
    name = "report-deployment",
    description = "Reports the deployment configuration"
)]
pub struct ReportDeployment {
    deployment: Arc<Deployment>,
    verbose: Option<bool>,
    worker_count: u16,
}

impl ReportDeployment {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        deployment: Arc<Deployment>,
        #[environment_variable(from = "MARGARET_FIXTURE_WORKER_COUNT")] worker_count: u16,
        #[environment_variable(from = "MARGARET_FIXTURE_VERBOSE")] verbose: Option<bool>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            deployment,
            verbose,
            worker_count,
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
                self.deployment.database_url,
                self.deployment.upload_root.display(),
                self.worker_count,
                self.verbose.unwrap_or(false)
            );

            CommandOutcome::Succeeded
        })
    }
}
