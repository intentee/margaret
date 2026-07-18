#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}
}

#[singleton]
struct Pool;

impl Pool {
    #[constructor]
    async fn new(config: Arc<Config>) -> Self {}
}
