#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}
}

#[service]
struct Pulse {
    config: Arc<Config>,
}

impl Pulse {
    #[constructor]
    fn new(config: Arc<Config>) -> Self {}
}

#[ticker(interval = crate::schedule::PERIOD)]
struct Sweeper;
