use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(from = "tags")] tags: Vec<String>) -> Self {}
}
