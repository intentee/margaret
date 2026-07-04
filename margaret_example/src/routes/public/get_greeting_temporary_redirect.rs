use margaret_http::redirect::Redirect;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = "get", path = "/greeting/temporary", server = "public")]
pub struct GetGreetingTemporaryRedirect;

impl GetGreetingTemporaryRedirect {
    #[process]
    pub async fn respond(&self, routes: &Routes) -> Redirect {
        routes.public.get_greeting.temporary_redirect()
    }
}
