use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

#[singleton]
#[console_command(name = "boot", description = "Boots the demo")]
pub struct Boot;

impl Boot {
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }

    #[process]
    pub async fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
