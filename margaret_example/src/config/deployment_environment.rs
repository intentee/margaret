use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
pub struct DeploymentEnvironment {
    name: String,
    sweep_budget: Option<u16>,
}

impl DeploymentEnvironment {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[environment_variable(from = "MARGARET_EXAMPLE_ENVIRONMENT")] name: String,
        #[environment_variable(from = "MARGARET_EXAMPLE_SWEEP_BUDGET")] sweep_budget: Option<u16>,
    ) -> anyhow::Result<Self> {
        Ok(Self { name, sweep_budget })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn sweep_budget(&self) -> Option<u16> {
        self.sweep_budget
    }
}
