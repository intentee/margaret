use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::english_greeter::EnglishGreeter;

#[singleton]
#[responds_to_http(access = margaret::framework::http::public_access::PublicAccess,
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
    pub fn create(greeter: Arc<EnglishGreeter>) -> anyhow::Result<Self> {
        Ok(Self { greeter })
    }

    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, self.greeter.greet()))
    }
}
