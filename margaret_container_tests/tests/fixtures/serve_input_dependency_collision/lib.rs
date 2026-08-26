#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "shared")] shared: String) -> anyhow::Result<Self> {}
}

#[singleton]
#[console_command(name = "sub")]
struct Sub;

impl Sub {
    #[constructor]
    fn create(
        config: std::sync::Arc<Config>,
        #[console_argument(positional)] shared: String,
    ) -> anyhow::Result<Self> {
    }
}

#[singleton]
struct Parent;

impl Parent {
    #[constructor]
    fn create(sub: std::sync::Arc<Sub>) -> anyhow::Result<Self> {}
}
