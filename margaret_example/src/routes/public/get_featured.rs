use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::featured_article_id::FEATURED_ARTICLE_ID;
use crate::margaret::forwarders::public::Forwarder;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, forward: Forwarder) -> anyhow::Result<Forward> {
        Ok(forward.get_article(FEATURED_ARTICLE_ID.to_string()))
    }
}
