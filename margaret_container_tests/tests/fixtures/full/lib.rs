#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}

    #[inline]
    fn helper(&self) {}
}

trait Greeter {}

#[singleton(provides = Greeter)]
struct EnglishGreeter;

impl EnglishGreeter {
    #[constructor]
    fn new(config: Arc<Config>) -> Self {}
}

trait Plugin {}

#[singleton(collection = Plugin)]
struct LoggingPlugin;

impl LoggingPlugin {
    #[constructor]
    fn new() -> Self {}
}

#[singleton(collection = Plugin)]
struct MetricsPlugin;

impl MetricsPlugin {
    #[constructor]
    fn new(_: Arc<Config>) -> Self {}
}

trait Hook {}

#[singleton]
struct App;

impl App {
    #[constructor]
    fn new(
        greeter: Arc<dyn Greeter>,
        plugins: Vec<Arc<dyn Plugin>>,
        hooks: Vec<Arc<dyn Hook>>,
    ) -> Self {
    }
}
