use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_three::LevelThree;

#[singleton]
#[console_command(name = "level-four")]
pub struct LevelFour {
    _dependency: Arc<LevelThree>,
}

impl LevelFour {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub async fn create(dependency: Arc<LevelThree>) -> anyhow::Result<Self> {
        super::record::record(3).await;
        Ok(Self {
            _dependency: dependency,
        })
    }

    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
