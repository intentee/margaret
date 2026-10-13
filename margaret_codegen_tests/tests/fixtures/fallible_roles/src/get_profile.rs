use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use super::result::AppResult;
use super::user::User;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/profile", server = "public")]
pub struct GetProfile;

impl GetProfile {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, #[authenticated_user] user: User) -> AppResult<Response> {
        Ok(Response::text(200, user.name))
    }
}
