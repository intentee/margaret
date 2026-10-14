use margaret::framework::macros::console_command;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
#[console_command(name = "greet")]
struct Greet;

impl Greet {
    #[constructor]
    fn create(#[console_argument(from = "salutation")] salutation: String) -> anyhow::Result<Self> {}
}

#[singleton]
#[console_command(name = "greet-arguments")]
struct GreetArguments;
