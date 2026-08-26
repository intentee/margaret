#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "name")] name: String) -> anyhow::Result<Self> {}
}

#[singleton]
#[console_command(name = "greet")]
struct Greet;

impl Greet {
    #[constructor]
    fn create(
        config: std::sync::Arc<Config>,
        #[console_argument(positional)] name: String,
    ) -> anyhow::Result<Self> {
    }
}
