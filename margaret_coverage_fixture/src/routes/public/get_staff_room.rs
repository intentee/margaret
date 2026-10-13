use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::librarian::Librarian;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/staff-room", server = "public")]
pub struct GetStaffRoom;

impl GetStaffRoom {
    /// # Errors
    ///
    /// Never returns an error; every admitted librarian is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] Librarian { badge }: Librarian,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, format!("staff room for badge {badge}")))
    }
}
