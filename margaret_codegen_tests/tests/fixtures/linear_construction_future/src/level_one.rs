use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

#[singleton]
#[console_command(name = "level-one")]
pub struct LevelOne;

impl LevelOne {
    #[constructor]
    pub async fn create() -> anyhow::Result<Self> {
        super::record::record(0).await;
        Ok(Self)
    }

    #[process]
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
