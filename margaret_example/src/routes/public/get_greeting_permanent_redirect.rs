use margaret::framework::http::redirect::Redirect;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(access = margaret::framework::http::public_access::PublicAccess, method = "get", path = "/greeting/permanent", server = "public")]
pub struct GetGreetingPermanentRedirect;

impl GetGreetingPermanentRedirect {
    #[process]
    pub async fn respond(&self, routes: &Routes) -> anyhow::Result<Redirect> {
        Ok(routes.public.get_greeting.permanent_redirect())
    }
}
