use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_six::LevelSix;

#[singleton]
#[console_command(name = "level-seven")]
pub struct LevelSeven {
    _dependency: Arc<LevelSix>,
}

impl LevelSeven {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub async fn create(dependency: Arc<LevelSix>) -> anyhow::Result<Self> {
        super::record::record(6).await;
        Ok(Self {
            _dependency: dependency,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
