use margaret::framework::http::redirect::Redirect;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = "get", path = "/greeting/see-other", server = "public")]
pub struct GetGreetingSeeOther;

impl GetGreetingSeeOther {
    #[process]
    pub fn respond(&self, routes: &Routes) -> anyhow::Result<Redirect> {
        Ok(routes.public.get_greeting.see_other())
    }
}
