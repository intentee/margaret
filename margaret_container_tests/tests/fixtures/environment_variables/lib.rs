#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create(
        #[environment_variable(from = "DATABASE_URL")] database_url: String,
        #[environment_variable(from = "WORKER_COUNT")] workers: Option<u16>,
        #[environment_variable(from = "WORKER_DEBUG")] debug: bool,
        #[environment_variable(from = "UPLOAD_ROOT")] upload_root: std::path::PathBuf,
    ) -> anyhow::Result<Self> {
    }
}
