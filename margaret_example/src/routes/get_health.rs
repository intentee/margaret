use async_trait::async_trait;
use margaret_http::http_responder::HttpResponder;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[responds_to_http(method = Get, path = "/health")]
pub struct GetHealth;

impl GetHealth {
    #[constructor]
    pub fn create() -> Self {
        Self
    }
}

#[async_trait]
impl HttpResponder for GetHealth {
    async fn respond(&self, _request: Request) -> Response {
        Response::text(200, "ok")
    }
}
