use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_one::LevelOne;
use super::record::record;

#[singleton]
#[console_command(name = "level-two")]
pub struct LevelTwo {
    _dependency: Arc<LevelOne>,
}

impl LevelTwo {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub async fn create(dependency: Arc<LevelOne>) -> anyhow::Result<Self> {
        record(1).await;
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
