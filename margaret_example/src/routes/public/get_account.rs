use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::user::User;

#[singleton]
#[responds_to_http(method = Get, path = "/account", server = crate::servers::public::Public)]
pub struct GetAccount;

impl GetAccount {
    #[responder]
    pub async fn respond(&self, #[session_authenticated] user: User) -> Response {
        Response::text(200, format!("account: {} #{}", user.name, user.id))
    }
}
