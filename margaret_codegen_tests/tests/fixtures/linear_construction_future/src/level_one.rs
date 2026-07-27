use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

#[singleton]
#[console_command(name = "level-one")]
pub(crate) struct LevelOne;

impl LevelOne {
    #[constructor]
    pub(crate) async fn create() -> anyhow::Result<Self> {
        super::record::record(0);
        Ok(Self)
    }

    #[process]
    pub(crate) async fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
