use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::routes::Routes;
use crate::margaret::routes::forward_targets::PublicForwardTarget;
use crate::margaret::routes::props::GetGreetingRouteProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/welcome", server = "public")]
pub struct GetWelcome;

impl GetWelcome {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, routes: &Routes) -> anyhow::Result<Forward<PublicForwardTarget>> {
        Ok(routes.public.get_greeting.forward_to(GetGreetingRouteProps))
    }
}
