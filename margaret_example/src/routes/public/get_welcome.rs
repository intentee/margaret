use margaret_http::forward::Forward;
use margaret_macros::middleware;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[middleware(logged)]
#[responds_to_http(method = Get, path = "/welcome", server = "public")]
pub struct GetWelcome;

impl GetWelcome {
    #[responder]
    pub async fn respond(&self) -> Forward {
        Forward::to("get_greeting")
    }
}
