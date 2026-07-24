trait Plugin {}

#[singleton]
struct Mapper;

impl Mapper {
    #[constructor]
    fn create(#[console_argument(from = "mapper")] mapper: Option<String>) -> Self {}
}

#[singleton(collection = Plugin)]
struct AlphaPlugin;

impl AlphaPlugin {
    #[constructor]
    fn create(mapper: std::sync::Arc<Mapper>) -> Self {}
}

#[singleton(collection = Plugin)]
struct BetaPlugin;

impl BetaPlugin {
    #[constructor]
    fn create(mapper: std::sync::Arc<Mapper>) -> Self {}
}

#[singleton]
struct Service;

impl Service {
    #[constructor]
    fn create(plugins: Vec<std::sync::Arc<dyn Plugin>>) -> Self {}
}
