use std::sync::Arc;

use margaret_console::command_outcome::CommandOutcome;
use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::runner;
use margaret_macros::singleton;

use crate::greeter::Greeter;

#[singleton]
#[console_command(name = "greet", description = "Greets a person by name")]
pub struct Greet {
    greeter: Arc<dyn Greeter + Send + Sync>,
}

impl Greet {
    #[constructor]
    pub fn create(greeter: Arc<dyn Greeter + Send + Sync>) -> Self {
        Self { greeter }
    }

    #[runner]
    pub async fn run(
        &self,
        #[console_argument] name: String,
        #[console_argument(name = "salutation")] salutation: Option<String>,
        #[console_argument(name = "loud")] loud: bool,
    ) -> CommandOutcome {
        println!(
            "{}, {} (salutation: {:?}, loud: {})",
            self.greeter.greet(),
            name,
            salutation,
            loud
        );

        CommandOutcome::Succeeded
    }
}
