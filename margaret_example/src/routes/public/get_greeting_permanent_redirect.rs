use margaret_http::redirect::Redirect;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = "get", path = "/greeting/permanent", server = "public")]
pub struct GetGreetingPermanentRedirect;

impl GetGreetingPermanentRedirect {
    #[process]
    pub async fn respond(&self, routes: &Routes) -> Redirect {
        routes.public.get_greeting.permanent_redirect()
    }
}
