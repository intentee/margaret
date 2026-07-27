use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_two::LevelTwo;

#[singleton]
#[console_command(name = "level-three")]
pub(crate) struct LevelThree {
    _dependency: Arc<LevelTwo>,
}

impl LevelThree {
    #[constructor]
    pub(crate) async fn create(dependency: Arc<LevelTwo>) -> anyhow::Result<Self> {
        super::construction_counts::record(2);
        Ok(Self {
            _dependency: dependency,
        })
    }

    #[process]
    pub(crate) async fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
