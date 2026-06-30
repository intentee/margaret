use margaret_example_macros::traced;
use margaret_http::forward::Forward;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::http::RouteSymbol;

#[singleton]
#[traced]
#[responds_to_http(method = Get, path = "/welcome")]
pub struct GetWelcome;

impl GetWelcome {
    #[responder]
    pub async fn respond(&self) -> Forward {
        Forward::to(RouteSymbol::GetGreeting)
    }
}
