use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_seven::LevelSeven;

#[singleton]
#[console_command(name = "level-eight")]
pub(crate) struct LevelEight {
    _dependency: Arc<LevelSeven>,
}

impl LevelEight {
    #[constructor]
    pub(crate) async fn create(dependency: Arc<LevelSeven>) -> anyhow::Result<Self> {
        super::record::record(7);
        Ok(Self {
            _dependency: dependency,
        })
    }

    #[process]
    pub(crate) async fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
