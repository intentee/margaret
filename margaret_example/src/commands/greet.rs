use std::sync::Arc;

use margaret_console::command_outcome::CommandOutcome;
use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::singleton;

use crate::greeter::Greeter;

#[singleton]
#[console_command(name = "greet", description = "Greets a person by name")]
pub struct Greet {
    greeter: Arc<dyn Greeter>,
    name: String,
    salutation: Option<String>,
    loud: bool,
}

impl Greet {
    #[constructor]
    pub fn create(
        greeter: Arc<dyn Greeter>,
        #[console_argument(positional)] name: String,
        #[console_argument(from = "salutation")] salutation: Option<String>,
        #[console_argument(from = "loud")] loud: bool,
    ) -> Self {
        Self {
            greeter,
            name,
            salutation,
            loud,
        }
    }

    #[process]
    pub async fn run(&self) -> CommandOutcome {
        println!(
            "{}, {} (salutation: {:?}, loud: {})",
            self.greeter.greet(),
            self.name,
            self.salutation,
            self.loud
        );

        CommandOutcome::Succeeded
    }
}
