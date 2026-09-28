use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::ci::upstream_runner::UpstreamRunner;

#[singleton]
#[responds_to_http(method = "get", path = "/upstream-runner", server = "public")]
pub struct UpstreamRunnerPage;

impl UpstreamRunnerPage {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] runner: UpstreamRunner,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, runner.repository))
    }
}
