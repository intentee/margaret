use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::visitor::Visitor;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/visitor-pass", server = "public")]
pub struct GetVisitorPass;

impl GetVisitorPass {
    /// # Errors
    ///
    /// Never returns an error; every admitted visitor is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] Visitor { name }: Visitor,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("pass for {name}")))
    }
}
