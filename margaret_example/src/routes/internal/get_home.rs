use margaret_http::forward::Forward;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = "get", path = "/home", server = "internal")]
pub struct GetHome;

impl GetHome {
    #[process]
    pub async fn respond(&self, routes: &Routes) -> Forward {
        routes.public.get_greeting.forward_to()
    }
}
