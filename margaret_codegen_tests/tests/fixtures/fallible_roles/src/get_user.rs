use failures::Result;

use margaret::framework::http::response::Response;
use margaret::framework::macros::middleware;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use super::user::User;

#[middleware(guarded)]
#[singleton]
#[responds_to_http(method = "get", path = "/users/{user}", server = "public")]
pub struct GetUser;

impl GetUser {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, #[route_parameter(from = "user")] user: User) -> Result<Response> {
        Ok(Response::text(200, user.name))
    }
}
