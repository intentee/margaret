use std::sync::Arc;

use async_trait::async_trait;
use margaret_console::command::Command;
use margaret_console::command_outcome::CommandOutcome;
use margaret_macros::console_command;
use margaret_macros::constructor;
use margaret_macros::singleton;

use crate::greeter::Greeter;

#[singleton]
#[console_command(name = "greet", description = "Greets a person by name")]
pub struct Greet {
    greeter: Arc<dyn Greeter + Send + Sync>,
    name: String,
}

impl Greet {
    #[constructor]
    pub fn create(
        greeter: Arc<dyn Greeter + Send + Sync>,
        #[console_argument(name = "name", required = true)] name: String,
    ) -> Self {
        Self { greeter, name }
    }
}

#[async_trait]
impl Command for Greet {
    async fn run(&self) -> CommandOutcome {
        println!("{}, {}", self.greeter.greet(), self.name);

        CommandOutcome::Succeeded
    }
}
