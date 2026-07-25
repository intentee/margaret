use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::models::user::User;

#[singleton]
#[responds_to_http(
    method = "get",
    name = "get_profile",
    path = "/profile",
    server = "public"
)]
pub struct GetProfile;

impl GetProfile {
    #[process]
    pub async fn respond(&self, #[authenticated_user] user: User) -> Response {
        Response::text(200, format!("signed in as {} ({})", user.name, user.id))
    }
}
