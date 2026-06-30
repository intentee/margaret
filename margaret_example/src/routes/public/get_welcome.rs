use margaret_example_macros::traced;
use margaret_http::forward::Forward;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

#[singleton]
#[traced]
#[responds_to_http(method = Get, path = "/welcome", server = "public")]
pub struct GetWelcome;

impl GetWelcome {
    #[responder]
    pub async fn respond(&self) -> Forward {
        Forward::to("get_greeting")
    }
}
