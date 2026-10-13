use tokio_util::sync::CancellationToken;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

#[singleton]
#[console_command(
    name = "hold",
    description = "Holds the library open until it is stopped"
)]
pub struct Hold;

impl Hold {
    /// # Errors
    ///
    /// Never returns an error; holding ends only when the library is stopped.
    #[process]
    pub async fn run(
        &self,
        cancellation_token: CancellationToken,
    ) -> anyhow::Result<CommandOutcome> {
        cancellation_token.cancelled().await;

        Ok(CommandOutcome::Succeeded)
    }
}
