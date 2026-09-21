use anyhow::Result;
use failures as anyhow;
use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

#[singleton]
#[console_command(name = "boot", description = "Boots the demo")]
pub struct Boot;

impl Boot {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> Result<Self> {
        Ok(Self)
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn run(&self) -> Result<CommandOutcome> {
        Ok(CommandOutcome::Succeeded)
    }
}
