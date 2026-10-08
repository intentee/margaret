use margaret::framework::http::redirect::Redirect;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/home", server = "internal")]
pub struct GetHome;

impl GetHome {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, routes: &Routes) -> anyhow::Result<Redirect> {
        Ok(routes.public.get_greeting.see_other())
    }
}
