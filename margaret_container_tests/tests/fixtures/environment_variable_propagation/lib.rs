#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[environment_variable(from = "DATABASE_URL")] database_url: String) -> anyhow::Result<Self> {}
}

#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn create(config: std::sync::Arc<Config>) -> anyhow::Result<Self> {}
}
