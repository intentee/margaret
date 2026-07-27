use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::forwarders::public::Forwarder;
use crate::stores::article_store::FEATURED_ARTICLE_ID;

#[singleton]
#[responds_to_http(method = "get", path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    #[process]
    pub async fn respond(&self, forward: Forwarder) -> anyhow::Result<Forward> {
        Ok(forward.get_article(FEATURED_ARTICLE_ID.to_string()))
    }
}
