use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::forwarders::public::Forwarder;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/featured", server = "public")]
pub struct GetFeatured;

impl GetFeatured {
    /// # Errors
    ///
    /// Never returns an error; the featured book is always forwarded.
    #[process]
    pub fn respond(&self, forward: Forwarder) -> anyhow::Result<Forward> {
        Ok(forward.get_book("fiction".to_string(), "1".to_string(), "Dune".to_string()))
    }
}
