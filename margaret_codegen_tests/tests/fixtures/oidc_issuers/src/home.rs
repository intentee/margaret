use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::asset_bag::asset;

#[singleton]
#[responds_to_http(method = "get", path = "/", server = "public")]
pub struct Home;

impl Home {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        let _asset = asset!("resources/ts/app.ts");

        Ok(Response::text(200, "home"))
    }
}
