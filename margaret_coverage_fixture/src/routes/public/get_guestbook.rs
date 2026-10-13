use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::guest::Guest;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/guestbook", server = "public")]
pub struct GetGuestbook;

impl GetGuestbook {
    /// # Errors
    ///
    /// Never returns an error; every guest signs the guestbook.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] Guest { address }: Guest,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("signed by {}", address.ip())))
    }
}
