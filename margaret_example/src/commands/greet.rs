use std::sync::Arc;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

use crate::english_greeter::EnglishGreeter;

#[singleton]
#[console_command(name = "greet", description = "Greets a person by name")]
pub struct Greet {
    greeter: Arc<EnglishGreeter>,
    name: String,
    salutation: Option<String>,
    loud: bool,
}

impl Greet {
    #[constructor]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn create(
        greeter: Arc<EnglishGreeter>,
        #[console_argument(positional)] name: String,
        #[console_argument(from = "salutation")] salutation: Option<String>,
        #[console_argument(from = "loud")] loud: bool,
    ) -> anyhow::Result<Self> {
        Ok({
            Self {
                greeter,
                name,
                salutation,
                loud,
            }
        })
    }

    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        Ok({
            println!(
                "{}, {} (salutation: {:?}, loud: {})",
                self.greeter.greet(),
                self.name,
                self.salutation,
                self.loud
            );

            CommandOutcome::Succeeded
        })
    }
}
