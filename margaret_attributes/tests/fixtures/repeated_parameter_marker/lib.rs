struct Config;

impl Config {
    fn create(
        #[environment_variable(from = "FIRST")]
        #[environment_variable(from = "SECOND")]
        name: String,
    ) {
    }
}
