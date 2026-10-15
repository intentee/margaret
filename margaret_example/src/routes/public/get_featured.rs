use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::url_path::path_parameter_value::PathParameterValue;

use crate::constants::FEATURED_ARTICLE_ID;
use crate::margaret::routes::Routes;
use crate::margaret::routes::forward_targets::PublicForwardTarget;
use crate::margaret::routes::props::GetArticleRouteProps;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, routes: &Routes) -> anyhow::Result<Forward<PublicForwardTarget>> {
        PathParameterValue::new(FEATURED_ARTICLE_ID.to_string())
            .map(|article| {
                routes
                    .public
                    .get_article
                    .forward_to(GetArticleRouteProps { article })
            })
            .map_err(anyhow::Error::from)
    }
}
