use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::user::User;

#[singleton]
#[responds_to_http(method = Get, path = "/profiles/{user}")]
pub struct GetProfile;

impl GetProfile {
    #[responder]
    pub async fn respond(
        &self,
        #[route_parameter(intent = CrudAction::Read)] user: User,
    ) -> Response {
        Response::text(200, format!("{} #{}", user.name, user.id))
    }
}
