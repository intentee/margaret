#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "path")] path: String) -> anyhow::Result<Self> {}
}

#[singleton]
struct AlphaPlugin;

impl AlphaPlugin {
    #[constructor]
    fn create(#[console_argument(from = "alpha")] alpha: String) -> anyhow::Result<Self> {}
}

#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn create(
        config: std::sync::Arc<Config>,
        alpha_plugin: std::sync::Arc<AlphaPlugin>,
    ) -> anyhow::Result<Self> {
    }
}
