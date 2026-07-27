use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::identity_client::IdentityClient;

#[singleton]
#[responds_to_http(method = "get", path = "/call", server = "public")]
pub struct CallRoute {
    identity_client: Arc<IdentityClient>,
}

impl CallRoute {
    #[constructor]
    pub fn create(identity_client: Arc<IdentityClient>) -> anyhow::Result<Self> {
        Ok(Self { identity_client })
    }

    #[process]
    pub async fn respond(&self) -> anyhow::Result<Response> {
        Ok({
            let _http_client = self.identity_client.http_client();

            Response::text(200, "called")
        })
    }
}
