use margaret_http::forward::Forward;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::forwarders::public::Forwarder;
use crate::stores::article_store::FEATURED_ARTICLE_ID;

#[singleton]
#[responds_to_http(method = "get", path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    #[process]
    pub async fn respond(&self, forward: Forwarder) -> Forward {
        forward.get_article(FEATURED_ARTICLE_ID.to_string())
    }
}
