use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::courier::Courier;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/deliveries", server = "public")]
pub struct GetDeliveries;

impl GetDeliveries {
    /// # Errors
    ///
    /// Never returns an error; every admitted courier is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] Courier { name }: Courier,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("deliveries for {name}")))
    }
}
