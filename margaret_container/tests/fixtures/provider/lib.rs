#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}
}

trait Greeter {}

#[provider(provides = Greeter)]
struct GreeterProvider {
    config: Arc<Config>,
}

impl GreeterProvider {
    #[constructor]
    fn new(config: Arc<Config>) -> Self {}

    #[provide]
    fn provide(&self) -> Arc<dyn Greeter> {}
}
