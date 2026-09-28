use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::ci::ci_runner::CiRunner;
use crate::margaret::asset_bag::asset;

#[singleton]
#[responds_to_http(method = "get", path = "/", server = "public")]
pub struct Home;

impl Home {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] runner: Option<CiRunner>,
    ) -> anyhow::Result<Response> {
        let _asset = asset!("resources/ts/app.ts");

        Ok(match runner {
            Some(runner) => Response::text(200, runner.repository),
            None => Response::text(200, "anonymous visitor"),
        })
    }
}
