use margaret_http::response::Response;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Get, path = "/users/{id}")]
pub struct GetUsers;

impl GetUsers {
    #[responder]
    pub async fn respond(&self, #[route_parameter] id: String) -> Response {
        Response::text(200, id)
    }
}
