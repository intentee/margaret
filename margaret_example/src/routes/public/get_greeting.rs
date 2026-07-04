use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::middleware;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::greeter::Greeter;

#[singleton]
#[middleware(logged)]
#[responds_to_http(
    method = "get",
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

    #[process]
    pub async fn respond(&self) -> Response {
        Response::text(200, self.greeter.greet())
    }
}
