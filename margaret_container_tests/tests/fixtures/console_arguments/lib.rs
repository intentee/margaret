trait Greeter {}

#[singleton(provides = Greeter)]
struct EnglishGreeter;

impl EnglishGreeter {
    #[constructor]
    fn create() -> Self {}
}

impl Greeter for EnglishGreeter {}

#[singleton]
#[console_command(name = "greet")]
struct Greet;

impl Greet {
    #[constructor]
    fn create(
        greeter: std::sync::Arc<dyn Greeter>,
        #[console_argument(positional)] name: String,
        #[console_argument(from = "salutation")] salutation: Option<String>,
        #[console_argument(from = "loud")] loud: bool,
    ) -> Self {
    }
}
