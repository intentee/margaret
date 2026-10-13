use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::patron::Patron;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/loans", server = "public")]
pub struct GetLoans;

impl GetLoans {
    /// # Errors
    ///
    /// Never returns an error; every admitted patron is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] Patron { card }: Patron,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("loans of card {card}")))
    }
}
