use std::sync::Arc;

use margaret_example_macros::traced;
use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::greeter::Greeter;

#[singleton]
#[traced]
#[responds_to_http(
    method = Get,
    name = "get_greeting",
    path = "/greeting",
    server = "public"
)]
pub struct GetGreeting {
    greeter: Arc<dyn Greeter>,
}

impl GetGreeting {
    #[constructor]
    pub fn create(greeter: Arc<dyn Greeter>) -> Self {
        Self { greeter }
    }

    #[responder]
    pub async fn respond(&self) -> Response {
        Response::text(200, self.greeter.greet())
    }
}
