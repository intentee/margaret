use std::sync::Arc;

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
    #[must_use]
    pub fn create(
        greeter: Arc<EnglishGreeter>,
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
    pub async fn run(&self) -> anyhow::Result<()> {
        println!(
            "{}, {} (salutation: {:?}, loud: {})",
            self.greeter.greet(),
            self.name,
            self.salutation,
            self.loud
        );

        Ok(())
    }
}
