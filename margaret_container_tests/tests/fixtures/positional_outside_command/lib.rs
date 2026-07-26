#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(positional)] name: String) -> anyhow::Result<Self> {}
}
