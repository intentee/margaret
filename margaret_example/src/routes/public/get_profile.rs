use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

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
