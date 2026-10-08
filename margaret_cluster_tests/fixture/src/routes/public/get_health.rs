use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/health", server = "public")]
pub struct GetHealth;

impl GetHealth {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, "ok"))
    }
}
