use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::ci::ci_runner::CiRunner;

#[singleton]
#[responds_to_http(method = "get", path = "/runner", server = "public")]
pub struct RunnerPage;

impl RunnerPage {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, #[authenticated_user] runner: CiRunner) -> anyhow::Result<Response> {
        Ok(Response::text(200, runner.repository))
    }
}
