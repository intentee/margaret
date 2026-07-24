use std::sync::Arc;

use margaret_http::response::Response;
use margaret_macros::constructor;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::english_greeter::EnglishGreeter;

#[singleton]
#[responds_to_http(
    method = "get",
    name = "get_greeting",
    path = "/greeting",
    server = "public"
)]
pub struct GetGreeting {
    greeter: Arc<EnglishGreeter>,
}

impl GetGreeting {
    #[constructor]
    #[must_use]
    pub fn create(greeter: Arc<EnglishGreeter>) -> Self {
        Self { greeter }
    }

    #[process]
    pub async fn respond(&self) -> Response {
        Response::text(200, self.greeter.greet())
    }
}
