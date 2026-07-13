use margaret_http::forward::Forward;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::forwarders::public::Forwarder;

#[singleton]
#[responds_to_http(method = "get", path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    #[process]
    pub async fn respond(&self, forward: Forwarder) -> Forward {
        forward.get_article("100".to_string())
    }
}
