use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(
        #[console_argument(from = "DATABASE_URL")] flag: String,
        #[environment_variable(from = "DATABASE_URL")] variable: String,
    ) -> anyhow::Result<Self> {
    }
}
