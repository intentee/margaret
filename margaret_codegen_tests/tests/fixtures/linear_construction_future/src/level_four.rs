use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_three::LevelThree;

#[singleton]
#[console_command(name = "level-four")]
pub(crate) struct LevelFour {
    _dependency: Arc<LevelThree>,
}

impl LevelFour {
    #[constructor]
    pub(crate) async fn create(dependency: Arc<LevelThree>) -> anyhow::Result<Self> {
        Ok(Self {
            _dependency: dependency,
        })
    }

    #[process]
    pub(crate) async fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
