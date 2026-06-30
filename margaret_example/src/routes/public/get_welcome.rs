use margaret_example_macros::traced;
use margaret_http::forward::Forward;
use margaret_macros::responder;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::route_symbol::RouteSymbol;

#[singleton]
#[traced]
#[responds_to_http(method = Get, path = "/welcome", server = crate::servers::public::Public)]
pub struct GetWelcome;

impl GetWelcome {
    #[responder]
    pub async fn respond(&self) -> Forward {
        Forward::to(RouteSymbol::GetGreeting)
    }
}
