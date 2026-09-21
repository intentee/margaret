#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "tags")] tags: Vec<String>) -> Self {}
}
