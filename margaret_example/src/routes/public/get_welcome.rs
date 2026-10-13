use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::margaret::forwarders::public::Forwarder;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/welcome", server = "public")]
pub struct GetWelcome;

impl GetWelcome {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(&self, forward: Forwarder) -> anyhow::Result<Forward> {
        Ok(forward.get_greeting())
    }
}
