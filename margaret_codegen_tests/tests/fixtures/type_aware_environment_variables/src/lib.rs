use std::path::PathBuf;

use margaret::framework::route_method::route_method::RouteMethod;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/configured", server = "public")]
struct Configured;

impl Configured {
    #[constructor]
    fn create(
        #[environment_variable(from = "DATABASE_URL")] database_url: String,
        #[environment_variable(from = "UPLOAD_ROOT")] upload_root: PathBuf,
        #[environment_variable(from = "WORKER_COUNT")] workers: u16,
        #[environment_variable(from = "WORKER_DEBUG")] debug: bool,
        #[environment_variable(from = "OPTIONAL_RETRIES")] optional_retries: Option<u16>,
        #[environment_variable(from = "NOTE")] note: Option<String>,
    ) -> anyhow::Result<Self> {
    }

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
