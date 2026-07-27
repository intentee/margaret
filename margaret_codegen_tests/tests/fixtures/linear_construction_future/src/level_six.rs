use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_five::LevelFive;

#[singleton]
#[console_command(name = "level-six")]
pub(crate) struct LevelSix {
    _dependency: Arc<LevelFive>,
}

impl LevelSix {
    #[constructor]
    pub(crate) async fn create(dependency: Arc<LevelFive>) -> anyhow::Result<Self> {
        super::construction_counts::record(5);
        Ok(Self {
            _dependency: dependency,
        })
    }

    #[process]
    pub(crate) async fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
