use margaret_http::forward::Forward;
use margaret_macros::middleware;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::routes::Routes;

#[singleton]
#[middleware(logged)]
#[responds_to_http(method = "get", path = "/welcome", server = "public")]
pub struct GetWelcome;

impl GetWelcome {
    #[process]
    pub async fn respond(&self, routes: &Routes) -> Forward {
        routes.public.get_greeting.forward_to()
    }
}
