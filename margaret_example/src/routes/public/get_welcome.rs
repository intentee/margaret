use margaret::framework::http::forward::Forward;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::margaret::forwarders::public::Forwarder;

#[singleton]
#[responds_to_http(access = margaret::framework::http::public_access::PublicAccess, method = "get", path = "/welcome", server = "public")]
pub struct GetWelcome;

impl GetWelcome {
    #[process]
    pub async fn respond(&self, forward: Forwarder) -> anyhow::Result<Forward> {
        Ok(forward.get_greeting())
    }
}
