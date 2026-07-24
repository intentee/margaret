#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}

    #[inline]
    fn helper(&self) {}
}

#[singleton]
struct EnglishGreeter;

impl EnglishGreeter {
    #[constructor]
    fn new(config: Arc<Config>) -> Self {}
}

struct Unprovided;

#[singleton]
struct App;

impl App {
    #[constructor]
    fn new(english_greeter: Arc<EnglishGreeter>, _: Arc<Config>) -> Self {}
}
