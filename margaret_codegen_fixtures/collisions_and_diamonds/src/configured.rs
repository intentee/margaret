use std::path::PathBuf;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::asset_bag::asset;

#[singleton]
#[responds_to_http(method = "get", path = "/configured", server = "public")]
pub struct Configured {
    _label: String,
    _tenant: String,
    _note: Option<String>,
    _optional_retries: Option<u16>,
    _retries: u16,
    _root: PathBuf,
    _verbose: bool,
}

impl Configured {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(
        #[console_argument(from = "label")] label: String,
        #[console_argument(from = "root")] root: PathBuf,
        #[console_argument(from = "retries")] retries: u16,
        #[console_argument(from = "verbose")] verbose: bool,
        #[console_argument(from = "optional-retries")] optional_retries: Option<u16>,
        #[console_argument(from = "note")] note: Option<String>,
        #[console_argument(from = "tenant")] tenant: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            _label: label,
            _note: note,
            _optional_retries: optional_retries,
            _retries: retries,
            _tenant: tenant,
            _root: root,
            _verbose: verbose,
        })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        tokio::task::yield_now().await;

        Ok({
            let _ = asset!("resources/ts/app.ts");

            Response::text(200, "configured")
        })
    }
}
