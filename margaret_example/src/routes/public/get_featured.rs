use margaret::framework::http::redirect::Redirect;
use margaret::framework::http::route_addressing::RouteAddressing;
use margaret::framework::http::unaddressable_parameter::UnaddressableParameter;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::constants::FEATURED_ARTICLE_ID;
use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    /// # Errors
    ///
    /// Returns an error when the featured article cannot address its route.
    #[process]
    pub fn respond(&self, routes: &Routes) -> anyhow::Result<Redirect> {
        match routes.public.get_article(FEATURED_ARTICLE_ID.to_string()) {
            RouteAddressing::Addressed(route) => Ok(route.see_other()),
            RouteAddressing::Misrouted { url } => Err(anyhow::anyhow!(
                "the link of the featured article at '{url}' reaches another route"
            )),
            RouteAddressing::Unaddressable(UnaddressableParameter { name, rejection }) => {
                Err(anyhow::Error::from(rejection).context(format!(
                    "the featured article cannot fill the route parameter '{name}'"
                )))
            }
        }
    }
}
