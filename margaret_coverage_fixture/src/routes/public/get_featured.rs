use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::routes::Routes;
use crate::margaret::routes::forward_targets::PublicForwardTarget;
use crate::margaret::routes::props::GetBookRouteProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    /// # Errors
    ///
    /// Never returns an error; the featured book is always forwarded.
    #[process]
    pub fn respond(&self, routes: &Routes) -> anyhow::Result<Forward<PublicForwardTarget>> {
        Ok(routes.public.get_book.forward_to(GetBookRouteProps {
            number: "1".to_string(),
            shelf: "fiction".to_string(),
            title: "Dune".to_string(),
        }))
    }
}
