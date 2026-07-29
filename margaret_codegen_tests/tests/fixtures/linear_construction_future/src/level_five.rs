use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use super::level_four::LevelFour;

#[singleton]
#[console_command(name = "level-five")]
pub struct LevelFive {
    _dependency: Arc<LevelFour>,
}

impl LevelFive {
    #[constructor]
    pub async fn create(dependency: Arc<LevelFour>) -> anyhow::Result<Self> {
        super::record::record(4).await;
        Ok(Self {
            _dependency: dependency,
        })
    }

    #[process]
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
