use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_six::LevelSix;

#[singleton]
#[console_command(name = "level-seven")]
pub(crate) struct LevelSeven {
    _dependency: Arc<LevelSix>,
}

impl LevelSeven {
    #[constructor]
    pub(crate) async fn create(dependency: Arc<LevelSix>) -> anyhow::Result<Self> {
        super::construction_counts::record(6);
        Ok(Self {
            _dependency: dependency,
        })
    }

    #[process]
    pub(crate) async fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
