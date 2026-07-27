#[singleton]
struct EnglishGreeter;

impl EnglishGreeter {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}
}

#[singleton]
#[console_command(name = "greet")]
struct Greet;

impl Greet {
    #[constructor]
    fn create(
        greeter: std::sync::Arc<EnglishGreeter>,
        #[console_argument(positional)] name: String,
        #[console_argument(from = "salutation")] salutation: Option<String>,
        #[console_argument(from = "loud")] loud: bool,
    ) -> anyhow::Result<Self> {
    }
}
