use margaret_http::forward::Forward;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::margaret::routes::Routes;

#[singleton]
#[responds_to_http(method = Get, path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    #[process]
    pub async fn respond(&self, routes: &Routes) -> Forward {
        routes.public.get_article("100".to_string()).forward_to()
    }
}
