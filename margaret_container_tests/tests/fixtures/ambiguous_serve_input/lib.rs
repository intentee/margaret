#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(
        #[console_argument(from = "label")]
        #[environment_variable(from = "LABEL")]
        label: String,
    ) -> anyhow::Result<Self> {
    }
}
