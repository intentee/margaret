use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Get, path = "/health")]
pub struct GetHealth;

impl GetHealth {
    #[responder]
    pub async fn respond(&self) -> Response {
        Response::text(200, "ok")
    }
}
