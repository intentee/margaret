use std::fs;
use std::path::PathBuf;

use margaret::framework::console::command_outcome::CommandOutcome;
use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

#[singleton]
#[console_command(name = "greet", description = "Writes a greeting for a reader")]
pub struct Greet {
    loud: bool,
    name: String,
    salutation: Option<String>,
    to: PathBuf,
}

impl Greet {
    #[must_use]
    #[constructor]
    pub fn create(
        #[console_argument(positional)] name: String,
        #[console_argument(from = "salutation")] salutation: Option<String>,
        #[console_argument(from = "loud")] loud: bool,
        #[console_argument(from = "to")] to: PathBuf,
    ) -> Self {
        Self {
            loud,
            name,
            salutation,
            to,
        }
    }

    /// # Errors
    ///
    /// Returns an error when the greeting cannot be written to its destination.
    #[process]
    pub fn run(&self) -> anyhow::Result<CommandOutcome> {
        let greeting = match &self.salutation {
            Some(salutation) => format!("{salutation}, {}", self.name),
            None => self.name.clone(),
        };
        let written = if self.loud {
            greeting.to_uppercase()
        } else {
            greeting
        };

        fs::write(&self.to, written)
            .map(|()| CommandOutcome::Succeeded)
            .map_err(anyhow::Error::from)
    }
}
