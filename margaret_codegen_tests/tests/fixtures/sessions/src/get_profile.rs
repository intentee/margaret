use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::reader::Reader;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Get,
    name = "get_profile",
    path = "/profile",
    server = "public"
)]
pub struct GetProfile;

impl GetProfile {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] Reader { subject }: Reader,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, subject.to_string()))
    }
}
