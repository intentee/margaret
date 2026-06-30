use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::views::login::LOGIN_FORM;

#[singleton]
#[responds_to_http(method = Get, path = "/login")]
pub struct GetLogin;

impl GetLogin {
    #[responder]
    pub async fn respond(&self) -> Response {
        Response::html(200, LOGIN_FORM)
    }
}
