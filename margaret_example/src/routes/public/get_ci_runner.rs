use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::ci_runner::CiRunner;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_ci_runner",
    path = "/ci/runner",
    server = "public"
)]
pub struct GetCiRunner;

impl GetCiRunner {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, #[authenticated_user] runner: CiRunner) -> anyhow::Result<Response> {
        let CiRunner {
            repository,
            repository_id,
        } = runner;

        Ok(Response::text(
            200,
            format!("GitHub Actions run of {repository} ({repository_id})"),
        ))
    }
}
