use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::ci::ci_runner::CiRunner;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/", server = "public")]
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
        Ok(match runner {
            Some(runner) => Response::text(200, runner.repository),
            None => Response::text(200, "anonymous visitor"),
        })
    }
}
