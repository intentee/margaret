use std::sync::Arc;

use async_trait::async_trait;
use margaret_http::http_responder::HttpResponder;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::greeter::Greeter;

#[singleton]
#[responds_to_http(method = Get, path = "/greeting")]
pub struct GetGreeting {
    greeter: Arc<dyn Greeter + Send + Sync>,
}

impl GetGreeting {
    #[constructor]
    pub fn create(greeter: Arc<dyn Greeter + Send + Sync>) -> Self {
        Self { greeter }
    }
}

#[async_trait]
impl HttpResponder for GetGreeting {
    async fn respond(&self, _request: Request) -> Response {
        Response::text(200, self.greeter.greet())
    }
}
