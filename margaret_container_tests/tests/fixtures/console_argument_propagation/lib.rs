trait Plugin {}

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "path")] path: String) -> Self {}
}

#[singleton(collection = Plugin, provides = Plugin)]
struct AlphaPlugin;

impl AlphaPlugin {
    #[constructor]
    fn create(#[console_argument(from = "alpha")] alpha: String) -> Self {}
}

impl Plugin for AlphaPlugin {}

#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn create(
        config: std::sync::Arc<Config>,
        plugins: Vec<std::sync::Arc<dyn Plugin>>,
    ) -> Self {
    }
}
