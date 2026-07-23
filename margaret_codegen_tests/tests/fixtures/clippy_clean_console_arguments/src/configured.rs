use std::path::PathBuf;

use crate::margaret::asset_bag::asset;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = "get", path = "/configured", server = "public")]
pub struct Configured;

impl Configured {
    #[constructor]
    #[must_use]
    pub fn create(
        #[console_argument(from = "label")] label: String,
        #[console_argument(from = "root")] root: PathBuf,
        #[console_argument(from = "retries")] retries: u16,
        #[console_argument(from = "verbose")] verbose: bool,
        #[console_argument(from = "optional-retries")] optional_retries: Option<u16>,
        #[console_argument(from = "note")] note: Option<String>,
    ) -> Self {
        let _ = (label, root, retries, verbose, optional_retries, note);

        Self
    }

    #[process]
    pub async fn respond(&self) -> Response {
        let _ = asset!("resources/ts/app.ts");

        Response::text(200, "configured")
    }
}
