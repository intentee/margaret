use std::sync::Arc;

use async_trait::async_trait;
use clap::Arg;
use clap::ArgMatches;
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
}

impl Greet {
    #[constructor]
    pub fn create(greeter: Arc<dyn Greeter + Send + Sync>) -> Self {
        Self { greeter }
    }
}

#[async_trait]
impl Command for Greet {
    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new("name").long("name").required(true)]
    }

    async fn run(&self, matches: &ArgMatches) -> CommandOutcome {
        let name = matches
            .get_one::<String>("name")
            .expect("name is a required argument");

        println!("{}, {name}", self.greeter.greet());

        CommandOutcome::Succeeded
    }
}
