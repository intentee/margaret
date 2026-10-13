use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(#[console_argument(positional)] name: String) -> anyhow::Result<Self> {}
}
