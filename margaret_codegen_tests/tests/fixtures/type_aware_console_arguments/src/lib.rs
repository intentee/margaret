use std::path::PathBuf;

#[singleton]
#[responds_to_http(method = "get", path = "/configured", server = "public")]
struct Configured;

impl Configured {
    #[constructor]
    fn create(
        #[console_argument(from = "label")] label: String,
        #[console_argument(from = "root")] root: PathBuf,
        #[console_argument(from = "retries")] retries: u16,
        #[console_argument(from = "verbose")] verbose: bool,
        #[console_argument(from = "optional-retries")] optional_retries: Option<u16>,
        #[console_argument(from = "note")] note: Option<String>,
    ) -> anyhow::Result<Self> {
    }

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}
