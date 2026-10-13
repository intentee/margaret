use anyhow::Result;

use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_projects_of_kind",
    path = "/kinds/{type}/{self}/projects",
    server = "public"
)]
pub struct GetProjectsOfKind;

impl GetProjectsOfKind {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "type")] kind: String,
        #[route_parameter(from = "self")] owner: String,
    ) -> Result<Response> {
        Ok(Response::text(200, format!("{kind}/{owner}")))
    }
}
